#!/usr/bin/env node

const fs = require('fs').promises;
const path = require('path');
const yaml = require('js-yaml');
const chalk = require('chalk');
const Handlebars = require('handlebars');
const matter = require('gray-matter');
const { MetapromptRunnerV2 } = require('./metaprompt-runner-v2');

/**
 * Metaprompt Executor
 * Executes metaprompts that generate other prompts, creating cascading generation patterns
 */

class MetapromptExecutor {
  constructor(options = {}) {
    this.runner = new MetapromptRunnerV2(options);
    this.metapromptsDir = options.metapromptsDir || '.github/prompts/metaprompts';
    this.outputPromptsDir = options.outputPromptsDir || '.github/prompts/generated';
    this.executionLog = [];
  }
  
  log(message, level = 'info') {
    const timestamp = new Date().toISOString();
    this.executionLog.push({ timestamp, level, message });
    
    switch (level) {
      case 'error':
        console.error(chalk.red(`[META] ${message}`));
        break;
      case 'warning':
        console.warn(chalk.yellow(`[META] ${message}`));
        break;
      case 'success':
        console.log(chalk.green(`[META] ✓ ${message}`));
        break;
      case 'cascade':
        console.log(chalk.cyan(`[META] 🔀 ${message}`));
        break;
      default:
        console.log(`[META] ${message}`);
    }
  }
  
  async initialize() {
    await this.runner.initialize();
    this.log('Metaprompt executor initialized', 'success');
  }
  
  async executeMetaprompt(metapromptName, params = {}) {
    this.log(`Executing metaprompt: ${metapromptName}`, 'cascade');
    
    // Load metaprompt template
    const metapromptPath = path.join(this.metapromptsDir, `${metapromptName}.md`);
    const metapromptContent = await fs.readFile(metapromptPath, 'utf-8');
    const { data: frontmatter, content } = matter(metapromptContent);
    
    // Parse system and user prompts
    const systemMatch = content.match(/<system>([\s\S]*?)<\/system>/);
    const userMatch = content.match(/<user>([\s\S]*?)<\/user>/);
    
    if (!systemMatch || !userMatch) {
      throw new Error(`Invalid metaprompt format in ${metapromptName}`);
    }
    
    // Compile templates with parameters
    const systemPrompt = Handlebars.compile(systemMatch[1].trim())(params);
    const userPrompt = Handlebars.compile(userMatch[1].trim())(params);
    
    // Execute metaprompt to generate new prompts
    const response = await this.runner.generateWithAI(systemPrompt, userPrompt, {
      model: frontmatter.model || 'gpt-4',
      temperature: frontmatter.temperature || 0.3,
      maxTokens: frontmatter.max_tokens || 4000,
      parseFormat: 'yaml',
    });
    
    // Save generated prompts
    const generatedPrompts = await this.saveGeneratedPrompts(response, params);
    
    this.log(`Generated ${generatedPrompts.length} prompt templates`, 'success');
    
    return generatedPrompts;
  }
  
  async saveGeneratedPrompts(promptData, params) {
    const savedPrompts = [];
    
    // Ensure output directory exists
    await fs.mkdir(this.outputPromptsDir, { recursive: true });
    
    // Handle both single prompt and multiple prompts
    const prompts = Array.isArray(promptData) ? promptData : [promptData];
    
    for (const prompt of prompts) {
      if (prompt.filename && prompt.content) {
        const filename = Handlebars.compile(prompt.filename)(params);
        const filepath = path.join(this.outputPromptsDir, filename);
        
        // Ensure subdirectories exist
        await fs.mkdir(path.dirname(filepath), { recursive: true });
        
        // Format as GitHub prompt template
        const formattedContent = this.formatAsGitHubPrompt(prompt);
        
        await fs.writeFile(filepath, formattedContent);
        savedPrompts.push({
          filename,
          filepath,
          template: prompt.template_name || path.basename(filename, '.md'),
        });
        
        this.log(`Saved prompt template: ${filename}`, 'success');
      }
    }
    
    return savedPrompts;
  }
  
  formatAsGitHubPrompt(promptData) {
    const frontmatter = {
      model: promptData.model || 'gpt-4',
      temperature: promptData.temperature || 0.7,
      max_tokens: promptData.max_tokens || 3000,
      ...promptData.frontmatter,
    };
    
    let content = '---\n';
    for (const [key, value] of Object.entries(frontmatter)) {
      content += `${key}: ${value}\n`;
    }
    content += '---\n\n';
    
    if (promptData.system) {
      content += `<system>\n${promptData.system.trim()}\n</system>\n\n`;
    }
    
    if (promptData.user) {
      content += `<user>\n${promptData.user.trim()}\n</user>`;
    }
    
    return content;
  }
  
  async executeCascade(cascadeName, initialParams = {}) {
    this.log(`Starting cascade: ${cascadeName}`, 'cascade');
    
    const cascade = {
      name: cascadeName,
      startTime: Date.now(),
      stages: [],
      generatedAssets: [],
    };
    
    try {
      // Stage 1: Generate level layout prompts
      const layoutPrompts = await this.executeMetaprompt('level-design-cascade', {
        ...initialParams,
        stage: 'layout',
      });
      cascade.stages.push({ name: 'layout', prompts: layoutPrompts });
      
      // Stage 2: Execute layout prompts to generate actual levels
      for (const prompt of layoutPrompts) {
        if (prompt.template.includes('layout')) {
          const levelData = await this.runner.generateWithTemplate(
            `generated/${prompt.template}`,
            initialParams,
            { parseFormat: 'yaml' }
          );
          
          const assetPath = `assets/levels/${initialParams.zone_name_slug}_layout.yol`;
          await this.runner.saveToFile(assetPath, levelData);
          cascade.generatedAssets.push(assetPath);
        }
      }
      
      // Stage 3: Generate entity placement based on layout
      const entityPrompts = await this.executeMetaprompt('entity-placement-cascade', {
        ...initialParams,
        stage: 'entities',
      });
      cascade.stages.push({ name: 'entities', prompts: entityPrompts });
      
      // Continue cascade...
      
    } catch (error) {
      this.log(`Cascade failed: ${error.message}`, 'error');
      throw error;
    }
    
    cascade.endTime = Date.now();
    cascade.duration = (cascade.endTime - cascade.startTime) / 1000;
    
    // Save cascade execution log
    await this.saveCascadeLog(cascade);
    
    return cascade;
  }
  
  async saveCascadeLog(cascade) {
    const logPath = path.join('.cache', 'cascades', `${cascade.name}_${Date.now()}.json`);
    await fs.mkdir(path.dirname(logPath), { recursive: true });
    await fs.writeFile(logPath, JSON.stringify(cascade, null, 2));
    this.log(`Cascade log saved: ${logPath}`, 'success');
  }
  
  async generateBevyIntegration(zone) {
    this.log(`Generating Bevy integration for ${zone.name}`, 'cascade');
    
    // Generate prompts for each Bevy library integration
    const integrations = [];
    
    // bevy_ecs_tilemap integration
    const tilemapPrompt = await this.executeMetaprompt('bevy-tilemap-integration', {
      zone_name: zone.name,
      zone_type: zone.type,
      tiles: zone.tiles,
      layers: zone.tilemap_layers,
      chunk_size: zone.chunk_size || '32x32',
    });
    integrations.push(...tilemapPrompt);
    
    // bevy-yoleck level editor integration
    const yoleckPrompt = await this.executeMetaprompt('bevy-yoleck-integration', {
      zone_name: zone.name,
      entity_types: ['Player', 'Monster', 'Treasure', 'Exit', 'Trigger'],
      level_features: zone.features || [],
    });
    integrations.push(...yoleckPrompt);
    
    // mapgen.rs algorithm integration
    const mapgenPrompt = await this.executeMetaprompt('mapgen-algorithm-integration', {
      zone_name: zone.name,
      algorithm: zone.algorithm || 'cellular_automata',
      size: zone.size || '50x50',
      parameters: zone.generation_params || {},
    });
    integrations.push(...mapgenPrompt);
    
    return integrations;
  }
}

// CLI interface
async function main() {
  const args = process.argv.slice(2);
  const command = args[0];
  
  const executor = new MetapromptExecutor({
    configPath: 'game-config.yaml',
    dryRun: args.includes('--dry-run'),
  });
  
  try {
    await executor.initialize();
    
    switch (command) {
      case 'metaprompt':
        // Execute a single metaprompt
        const metapromptName = args[1];
        const params = JSON.parse(args[2] || '{}');
        await executor.executeMetaprompt(metapromptName, params);
        break;
        
      case 'cascade':
        // Execute a full cascade
        const cascadeName = args[1] || 'full-game';
        await executor.executeCascade(cascadeName, {
          game_title: executor.runner.config.game.title,
          zone_name: 'Verdant Flats',
          zone_name_slug: 'verdant_flats',
          zone_type: 'starter',
          mapgen_algorithm: 'cellular_automata',
          map_size: '50x50',
        });
        break;
        
      case 'bevy-integration':
        // Generate Bevy library integrations
        const zone = executor.runner.config.environments.outdoor_zones[0];
        await executor.generateBevyIntegration(zone);
        break;
        
      default:
        console.log(`
${chalk.bold('Metaprompt Executor')} - Generate cascading prompts for game assets

${chalk.bold('Usage:')}
  node metaprompt-executor.js metaprompt <name> [params]
  node metaprompt-executor.js cascade [name]
  node metaprompt-executor.js bevy-integration

${chalk.bold('Commands:')}
  metaprompt     Execute a single metaprompt
  cascade        Execute a full generation cascade
  bevy-integration Generate Bevy library integrations

${chalk.bold('Options:')}
  --dry-run      Preview without generating

${chalk.bold('Examples:')}
  node metaprompt-executor.js metaprompt level-design-cascade '{"zone_name":"Verdant Flats"}'
  node metaprompt-executor.js cascade full-game
  node metaprompt-executor.js bevy-integration
        `);
    }
  } catch (error) {
    console.error(chalk.red(`Error: ${error.message}`));
    process.exit(1);
  }
}

module.exports = { MetapromptExecutor };

if (require.main === module) {
  main().catch(console.error);
}