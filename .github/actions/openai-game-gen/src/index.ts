import * as core from '@actions/core';
import * as fs from 'fs/promises';
import * as path from 'path';
import * as yaml from 'js-yaml';
import { Configuration, OpenAIApi } from 'openai';

interface GameConfig {
  game: any;
  theme: any;
  gameplay: any;
  monsters: any;
  environments: any;
  generation_rules: any;
  build: any;
}

interface GenerationTemplate {
  system_base: string;
  user_base: string;
  output_format?: string;
  validation_rules?: string[];
}

const GENERATION_TEMPLATES: Record<string, GenerationTemplate> = {
  tilemap: {
    system_base: `You are an expert in bevy_ecs_tilemap and procedural tilemap generation.
Generate tilemap configurations that are efficient and visually appealing.
Use the bevy_ecs_tilemap API correctly with proper layer management.`,
    user_base: `Generate a tilemap configuration for {zone_name} with:
- Tile size: {tile_size}
- Map size: {map_size}
- Layers: terrain, decorations, collision
- Tile indices mapped to sprite sheet positions
- Proper chunk sizing for performance`,
    output_format: 'rust'
  },
  
  level: {
    system_base: `You are an expert in level design using bevy-yoleck and mapgen.rs algorithms.
Create level definitions that use mapgen.rs algorithms for generation.
Output yoleck-compatible entity definitions.`,
    user_base: `Generate a level using {algorithm} from mapgen.rs:
- Size: {width}x{height}
- Features: {features}
- Entity spawns with yoleck markers
- Proper room/corridor/POI distribution`,
    output_format: 'yaml'
  },
  
  monster: {
    system_base: `You are creating monsters for a Pokemon/FF hybrid game.
Use the word combination rules to create unique creatures.
Ensure balanced stats and interesting abilities.`,
    user_base: `Generate {count} monsters using:
- Nouns: {nouns}
- Verbs: {verbs}
- Adjectives: {adjectives}
- Level range: {level_range}
- Include sprite references and color palettes`,
    output_format: 'yaml'
  },
  
  code: {
    system_base: `You are a Rust game developer using Bevy ECS.
Write idiomatic Rust code that follows Bevy best practices.
Use the specified crates correctly.`,
    user_base: `{prompt}`,
    output_format: 'rust'
  }
};

async function loadGameConfig(configPath: string): Promise<GameConfig> {
  const content = await fs.readFile(configPath, 'utf-8');
  return yaml.load(content) as GameConfig;
}

async function loadTemplate(templatePath: string | undefined): Promise<any> {
  if (!templatePath) return null;
  const content = await fs.readFile(templatePath, 'utf-8');
  return yaml.load(content);
}

export function interpolatePrompt(template: string, config: GameConfig, params: any): string {
  let result = template;
  
  // Replace config references
  result = result.replace(/\{config\.([^}]+)\}/g, (match, path) => {
    const value = path.split('.').reduce((obj: any, key: string) => obj?.[key], config);
    return value || match;
  });
  
  // Replace direct parameters
  result = result.replace(/\{([^}]+)\}/g, (match, key) => {
    return params[key] || match;
  });
  
  return result;
}

async function generateWithRetry(
  openai: OpenAIApi,
  messages: any[],
  model: string,
  temperature: number,
  maxTokens: number,
  retries: number = 3
): Promise<string> {
  for (let i = 0; i < retries; i++) {
    try {
      const response = await openai.createChatCompletion({
        model,
        messages,
        temperature,
        max_tokens: maxTokens,
      });
      
      return response.data.choices[0]?.message?.content || '';
    } catch (error: any) {
      if (i === retries - 1) throw error;
      core.warning(`Retry ${i + 1}/${retries} after error: ${error.message}`);
      await new Promise(resolve => setTimeout(resolve, 1000 * (i + 1)));
    }
  }
  throw new Error('Failed after all retries');
}

export function parseGeneratedContent(content: string, format: string | undefined): any {
  if (!format) return content;
  
  try {
    switch (format) {
      case 'json':
        return JSON.parse(content);
      
      case 'yaml':
        return yaml.load(content);
      
      case 'rust':
        // Extract code blocks if wrapped in markdown
        const rustMatch = content.match(/```rust\n([\s\S]*?)\n```/);
        return rustMatch ? rustMatch[1] : content;
      
      case 'toml':
        // For TOML, we'd need a parser, but for now return as-is
        return content;
      
      default:
        return content;
    }
  } catch (error) {
    core.warning(`Failed to parse as ${format}, returning raw content`);
    return content;
  }
}

async function run(): Promise<void> {
  try {
    // Get inputs
    const apiKey = core.getInput('api_key', { required: true });
    const outputPath = core.getInput('output_path', { required: true });
    const generationType = core.getInput('generation_type', { required: true });
    const model = core.getInput('model') || 'gpt-4';
    const temperature = parseFloat(core.getInput('temperature') || '0.7');
    const maxTokens = parseInt(core.getInput('max_tokens') || '4000');
    const configPath = core.getInput('config_path') || 'game-config.yaml';
    const templatePath = core.getInput('template_path');
    const parseFormat = core.getInput('parse_format');
    
    let systemPrompt = core.getInput('system_prompt');
    let userPrompt = core.getInput('user_prompt', { required: true });
    
    // Load game configuration
    const gameConfig = await loadGameConfig(configPath);
    const customTemplate = await loadTemplate(templatePath);
    
    // Get generation template if available
    const genTemplate = GENERATION_TEMPLATES[generationType];
    if (genTemplate && !systemPrompt) {
      systemPrompt = genTemplate.system_base;
    }
    
    // Parse user prompt as JSON if it contains parameters
    let promptParams = {};
    try {
      promptParams = JSON.parse(userPrompt);
      if (genTemplate && promptParams.prompt) {
        userPrompt = genTemplate.user_base;
      }
    } catch {
      // Not JSON, use as-is
    }
    
    // Interpolate prompts with config and parameters
    systemPrompt = interpolatePrompt(systemPrompt || '', gameConfig, promptParams);
    userPrompt = interpolatePrompt(userPrompt, gameConfig, promptParams);
    
    // Add generation-specific context
    if (generationType === 'tilemap') {
      systemPrompt += `\n\nUse bevy_ecs_tilemap 0.12 API. Generate valid Rust code that compiles.`;
    } else if (generationType === 'level') {
      systemPrompt += `\n\nUse bevy-yoleck entity format. Reference mapgen.rs algorithms correctly.`;
    }
    
    // Configure OpenAI
    const configuration = new Configuration({ apiKey });
    const openai = new OpenAIApi(configuration);
    
    // Build messages
    const messages = [
      { role: 'system', content: systemPrompt },
      { role: 'user', content: userPrompt }
    ];
    
    // Add custom template context if provided
    if (customTemplate) {
      messages.push({
        role: 'system',
        content: `Use this template structure:\n${yaml.dump(customTemplate)}`
      });
    }
    
    // Generate content
    core.info(`Generating ${generationType} content...`);
    const content = await generateWithRetry(openai, messages, model, temperature, maxTokens);
    
    // Parse content if format specified
    const outputFormat = parseFormat || genTemplate?.output_format;
    const parsedContent = parseGeneratedContent(content, outputFormat);
    
    // Create output directory
    const outputDir = path.dirname(outputPath);
    await fs.mkdir(outputDir, { recursive: true });
    
    // Write output file
    if (typeof parsedContent === 'string') {
      await fs.writeFile(outputPath, parsedContent);
    } else {
      // Write as YAML for structured data
      await fs.writeFile(outputPath, yaml.dump(parsedContent));
    }
    
    // Set outputs
    core.setOutput('content', content);
    if (parsedContent !== content) {
      core.setOutput('parsed_data', JSON.stringify(parsedContent));
    }
    
    core.info(`✅ Generated ${generationType} content to ${outputPath}`);
    
  } catch (error: any) {
    core.setFailed(error.message);
  }
}

run();