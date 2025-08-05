#!/usr/bin/env node

const fs = require('fs').promises;
const path = require('path');
const yaml = require('js-yaml');
const chalk = require('chalk');

/**
 * Asset Validator for Echoes of Beastlight
 * Validates all game assets and reports issues
 */

class AssetValidator {
  constructor() {
    this.errors = [];
    this.warnings = [];
    this.assetsDir = path.join(__dirname, '..', 'assets');
  }

  log(message, level = 'info') {
    switch (level) {
      case 'error':
        console.error(chalk.red(`❌ ${message}`));
        this.errors.push(message);
        break;
      case 'warn':
        console.warn(chalk.yellow(`⚠️  ${message}`));
        this.warnings.push(message);
        break;
      case 'success':
        console.log(chalk.green(`✅ ${message}`));
        break;
      default:
        console.log(chalk.blue(`ℹ️  ${message}`));
    }
  }

  async validateYAMLFile(filePath) {
    try {
      const content = await fs.readFile(filePath, 'utf-8');
      const data = yaml.load(content);
      
      // Check if it's empty
      if (!data || Object.keys(data).length === 0) {
        this.log(`Empty YAML file: ${filePath}`, 'warn');
        return false;
      }
      
      return true;
    } catch (error) {
      this.log(`Invalid YAML in ${filePath}: ${error.message}`, 'error');
      return false;
    }
  }

  async validateMonsterData() {
    const monstersFile = path.join(this.assetsDir, 'data', 'monsters.yaml');
    
    try {
      const content = await fs.readFile(monstersFile, 'utf-8');
      const monsters = yaml.load(content);
      
      if (!Array.isArray(monsters)) {
        this.log('monsters.yaml should contain an array of monsters', 'error');
        return;
      }
      
      monsters.forEach((monster, index) => {
        // Validate required fields
        const required = ['id', 'name', 'stats', 'abilities'];
        for (const field of required) {
          if (!monster[field]) {
            this.log(`Monster ${index} missing required field: ${field}`, 'error');
          }
        }
        
        // Validate stats
        if (monster.stats) {
          const requiredStats = ['hp', 'atk', 'def', 'spd'];
          for (const stat of requiredStats) {
            if (typeof monster.stats[stat] !== 'number') {
              this.log(`Monster ${monster.name || index} has invalid ${stat} stat`, 'error');
            }
          }
        }
        
        // Validate abilities
        if (monster.abilities && !Array.isArray(monster.abilities)) {
          this.log(`Monster ${monster.name || index} abilities should be an array`, 'error');
        }
      });
      
      this.log(`Validated ${monsters.length} monsters`, 'success');
    } catch (error) {
      if (error.code === 'ENOENT') {
        this.log('monsters.yaml not found', 'warn');
      } else {
        this.log(`Error validating monsters: ${error.message}`, 'error');
      }
    }
  }

  async validateLevelFiles() {
    const levelsDir = path.join(this.assetsDir, 'levels');
    
    try {
      const files = await fs.readdir(levelsDir);
      const yolFiles = files.filter(f => f.endsWith('.yol'));
      
      for (const file of yolFiles) {
        const filePath = path.join(levelsDir, file);
        const valid = await this.validateYAMLFile(filePath);
        
        if (valid) {
          // Additional yoleck-specific validation
          const content = await fs.readFile(filePath, 'utf-8');
          const data = yaml.load(content);
          
          if (Array.isArray(data)) {
            data.forEach((entity, index) => {
              if (!entity.type) {
                this.log(`Entity ${index} in ${file} missing 'type' field`, 'error');
              }
              if (!entity.position) {
                this.log(`Entity ${index} in ${file} missing 'position' field`, 'warn');
              }
            });
          }
        }
      }
      
      this.log(`Validated ${yolFiles.length} level files`, 'success');
    } catch (error) {
      this.log(`Error validating levels: ${error.message}`, 'error');
    }
  }

  async validateTilemapData() {
    const tilemapsDir = path.join(this.assetsDir, 'tilemaps');
    
    try {
      const files = await fs.readdir(tilemapsDir);
      const yamlFiles = files.filter(f => f.endsWith('.yaml') || f.endsWith('.yml'));
      
      for (const file of yamlFiles) {
        const filePath = path.join(tilemapsDir, file);
        const valid = await this.validateYAMLFile(filePath);
        
        if (valid && file.includes('_tiles')) {
          // Validate tile data structure
          const content = await fs.readFile(filePath, 'utf-8');
          const data = yaml.load(content);
          
          if (data.tiles && !Array.isArray(data.tiles)) {
            this.log(`${file}: tiles should be an array`, 'error');
          }
          
          if (data.layers && !Array.isArray(data.layers)) {
            this.log(`${file}: layers should be an array`, 'error');
          }
        }
      }
      
      this.log(`Validated ${yamlFiles.length} tilemap files`, 'success');
    } catch (error) {
      this.log(`Error validating tilemaps: ${error.message}`, 'error');
    }
  }

  async checkRequiredAssets() {
    const requiredPaths = [
      'data/monsters.yaml',
      'data/items.yaml',
      'data/quest_templates.yaml',
      'prompts/dalle_prompts.txt',
      'audio/audio_specs.yaml'
    ];
    
    for (const reqPath of requiredPaths) {
      const fullPath = path.join(this.assetsDir, reqPath);
      try {
        await fs.access(fullPath);
      } catch {
        this.log(`Missing required asset: ${reqPath}`, 'warn');
      }
    }
  }

  async validateAll() {
    console.log(chalk.bold.blue('\n🔍 Validating Echoes of Beastlight Assets\n'));
    
    await this.checkRequiredAssets();
    await this.validateMonsterData();
    await this.validateLevelFiles();
    await this.validateTilemapData();
    
    console.log(chalk.bold('\n📊 Validation Summary:'));
    console.log(chalk.green(`   ✅ Success`));
    console.log(chalk.red(`   ❌ Errors: ${this.errors.length}`));
    console.log(chalk.yellow(`   ⚠️  Warnings: ${this.warnings.length}`));
    
    if (this.errors.length > 0) {
      console.log(chalk.red('\n❌ Validation failed with errors'));
      process.exit(1);
    } else if (this.warnings.length > 0) {
      console.log(chalk.yellow('\n⚠️  Validation passed with warnings'));
    } else {
      console.log(chalk.green('\n✅ All assets validated successfully!'));
    }
  }
}

// Run validation
if (require.main === module) {
  const validator = new AssetValidator();
  validator.validateAll().catch(console.error);
}

module.exports = { AssetValidator };