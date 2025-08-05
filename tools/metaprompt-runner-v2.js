#!/usr/bin/env node

const fs = require('fs').promises;
const path = require('path');
const yaml = require('js-yaml');
const chalk = require('chalk');
const Handlebars = require('handlebars');
const { encoding_for_model } = require('tiktoken');
const pRetry = require('p-retry');
const pLimit = require('p-limit');
const matter = require('gray-matter');
const { LRUCache } = require('lru-cache');
const crypto = require('crypto');
const OpenAI = require('openai');

/**
 * Enhanced Metaprompt Runner v2
 * - GitHub prompt template support
 * - Token counting and management
 * - Caching with LRU
 * - Retry logic with exponential backoff
 * - Parallel execution with rate limiting
 * - Progress tracking
 * - IDEMPOTENT generation
 */

class MetapromptRunnerV2 {
  constructor(options = {}) {
    this.configPath = options.configPath || 'game-config.yaml';
    this.promptsDir = options.promptsDir || '.github/prompts';
    this.openaiKey = options.openaiKey || process.env.OPENAI_API_KEY;
    this.isDryRun = options.dryRun || false;
    this.isGitHubAction = process.env.GITHUB_ACTIONS === 'true';
    this.outputDir = options.outputDir || '.';
    this.maxRetries = options.maxRetries || 3;
    this.concurrency = options.concurrency || 3;
    this.cacheDir = options.cacheDir || '.cache/metaprompt';
    
    if (!this.openaiKey && !this.isDryRun) {
      throw new Error('OPENAI_API_KEY is required');
    }
    
    // Initialize OpenAI client
    this.openai = null;
    if (this.openaiKey) {
      try {
        this.openai = new OpenAI({
          apiKey: this.openaiKey,
        });
      } catch (error) {
        console.error('Error initializing OpenAI:', error);
        console.error('OpenAI type:', typeof OpenAI);
        console.error('OpenAI:', OpenAI);
        throw error;
      }
    }
    
    // Initialize cache
    this.cache = new LRUCache({
      max: 500,
      ttl: 1000 * 60 * 60 * 24, // 24 hours
    });
    
    // Rate limiter
    this.limit = pLimit(this.concurrency);
    
    // Token encoder
    this.encoder = null;
    
    // Prompt templates
    this.templates = new Map();
    
    // Progress tracking
    this.progress = {
      total: 0,
      completed: 0,
      failed: 0,
      cached: 0,
      skipped: 0,
    };
    
    // Track generated files for idempotency
    this.generatedFiles = new Set();
  }
  
  log(message, level = 'info') {
    const prefix = this.isGitHubAction ? '::' : '';
    const timestamp = new Date().toISOString();
    
    switch (level) {
      case 'error':
        console.error(`${prefix}error::${chalk.red(message)}`);
        break;
      case 'warning':
        console.warn(`${prefix}warning::${chalk.yellow(message)}`);
        break;
      case 'success':
        console.log(`${prefix}${chalk.green('✓')} ${chalk.green(message)}`);
        break;
      case 'skip':
        console.log(`${prefix}${chalk.blue('↷')} ${chalk.blue(message)}`);
        break;
      case 'group':
        if (this.isGitHubAction) {
          console.log(`::group::${message}`);
        } else {
          console.log(chalk.bold.blue(`\n╔═══ ${message} ═══╗`));
        }
        break;
      case 'endgroup':
        if (this.isGitHubAction) {
          console.log('::endgroup::');
        } else {
          console.log(chalk.bold.blue('╚' + '═'.repeat(50) + '╝\n'));
        }
        break;
      case 'progress':
        const percent = Math.round((this.progress.completed + this.progress.skipped) / this.progress.total * 100);
        const bar = '█'.repeat(percent / 2) + '░'.repeat(50 - percent / 2);
        console.log(`\r[${bar}] ${percent}% (${this.progress.completed} completed, ${this.progress.skipped} skipped)`);
        break;
      default:
        console.log(`[${timestamp}] ${message}`);
    }
  }
  
  async initialize() {
    // Load configuration
    await this.loadConfig();
    
    // Initialize token encoder
    this.encoder = encoding_for_model('gpt-4');
    
    // Load prompt templates
    await this.loadPromptTemplates();
    
    // Initialize cache directory
    await fs.mkdir(this.cacheDir, { recursive: true });
    
    // Load cache from disk
    await this.loadCacheFromDisk();
    
    // Scan existing generated files
    await this.scanExistingFiles();
    
    this.log('✅ Initialization complete', 'success');
  }
  
  async scanExistingFiles() {
    this.log('Scanning existing generated files...', 'info');
    
    // Track existing Rust files
    try {
      const srcFiles = await this.walkDir('src');
      srcFiles.forEach(file => {
        if (file.endsWith('.rs')) {
          this.generatedFiles.add(file);
        }
      });
    } catch (error) {
      // src directory doesn't exist yet
    }
    
    // Track existing asset files
    try {
      const assetFiles = await this.walkDir('assets');
      assetFiles.forEach(file => {
        if (file.endsWith('.yaml') || file.endsWith('.json')) {
          this.generatedFiles.add(file);
        }
      });
    } catch (error) {
      // assets directory doesn't exist yet
    }
    
    this.log(`Found ${this.generatedFiles.size} existing generated files`, 'info');
  }
  
  async walkDir(dir) {
    const files = [];
    try {
      const entries = await fs.readdir(dir, { withFileTypes: true });
      for (const entry of entries) {
        const fullPath = path.join(dir, entry.name);
        if (entry.isDirectory() && !entry.name.startsWith('.')) {
          files.push(...await this.walkDir(fullPath));
        } else if (entry.isFile()) {
          files.push(fullPath);
        }
      }
    } catch (error) {
      // Directory doesn't exist
    }
    return files;
  }
  
  async loadConfig() {
    const content = await fs.readFile(this.configPath, 'utf-8');
    this.config = yaml.load(content);
    this.log('Loaded game configuration', 'success');
    return this.config;
  }
  
  async loadPromptTemplates() {
    this.log('Loading GitHub prompt templates...', 'info');
    
    const loadTemplatesRecursive = async (dir) => {
      const entries = await fs.readdir(dir, { withFileTypes: true });
      
      for (const entry of entries) {
        const fullPath = path.join(dir, entry.name);
        
        if (entry.isDirectory()) {
          await loadTemplatesRecursive(fullPath);
        } else if (entry.name.endsWith('.md')) {
          const content = await fs.readFile(fullPath, 'utf-8');
          const { data: frontmatter, content: promptContent } = matter(content);
          
          const templateName = path.relative(this.promptsDir, fullPath)
            .replace(/\.md$/, '')
            .replace(/\\/g, '/');
          
          // Parse system and user prompts
          const systemMatch = promptContent.match(/<system>([\s\S]*?)<\/system>/);
          const userMatch = promptContent.match(/<user>([\s\S]*?)<\/user>/);
          
          this.templates.set(templateName, {
            name: templateName,
            model: frontmatter.model || 'gpt-4',
            temperature: frontmatter.temperature || 0.7,
            maxTokens: frontmatter.max_tokens || 4000,
            systemPrompt: systemMatch ? systemMatch[1].trim() : '',
            userPrompt: userMatch ? userMatch[1].trim() : '',
            metadata: frontmatter,
          });
          
          this.log(`  Loaded template: ${templateName}`, 'info');
        }
      }
    };
    
    try {
      await loadTemplatesRecursive(this.promptsDir);
      this.log(`Loaded ${this.templates.size} prompt templates`, 'success');
    } catch (error) {
      this.log(`Warning: Could not load prompt templates: ${error.message}`, 'warning');
    }
  }
  
  async loadCacheFromDisk() {
    try {
      const cacheFile = path.join(this.cacheDir, 'cache.json');
      const cacheData = await fs.readFile(cacheFile, 'utf-8');
      const entries = JSON.parse(cacheData);
      
      for (const [key, value] of entries) {
        this.cache.set(key, value);
      }
      
      this.log(`Loaded ${entries.length} cached entries`, 'info');
    } catch (error) {
      // Cache file doesn't exist yet, that's ok
    }
  }
  
  async saveCacheToDisk() {
    const cacheFile = path.join(this.cacheDir, 'cache.json');
    const entries = Array.from(this.cache.entries());
    await fs.writeFile(cacheFile, JSON.stringify(entries, null, 2));
  }
  
  getCacheKey(template, params) {
    const data = JSON.stringify({ template, params });
    return crypto.createHash('sha256').update(data).digest('hex');
  }
  
  countTokens(text) {
    return this.encoder.encode(text).length;
  }
  
  compileTemplate(template, data) {
    const compiled = Handlebars.compile(template);
    return compiled(data);
  }
  
  async fileNeedsUpdate(filePath, newContent) {
    const fullPath = path.join(this.outputDir, filePath);
    
    try {
      const existingContent = await fs.readFile(fullPath, 'utf-8');
      
      // For Rust files, check if the module structure is intact
      if (filePath.endsWith('.rs')) {
        // Extract module declarations
        const existingMods = this.extractModuleDeclarations(existingContent);
        const newMods = this.extractModuleDeclarations(newContent);
        
        // If module structure changed significantly, update is needed
        if (existingMods.length !== newMods.length) {
          return true;
        }
        
        // Check for significant changes (not just whitespace or comments)
        const existingCode = this.normalizeCode(existingContent);
        const newCode = this.normalizeCode(newContent);
        
        return existingCode !== newCode;
      }
      
      // For YAML/JSON files, parse and compare
      if (filePath.endsWith('.yaml') || filePath.endsWith('.yml')) {
        try {
          const existingData = yaml.load(existingContent);
          const newData = typeof newContent === 'string' ? yaml.load(newContent) : newContent;
          return JSON.stringify(existingData) !== JSON.stringify(newData);
        } catch {
          return true; // If parsing fails, update the file
        }
      }
      
      // For other files, simple comparison
      return existingContent.trim() !== newContent.trim();
      
    } catch (error) {
      // File doesn't exist, needs to be created
      return true;
    }
  }
  
  extractModuleDeclarations(rustCode) {
    const modRegex = /(?:pub\s+)?mod\s+(\w+)\s*[{;]/g;
    const mods = [];
    let match;
    while ((match = modRegex.exec(rustCode)) !== null) {
      mods.push(match[1]);
    }
    return mods;
  }
  
  normalizeCode(code) {
    // Remove comments and normalize whitespace for comparison
    return code
      .replace(/\/\/.*$/gm, '') // Remove line comments
      .replace(/\/\*[\s\S]*?\*\//g, '') // Remove block comments
      .replace(/\s+/g, ' ') // Normalize whitespace
      .trim();
  }
  
  async generateWithTemplate(templateName, params = {}, options = {}) {
    const template = this.templates.get(templateName);
    if (!template) {
      throw new Error(`Template not found: ${templateName}`);
    }
    
    // Merge config data with params
    const templateData = {
      ...this.config.game,
      ...this.config.build,
      ...this.config.environments,
      ...params,
    };
    
    // Compile templates
    const systemPrompt = this.compileTemplate(template.systemPrompt, templateData);
    const userPrompt = this.compileTemplate(template.userPrompt, templateData);
    
    // Generate with retry logic
    return await this.generateWithAI(systemPrompt, userPrompt, {
      ...template.metadata,
      ...options,
      templateName,
    });
  }
  
  async generateWithAI(systemPrompt, userPrompt, options = {}) {
    const {
      temperature = 0.7,
      maxTokens = 4000,
      model = 'gpt-4',
      outputPath,
      parseFormat,
      templateName,
      skipIfExists = false,
    } = options;
    
    // Check if file already exists and skipIfExists is true
    if (skipIfExists && outputPath) {
      const fullPath = path.join(this.outputDir, outputPath);
      if (this.generatedFiles.has(outputPath)) {
        this.log(`Skipping existing file: ${outputPath}`, 'skip');
        this.progress.skipped++;
        this.log('progress');
        return null;
      }
    }
    
    // Check cache first
    const cacheKey = this.getCacheKey({ systemPrompt, userPrompt, model, temperature }, {});
    const cached = this.cache.get(cacheKey);
    if (cached && !this.isDryRun) {
      this.log(`Using cached result for: ${outputPath || templateName}`, 'info');
      this.progress.cached++;
      
      // Check if file needs update even with cached content
      if (outputPath) {
        const needsUpdate = await this.fileNeedsUpdate(outputPath, cached);
        if (!needsUpdate) {
          this.log(`File unchanged: ${outputPath}`, 'skip');
          this.progress.skipped++;
          this.log('progress');
          return cached;
        }
        
        await this.saveToFile(outputPath, cached);
      }
      return cached;
    }
    
    this.log(`Generating: ${outputPath || templateName || 'content'}`, 'info');
    
    if (this.isDryRun) {
      this.log('DRY RUN: Would generate with:', 'warning');
      console.log('System tokens:', this.countTokens(systemPrompt));
      console.log('User tokens:', this.countTokens(userPrompt));
      console.log('Model:', model, 'Temp:', temperature, 'Max:', maxTokens);
      return 'DRY RUN OUTPUT';
    }
    
    try {
      // Retry logic with exponential backoff
      const response = await pRetry(
        async () => {
          // Token counting and validation
          const totalTokens = this.countTokens(systemPrompt) + this.countTokens(userPrompt);
          if (totalTokens > 8000) {
            this.log(`Warning: High token count (${totalTokens}), may need to split`, 'warning');
          }
          
          const completion = await this.openai.chat.completions.create({
            model,
            messages: [
              { role: 'system', content: systemPrompt },
              { role: 'user', content: userPrompt }
            ],
            temperature,
            max_tokens: maxTokens,
          });
          
          return completion.choices[0]?.message?.content || '';
        },
        {
          retries: this.maxRetries,
          onFailedAttempt: error => {
            this.log(`Attempt ${error.attemptNumber} failed: ${error.message}`, 'warning');
          },
        }
      );
      
      let content = response;
      
      // Parse if format specified
      if (parseFormat) {
        content = this.parseContent(content, parseFormat);
      }
      
      // Cache the result
      this.cache.set(cacheKey, content);
      
      // Save to file if path specified
      if (outputPath) {
        const needsUpdate = await this.fileNeedsUpdate(outputPath, content);
        if (!needsUpdate) {
          this.log(`File unchanged: ${outputPath}`, 'skip');
          this.progress.skipped++;
        } else {
          await this.saveToFile(outputPath, content);
          this.progress.completed++;
        }
      } else {
        this.progress.completed++;
      }
      
      this.log('progress');
      
      return content;
    } catch (error) {
      this.progress.failed++;
      this.log(`AI generation failed: ${error.message}`, 'error');
      throw error;
    }
  }
  
  parseContent(content, format) {
    switch (format) {
      case 'rust':
        // Extract from code blocks
        const rustMatch = content.match(/```rust\n([\s\S]*?)\n```/);
        return rustMatch ? rustMatch[1] : content;
        
      case 'yaml':
        try {
          // Try to extract from code blocks first
          const yamlMatch = content.match(/```ya?ml\n([\s\S]*?)\n```/);
          if (yamlMatch) {
            return yaml.load(yamlMatch[1]);
          }
          return yaml.load(content);
        } catch (error) {
          this.log(`YAML parse error: ${error.message}`, 'warning');
          // Clean up common YAML issues
          let cleaned = content
            .replace(/```ya?ml\n?/g, '')
            .replace(/```\n?/g, '')
            .replace(/^\s*-\s+/gm, '- '); // Fix list indentation
          
          try {
            return yaml.load(cleaned);
          } catch (secondError) {
            // Return as string if still fails
            return content;
          }
        }
        
      case 'json':
        try {
          const jsonMatch = content.match(/```json\n([\s\S]*?)\n```/);
          if (jsonMatch) {
            return JSON.parse(jsonMatch[1]);
          }
          return JSON.parse(content);
        } catch (error) {
          this.log(`JSON parse error: ${error.message}`, 'warning');
          return content;
        }
        
      default:
        return content;
    }
  }
  
  async saveToFile(filePath, content) {
    const fullPath = path.join(this.outputDir, filePath);
    const dir = path.dirname(fullPath);
    await fs.mkdir(dir, { recursive: true });
    
    let fileContent = typeof content === 'string' 
      ? content 
      : yaml.dump(content, { noRefs: true, lineWidth: -1 });
    
    // For Rust files, ensure proper formatting
    if (filePath.endsWith('.rs')) {
      // Add file header comment
      const header = `// Generated by metaprompt-runner-v2
// Source: game-config.yaml
// Template: ${filePath.replace(/\.rs$/, '')}

`;
      if (!fileContent.startsWith('//')) {
        fileContent = header + fileContent;
      }
    }
    
    await fs.writeFile(fullPath, fileContent);
    this.generatedFiles.add(filePath);
    this.log(`📝 Saved: ${filePath}`, 'success');
  }
  
  async generateModuleIndex(modulePath, submodules) {
    // Generate a mod.rs file that re-exports all submodules
    const modContent = submodules.map(mod => {
      const modName = path.basename(mod, '.rs');
      return `pub mod ${modName};`;
    }).join('\n');
    
    const modPath = path.join(modulePath, 'mod.rs');
    await this.saveToFile(modPath, modContent);
  }
  
  async executePhase1_CoreFiles() {
    this.log('Phase 1: Core Rust Files', 'group');
    
    const tasks = [
      // Cargo.toml
      this.limit(() => this.generateWithTemplate('game-generation/cargo-toml', {
        game_title: this.config.game.title,
        game_codename: this.config.game.codename,
        game_version: this.config.game.version,
        dependencies: this.config.build.dependencies,
        rust_version: this.config.build.rust_version,
      }, {
        outputPath: 'Cargo.toml',
        parseFormat: 'rust',
      })),
      
      // main.rs
      this.limit(() => this.generateWithTemplate('game-generation/main-rs', {
        tile_size: this.config.graphics.tile_size,
        perspective: this.config.graphics.perspective,
        game_states: ['Menu', 'Overworld', 'Battle', 'Shop', 'Dungeon', 'Inventory'],
      }, {
        outputPath: 'src/main.rs',
        parseFormat: 'rust',
      })),
      
      // lib.rs for module organization
      this.limit(async () => {
        const libContent = `// ${this.config.game.title} - Game Library
// This file organizes all game modules

pub mod components;
pub mod tilemaps;
pub mod systems;

pub use components::*;
`;
        await this.saveToFile('src/lib.rs', libContent);
      }),
    ];
    
    await Promise.all(tasks);
    this.log('endgroup');
  }
  
  async executePhase2_Tilemaps() {
    this.log('Phase 2: Tilemap Generation', 'group');
    
    const zones = this.config.environments.outdoor_zones;
    const tasks = [];
    const generatedModules = [];
    
    for (const zone of zones) {
      const moduleName = zone.name.toLowerCase().replace(/\s+/g, '_');
      const modulePath = `src/tilemaps/${moduleName}.rs`;
      generatedModules.push(modulePath);
      
      // Generate tilemap module
      tasks.push(
        this.limit(() => this.generateWithTemplate('game-generation/tilemap-module', {
          zone_name: zone.name,
          zone_type: zone.type,
          tiles: zone.tiles,
          chunk_size: this.config.environments.map_generation.chunk_size,
          layers: zone.tilemap_layers,
          palette_count: 3,
        }, {
          outputPath: modulePath,
          parseFormat: 'rust',
        }))
      );
    }
    
    await Promise.all(tasks);
    
    // Generate mod.rs for tilemaps
    await this.generateModuleIndex('src/tilemaps', generatedModules);
    
    this.log('endgroup');
  }
  
  async executePhase3_Monsters() {
    this.log('Phase 3: Monster Generation', 'group');
    
    await this.generateWithTemplate('game-generation/monster-generator', {
      monster_count: 25,
      nouns: this.config.generation_rules.nouns,
      verbs: this.config.generation_rules.verbs,
      adjectives: this.config.generation_rules.adjectives,
      monster_types: ['Beast', 'Spirit', 'Elemental', 'Construct', 'Aberration'],
    }, {
      outputPath: 'assets/data/monsters.yaml',
      parseFormat: 'yaml',
    });
    
    this.log('endgroup');
  }
  
  async executeFullCascade() {
    this.log(chalk.bold.magenta(`
╔═══════════════════════════════════════════════╗
║   Metaprompt Cascade v2: ${this.config.game.title.padEnd(20)} ║
║   Enhanced with GitHub Prompts & Idempotency  ║
╚═══════════════════════════════════════════════╝
    `));
    
    const startTime = Date.now();
    
    // Calculate total tasks for progress tracking
    this.progress.total = 
      3 + // Core files (Cargo.toml, main.rs, lib.rs)
      this.config.environments.outdoor_zones.length + 1 + // Tilemaps + mod.rs
      1; // Monsters
    
    try {
      await this.executePhase1_CoreFiles();
      await this.executePhase2_Tilemaps();
      await this.executePhase3_Monsters();
      
      // Save cache to disk
      await this.saveCacheToDisk();
      
      const duration = ((Date.now() - startTime) / 1000).toFixed(1);
      
      this.log(`\n📊 Generation Statistics:`, 'info');
      this.log(`  Total tasks: ${this.progress.total}`, 'info');
      this.log(`  Completed: ${this.progress.completed}`, 'success');
      this.log(`  Skipped (unchanged): ${this.progress.skipped}`, 'info');
      this.log(`  Failed: ${this.progress.failed}`, this.progress.failed > 0 ? 'error' : 'info');
      this.log(`  From cache: ${this.progress.cached}`, 'info');
      this.log(`  Duration: ${duration}s`, 'info');
      
      this.log(chalk.bold.green(`\n✅ Metaprompt cascade complete!`), 'success');
      
      // Generate summary
      await this.generateSummary();
      
    } catch (error) {
      this.log(`Cascade failed: ${error.message}`, 'error');
      throw error;
    }
  }
  
  async generateSummary() {
    const summary = {
      generated_at: new Date().toISOString(),
      game: this.config.game,
      statistics: {
        total_tasks: this.progress.total,
        completed: this.progress.completed,
        skipped: this.progress.skipped,
        failed: this.progress.failed,
        from_cache: this.progress.cached,
      },
      files_generated: Array.from(this.generatedFiles).sort(),
      next_steps: [
        'Run `cargo build` to compile the game',
        'Run `cargo run` to play the game',
        'Run `./build-web.sh` for WebAssembly build',
        'Run `npm run validate` to check generated assets',
      ]
    };
    
    const summaryContent = `# Generation Summary

Generated: ${summary.generated_at}

## Game Info
- **Title**: ${summary.game.title}
- **Version**: ${summary.game.version}
- **Codename**: ${summary.game.codename}

## Generation Statistics
- **Total tasks**: ${summary.statistics.total_tasks}
- **Completed**: ${summary.statistics.completed}
- **Skipped (unchanged)**: ${summary.statistics.skipped}
- **Failed**: ${summary.statistics.failed}
- **From cache**: ${summary.statistics.from_cache}

## Files Generated (${summary.files_generated.length})
${summary.files_generated.map(f => `- \`${f}\``).join('\n')}

## Next Steps
${summary.next_steps.map((s, i) => `${i + 1}. ${s}`).join('\n')}

## Notes
- Files are only regenerated if their content has changed
- Cached results are used when available (24hr TTL)
- Run with \`--dry-run\` to preview changes without generating
`;
    
    await this.saveToFile('GENERATION_SUMMARY.md', summaryContent);
  }
}

// CLI Interface
async function main() {
  const args = process.argv.slice(2);
  const options = {
    dryRun: args.includes('--dry-run'),
    configPath: args.find(a => a.startsWith('--config='))?.split('=')[1],
    outputDir: args.find(a => a.startsWith('--output='))?.split('=')[1],
    promptsDir: args.find(a => a.startsWith('--prompts='))?.split('=')[1],
    concurrency: parseInt(args.find(a => a.startsWith('--concurrency='))?.split('=')[1] || '3'),
  };
  
  if (args.includes('--help')) {
    console.log(`
${chalk.bold('Metaprompt Runner v2')} - Enhanced game generation with GitHub prompts

${chalk.bold('Usage:')} node metaprompt-runner-v2.js [options]

${chalk.bold('Options:')}
  --dry-run              Show what would be generated without calling OpenAI
  --config=PATH          Use custom config file (default: game-config.yaml)
  --output=DIR           Output directory (default: current directory)
  --prompts=DIR          GitHub prompts directory (default: .github/prompts)
  --concurrency=N        Max parallel API calls (default: 3)
  --help                 Show this help

${chalk.bold('Environment:')}
  OPENAI_API_KEY         Required for AI generation

${chalk.bold('Features:')}
  ✓ GitHub prompt template support
  ✓ Token counting and management
  ✓ Response caching (24hr TTL)
  ✓ Retry with exponential backoff
  ✓ Parallel execution with rate limiting
  ✓ Progress tracking
  ✓ Idempotent generation (only updates changed files)

${chalk.bold('Examples:')}
  node metaprompt-runner-v2.js
  node metaprompt-runner-v2.js --dry-run
  node metaprompt-runner-v2.js --output=generated/ --concurrency=5
    `);
    process.exit(0);
  }
  
  try {
    const runner = new MetapromptRunnerV2(options);
    await runner.initialize();
    await runner.executeFullCascade();
  } catch (error) {
    console.error(chalk.red(`\n❌ Error: ${error.message}`));
    process.exit(1);
  }
}

// Export for use as module
module.exports = { MetapromptRunnerV2 };

// Run if called directly
if (require.main === module) {
  main().catch(console.error);
}