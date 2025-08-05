#!/usr/bin/env node

const fs = require('fs').promises;
const path = require('path');
const { spawn } = require('child_process');
const yaml = require('js-yaml');
const { Octokit } = require('@octokit/rest');
const chalk = require('chalk');

/**
 * Beastlight Build Tool
 * Orchestrates the entire game generation process
 */

class BeastlightBuilder {
  constructor(options = {}) {
    this.configPath = options.configPath || 'game-config.yaml';
    this.githubToken = options.githubToken || process.env.GITHUB_TOKEN;
    this.openaiKey = options.openaiKey || process.env.OPENAI_API_KEY;
    this.workflowsDir = '.github/workflows';
    this.verbose = options.verbose || false;
    
    if (this.githubToken) {
      this.octokit = new Octokit({ auth: this.githubToken });
    }
  }
  
  async loadConfig() {
    const configContent = await fs.readFile(this.configPath, 'utf-8');
    this.config = yaml.load(configContent);
    this.log(chalk.green('✓ Loaded game configuration'));
    return this.config;
  }
  
  log(message, level = 'info') {
    if (level === 'error') {
      console.error(chalk.red(message));
    } else if (level === 'warn') {
      console.warn(chalk.yellow(message));
    } else if (this.verbose || level === 'info') {
      console.log(message);
    }
  }
  
  async runCommand(command, args = [], options = {}) {
    return new Promise((resolve, reject) => {
      const child = spawn(command, args, {
        stdio: this.verbose ? 'inherit' : 'pipe',
        ...options
      });
      
      let stdout = '';
      let stderr = '';
      
      if (!this.verbose) {
        child.stdout?.on('data', (data) => { stdout += data; });
        child.stderr?.on('data', (data) => { stderr += data; });
      }
      
      child.on('close', (code) => {
        if (code !== 0) {
          reject(new Error(`Command failed: ${command} ${args.join(' ')}\n${stderr}`));
        } else {
          resolve({ stdout, stderr });
        }
      });
    });
  }
  
  async triggerWorkflow(workflowName, inputs = {}) {
    if (!this.githubToken) {
      this.log('No GitHub token, running workflow locally', 'warn');
      return this.runWorkflowLocally(workflowName, inputs);
    }
    
    try {
      const [owner, repo] = process.env.GITHUB_REPOSITORY?.split('/') || ['', ''];
      
      if (!owner || !repo) {
        throw new Error('GITHUB_REPOSITORY not set');
      }
      
      await this.octokit.actions.createWorkflowDispatch({
        owner,
        repo,
        workflow_id: `${workflowName}.yml`,
        ref: process.env.GITHUB_REF || 'main',
        inputs
      });
      
      this.log(chalk.green(`✓ Triggered workflow: ${workflowName}`));
    } catch (error) {
      this.log(`Failed to trigger workflow: ${error.message}`, 'error');
      throw error;
    }
  }
  
  async runWorkflowLocally(workflowName, inputs) {
    // Simulate workflow execution locally
    this.log(`Running ${workflowName} locally...`);
    
    switch (workflowName) {
      case 'generate-tilemaps':
        await this.generateTilemaps(inputs.zones || 'all');
        break;
      case 'generate-levels':
        await this.generateLevels(inputs.level_types || ['overworld', 'dungeon']);
        break;
      case 'generate-monsters':
        await this.generateMonsters(inputs.count || 20);
        break;
      default:
        this.log(`Unknown workflow: ${workflowName}`, 'warn');
    }
  }
  
  async generateTilemaps(zones) {
    this.log(chalk.blue('🗺️  Generating tilemaps...'));
    
    const zoneList = zones === 'all' 
      ? this.config.environments.outdoor_zones.map(z => z.name)
      : zones;
      
    for (const zone of zoneList) {
      this.log(`  Generating tilemap for ${zone}`);
      // In a real implementation, this would call the OpenAI action
      await this.generateFile(
        `src/tilemaps/${zone}_tilemap.rs`,
        'tilemap',
        { zone_name: zone }
      );
    }
  }
  
  async generateLevels(levelTypes) {
    this.log(chalk.blue('🏔️  Generating levels...'));
    
    for (const type of levelTypes) {
      this.log(`  Generating ${type} levels`);
      // Generate multiple variations
      for (let i = 1; i <= 5; i++) {
        await this.generateFile(
          `assets/levels/${type}_${i}.yol`,
          'level',
          { level_type: type, variation: i }
        );
      }
    }
  }
  
  async generateMonsters(count) {
    this.log(chalk.blue('👾 Generating monsters...'));
    
    await this.generateFile(
      'assets/data/monsters.yaml',
      'monster',
      { count }
    );
  }
  
  async generateFile(outputPath, generationType, params) {
    // This would integrate with the custom OpenAI action
    // For now, create placeholder files
    const dir = path.dirname(outputPath);
    await fs.mkdir(dir, { recursive: true });
    
    const placeholder = `// Generated ${generationType} file\n// Params: ${JSON.stringify(params)}\n`;
    await fs.writeFile(outputPath, placeholder);
    
    this.log(`    Created: ${outputPath}`, 'verbose');
  }
  
  async buildGame() {
    this.log(chalk.bold.blue('\n🎮 Building Echoes of Beastlight\n'));
    
    try {
      // Phase 1: Setup
      await this.loadConfig();
      await this.validateEnvironment();
      
      // Phase 2: Generate Assets
      this.log(chalk.bold('\n📦 Phase 1: Generating Assets'));
      await this.triggerWorkflow('generate-tilemaps', { zones: 'all' });
      await this.triggerWorkflow('generate-levels', { 
        level_types: ['overworld', 'dungeon', 'cave', 'boss'],
        count_per_type: 5 
      });
      
      // Phase 3: Generate Code
      this.log(chalk.bold('\n🔧 Phase 2: Generating Code'));
      await this.triggerWorkflow('metaprompt-executor');
      
      // Phase 4: Build
      this.log(chalk.bold('\n🏗️  Phase 3: Building Game'));
      await this.buildRust();
      
      // Phase 5: Package
      this.log(chalk.bold('\n📦 Phase 4: Packaging'));
      await this.packageGame();
      
      this.log(chalk.bold.green('\n✅ Build complete!'));
      
    } catch (error) {
      this.log(`Build failed: ${error.message}`, 'error');
      process.exit(1);
    }
  }
  
  async validateEnvironment() {
    const checks = [
      { name: 'Rust', command: 'rustc', args: ['--version'] },
      { name: 'Cargo', command: 'cargo', args: ['--version'] },
      { name: 'wasm-pack', command: 'wasm-pack', args: ['--version'] },
    ];
    
    for (const check of checks) {
      try {
        await this.runCommand(check.command, check.args);
        this.log(chalk.green(`✓ ${check.name} is installed`));
      } catch {
        this.log(`✗ ${check.name} is not installed`, 'error');
        throw new Error(`Missing dependency: ${check.name}`);
      }
    }
  }
  
  async buildRust() {
    this.log('Building native version...');
    await this.runCommand('cargo', ['build', '--release']);
    
    this.log('Building WASM version...');
    await this.runCommand('cargo', ['build', '--release', '--target', 'wasm32-unknown-unknown']);
    
    this.log('Running wasm-bindgen...');
    await this.runCommand('wasm-bindgen', [
      '--out-dir', './out',
      '--target', 'web',
      './target/wasm32-unknown-unknown/release/beastlight.wasm'
    ]);
  }
  
  async packageGame() {
    // Create distribution directories
    await fs.mkdir('dist/native', { recursive: true });
    await fs.mkdir('dist/web', { recursive: true });
    
    // Copy native build
    await fs.copyFile(
      'target/release/beastlight',
      'dist/native/beastlight'
    );
    
    // Copy web build
    await fs.copyFile('out/beastlight.js', 'dist/web/beastlight.js');
    await fs.copyFile('out/beastlight_bg.wasm', 'dist/web/beastlight_bg.wasm');
    await fs.copyFile('index.html', 'dist/web/index.html');
    
    // Copy assets
    await this.copyDir('assets', 'dist/native/assets');
    await this.copyDir('assets', 'dist/web/assets');
    
    this.log(chalk.green('✓ Game packaged to dist/'));
  }
  
  async copyDir(src, dest) {
    await fs.mkdir(dest, { recursive: true });
    const entries = await fs.readdir(src, { withFileTypes: true });
    
    for (const entry of entries) {
      const srcPath = path.join(src, entry.name);
      const destPath = path.join(dest, entry.name);
      
      if (entry.isDirectory()) {
        await this.copyDir(srcPath, destPath);
      } else {
        await fs.copyFile(srcPath, destPath);
      }
    }
  }
  
  async clean() {
    this.log(chalk.blue('🧹 Cleaning build artifacts...'));
    
    const dirsToClean = [
      'target',
      'dist',
      'out',
      'node_modules'
    ];
    
    for (const dir of dirsToClean) {
      try {
        await fs.rm(dir, { recursive: true, force: true });
        this.log(`  Removed ${dir}`);
      } catch {
        // Ignore if doesn't exist
      }
    }
    
    this.log(chalk.green('✓ Clean complete'));
  }
}

// CLI Interface
async function main() {
  const args = process.argv.slice(2);
  const command = args[0] || 'build';
  
  const builder = new BeastlightBuilder({
    verbose: args.includes('--verbose') || args.includes('-v'),
    configPath: args.find(a => a.startsWith('--config='))?.split('=')[1] || 'game-config.yaml'
  });
  
  console.log(chalk.bold.magenta(`
╔═══════════════════════════════════════╗
║   Echoes of Beastlight Build Tool     ║
╚═══════════════════════════════════════╝
  `));
  
  switch (command) {
    case 'build':
      await builder.buildGame();
      break;
      
    case 'clean':
      await builder.clean();
      break;
      
    case 'tilemaps':
      await builder.loadConfig();
      await builder.generateTilemaps('all');
      break;
      
    case 'levels':
      await builder.loadConfig();
      await builder.generateLevels(['overworld', 'dungeon', 'cave', 'boss']);
      break;
      
    case 'help':
    default:
      console.log(`
Usage: node tools/build.js [command] [options]

Commands:
  build     Build the complete game (default)
  clean     Remove build artifacts
  tilemaps  Generate only tilemaps
  levels    Generate only levels
  help      Show this help

Options:
  --verbose, -v     Show detailed output
  --config=PATH     Use custom config file

Examples:
  node tools/build.js build --verbose
  node tools/build.js clean
  node tools/build.js tilemaps
      `);
  }
}

if (require.main === module) {
  main().catch(console.error);
}

module.exports = { BeastlightBuilder };