#!/usr/bin/env node

const fs = require('fs').promises;
const path = require('path');
const yaml = require('js-yaml');
const { Configuration, OpenAIApi } = require('openai');
const chalk = require('chalk');
const { spawn } = require('child_process');

/**
 * Metaprompt Runner - Executes the complete game generation cascade
 * Can be run by GitHub Actions or directly by the AI assistant
 */

class MetapromptRunner {
  constructor(options = {}) {
    this.configPath = options.configPath || 'game-config.yaml';
    this.openaiKey = options.openaiKey || process.env.OPENAI_API_KEY;
    this.isDryRun = options.dryRun || false;
    this.isGitHubAction = process.env.GITHUB_ACTIONS === 'true';
    this.outputDir = options.outputDir || '.';
    
    if (!this.openaiKey) {
      throw new Error('OPENAI_API_KEY is required');
    }
    
    this.openai = new OpenAIApi(new Configuration({
      apiKey: this.openaiKey,
    }));
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
      case 'group':
        if (this.isGitHubAction) {
          console.log(`::group::${message}`);
        } else {
          console.log(chalk.bold.blue(`\n=== ${message} ===`));
        }
        break;
      case 'endgroup':
        if (this.isGitHubAction) {
          console.log('::endgroup::');
        }
        break;
      default:
        console.log(`[${timestamp}] ${chalk.green(message)}`);
    }
  }
  
  async loadConfig() {
    const content = await fs.readFile(this.configPath, 'utf-8');
    this.config = yaml.load(content);
    this.log('✅ Loaded game configuration');
    return this.config;
  }
  
  async generateWithAI(systemPrompt, userPrompt, options = {}) {
    const {
      temperature = 0.7,
      maxTokens = 4000,
      model = 'gpt-4',
      outputPath,
      parseFormat
    } = options;
    
    this.log(`Generating: ${outputPath || 'content'}`, 'info');
    
    if (this.isDryRun) {
      this.log('DRY RUN: Would generate with prompts:', 'warning');
      console.log('System:', systemPrompt.substring(0, 100) + '...');
      console.log('User:', userPrompt.substring(0, 100) + '...');
      return 'DRY RUN OUTPUT';
    }
    
    try {
      const response = await this.openai.createChatCompletion({
        model,
        messages: [
          { role: 'system', content: systemPrompt },
          { role: 'user', content: userPrompt }
        ],
        temperature,
        max_tokens: maxTokens,
      });
      
      let content = response.data.choices[0]?.message?.content || '';
      
      // Parse if format specified
      if (parseFormat) {
        content = this.parseContent(content, parseFormat);
      }
      
      // Save to file if path specified
      if (outputPath) {
        await this.saveToFile(outputPath, content);
      }
      
      return content;
    } catch (error) {
      this.log(`AI generation failed: ${error.message}`, 'error');
      throw error;
    }
  }
  
  parseContent(content, format) {
    switch (format) {
      case 'rust':
        const rustMatch = content.match(/```rust\n([\s\S]*?)\n```/);
        return rustMatch ? rustMatch[1] : content;
      case 'yaml':
        return yaml.load(content);
      case 'json':
        return JSON.parse(content);
      default:
        return content;
    }
  }
  
  async saveToFile(filePath, content) {
    const fullPath = path.join(this.outputDir, filePath);
    const dir = path.dirname(fullPath);
    await fs.mkdir(dir, { recursive: true });
    
    const fileContent = typeof content === 'string' 
      ? content 
      : yaml.dump(content);
      
    await fs.writeFile(fullPath, fileContent);
    this.log(`📝 Saved: ${filePath}`);
  }
  
  interpolate(template, params = {}) {
    let result = template;
    
    // Replace config values
    result = result.replace(/\{config\.([^}]+)\}/g, (match, path) => {
      const value = path.split('.').reduce((obj, key) => obj?.[key], this.config);
      return value !== undefined ? value : match;
    });
    
    // Replace direct params
    Object.entries(params).forEach(([key, value]) => {
      result = result.replace(new RegExp(`\\{${key}\\}`, 'g'), value);
    });
    
    return result;
  }
  
  async executePhase1_CoreFiles() {
    this.log('Phase 1: Generating Core Files', 'group');
    
    // Generate Cargo.toml
    await this.generateWithAI(
      'You are a Rust expert creating a Bevy game project. Use the exact dependency versions specified.',
      this.interpolate(`Generate Cargo.toml for {config.game.title} with these dependencies:
        ${JSON.stringify(this.config.build.dependencies, null, 2)}
        
        Include workspace setup, WASM features, and optimization profiles.`),
      {
        outputPath: 'Cargo.toml',
        temperature: 0.2,
        parseFormat: 'rust'
      }
    );
    
    // Generate main.rs
    await this.generateWithAI(
      'You are a Rust game developer using Bevy ECS. Write idiomatic Rust code.',
      this.interpolate(`Generate main.rs for {config.game.title}:
        - Bevy app with DefaultPlugins
        - Window: {config.graphics.tile_size}px tiles, {config.graphics.perspective}
        - States: Menu, Overworld, Battle, Shop, Dungeon
        - WASM compatibility
        - Integrate bevy_ecs_tilemap, bevy-yoleck`),
      {
        outputPath: 'src/main.rs',
        temperature: 0.3,
        parseFormat: 'rust'
      }
    );
    
    // Generate components
    await this.generateWithAI(
      'Define Bevy components following ECS best practices.',
      'Generate component definitions: Player, Monster, Tile, Stats, Position, etc.',
      {
        outputPath: 'src/components.rs',
        temperature: 0.3,
        parseFormat: 'rust'
      }
    );
    
    this.log('endgroup');
  }
  
  async executePhase2_Tilemaps() {
    this.log('Phase 2: Generating Tilemaps', 'group');
    
    const zones = this.config.environments.outdoor_zones;
    
    for (const zone of zones) {
      // Generate tilemap module
      await this.generateWithAI(
        'You are an expert in bevy_ecs_tilemap. Generate efficient tilemap configurations.',
        this.interpolate(`Generate tilemap for ${zone.name}:
          - Layers: ${JSON.stringify(zone.tilemap_layers)}
          - Chunk size: {config.environments.map_generation.chunk_size}
          - Tiles: ${JSON.stringify(zone.tiles)}`),
        {
          outputPath: `src/tilemaps/${zone.name.toLowerCase().replace(/\s+/g, '_')}_tilemap.rs`,
          temperature: 0.4,
          parseFormat: 'rust'
        }
      );
      
      // Generate tile data
      await this.generateWithAI(
        'Generate tilemap data with indices, collision, and visual properties.',
        `Create tile data for ${zone.name} with collision flags and palette variations`,
        {
          outputPath: `assets/tilemaps/${zone.name.toLowerCase().replace(/\s+/g, '_')}_tiles.yaml`,
          temperature: 0.6,
          parseFormat: 'yaml'
        }
      );
    }
    
    this.log('endgroup');
  }
  
  async executePhase3_Levels() {
    this.log('Phase 3: Generating Levels', 'group');
    
    const levelTypes = ['overworld', 'dungeon', 'cave', 'boss'];
    const algorithms = this.config.environments.map_generation.mapgen_algorithms;
    
    for (const type of levelTypes) {
      for (let i = 1; i <= 3; i++) {
        await this.generateWithAI(
          'Create levels using bevy-yoleck format with mapgen.rs algorithms.',
          `Generate ${type} level ${i} using ${algorithms[type] || 'cellular_automata'} algorithm`,
          {
            outputPath: `assets/levels/${type}_${i}.yol`,
            temperature: 0.7,
            parseFormat: 'yaml'
          }
        );
      }
    }
    
    this.log('endgroup');
  }
  
  async executePhase4_Monsters() {
    this.log('Phase 4: Generating Monsters', 'group');
    
    const { nouns, verbs, adjectives } = this.config.generation_rules;
    
    await this.generateWithAI(
      'Create unique monsters by combining word elements. Balance stats appropriately.',
      this.interpolate(`Generate 25 monsters using:
        Nouns: ${nouns.join(', ')}
        Verbs: ${verbs.join(', ')}
        Adjectives: ${adjectives.join(', ')}
        
        Each with stats, abilities, XP reward, tame difficulty`),
      {
        outputPath: 'assets/data/monsters.yaml',
        temperature: 0.8,
        parseFormat: 'yaml'
      }
    );
    
    this.log('endgroup');
  }
  
  async executePhase5_Systems() {
    this.log('Phase 5: Generating Game Systems', 'group');
    
    // Map system
    await this.generateWithAI(
      'Create procedural map generation using Bevy ECS and mapgen.rs.',
      this.interpolate(`Generate map system with:
        - {config.environments.map_generation.size} maps
        - DAG traversal algorithm
        - Dungeon every 5 maps
        - Shop placement 20-40%`),
      {
        outputPath: 'src/systems/map_system.rs',
        temperature: 0.4,
        parseFormat: 'rust'
      }
    );
    
    // Monster system
    await this.generateWithAI(
      'Create Pokemon/FF hybrid battle system with taming mechanics.',
      'Generate monster system with tame vs slay choice, party management, abilities',
      {
        outputPath: 'src/systems/monster_system.rs',
        temperature: 0.4,
        parseFormat: 'rust'
      }
    );
    
    this.log('endgroup');
  }
  
  async executePhase6_BuildFiles() {
    this.log('Phase 6: Generating Build Files', 'group');
    
    // Web build script
    const buildScript = `#!/bin/bash
set -euo pipefail

echo "🎮 Building ${this.config.game.title} for Web..."

# Ensure WASM target
rustup target add wasm32-unknown-unknown

# Install wasm-bindgen if needed
command -v wasm-bindgen-cli >/dev/null 2>&1 || cargo install wasm-bindgen-cli

# Build
cargo build --release --target wasm32-unknown-unknown

# Generate bindings
wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/${this.config.game.codename}.wasm

echo "✅ Build complete! Serve index.html to play."`;
    
    await this.saveToFile('build-web.sh', buildScript);
    
    // Make executable if not on Windows
    if (process.platform !== 'win32') {
      await fs.chmod(path.join(this.outputDir, 'build-web.sh'), 0o755);
    }
    
    // Generate index.html
    await this.generateWithAI(
      'Create a retro-styled web page for hosting a Bevy WASM game.',
      `Generate index.html for ${this.config.game.title} with pixel-perfect rendering and loading screen`,
      {
        outputPath: 'index.html',
        temperature: 0.3
      }
    );
    
    this.log('endgroup');
  }
  
  async executeFullCascade() {
    this.log(chalk.bold.magenta(`
╔═══════════════════════════════════════════════╗
║   Metaprompt Cascade: ${this.config.game.title.padEnd(23)} ║
╚═══════════════════════════════════════════════╝
    `));
    
    const startTime = Date.now();
    
    try {
      await this.executePhase1_CoreFiles();
      await this.executePhase2_Tilemaps();
      await this.executePhase3_Levels();
      await this.executePhase4_Monsters();
      await this.executePhase5_Systems();
      await this.executePhase6_BuildFiles();
      
      const duration = ((Date.now() - startTime) / 1000).toFixed(1);
      this.log(chalk.bold.green(`\n✅ Metaprompt cascade complete in ${duration}s!`));
      
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
      files_created: [],
      next_steps: [
        'Run `cargo build` to compile',
        'Run `./build-web.sh` for web version',
        'Run `npm run validate` to check assets',
      ]
    };
    
    // List generated files
    const walkDir = async (dir) => {
      try {
        const entries = await fs.readdir(dir, { withFileTypes: true });
        for (const entry of entries) {
          const fullPath = path.join(dir, entry.name);
          if (entry.isDirectory() && !entry.name.startsWith('.') && entry.name !== 'node_modules') {
            await walkDir(fullPath);
          } else if (entry.isFile()) {
            summary.files_created.push(fullPath);
          }
        }
      } catch (error) {
        // Directory doesn't exist yet
      }
    };
    
    await walkDir(this.outputDir);
    await this.saveToFile('GENERATION_SUMMARY.md', this.formatSummary(summary));
  }
  
  formatSummary(summary) {
    return `# Generation Summary

Generated: ${summary.generated_at}

## Game Info
- Title: ${summary.game.title}
- Version: ${summary.game.version}
- Codename: ${summary.game.codename}

## Files Created (${summary.files_created.length})
${summary.files_created.map(f => `- ${f}`).join('\n')}

## Next Steps
${summary.next_steps.map((s, i) => `${i + 1}. ${s}`).join('\n')}
`;
  }
}

// CLI Interface
async function main() {
  const args = process.argv.slice(2);
  const options = {
    dryRun: args.includes('--dry-run'),
    configPath: args.find(a => a.startsWith('--config='))?.split('=')[1],
    outputDir: args.find(a => a.startsWith('--output='))?.split('=')[1],
  };
  
  if (args.includes('--help')) {
    console.log(`
Metaprompt Runner - Execute the complete game generation cascade

Usage: node metaprompt-runner.js [options]

Options:
  --dry-run         Show what would be generated without calling OpenAI
  --config=PATH     Use custom config file (default: game-config.yaml)
  --output=DIR      Output directory (default: current directory)
  --help           Show this help

Environment:
  OPENAI_API_KEY   Required for AI generation

Examples:
  node metaprompt-runner.js
  node metaprompt-runner.js --dry-run
  node metaprompt-runner.js --output=generated/
    `);
    process.exit(0);
  }
  
  try {
    const runner = new MetapromptRunner(options);
    await runner.loadConfig();
    await runner.executeFullCascade();
  } catch (error) {
    console.error(chalk.red(`\n❌ Error: ${error.message}`));
    process.exit(1);
  }
}

// Export for use as module
module.exports = { MetapromptRunner };

// Run if called directly
if (require.main === module) {
  main().catch(console.error);
}