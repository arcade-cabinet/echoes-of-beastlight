use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tokio::fs;
use async_openai::Client;
use crate::config::GameConfig;

#[derive(Debug, Deserialize)]
pub struct AssetPrompt {
    pub prompt: PromptConfig,
    pub context: Option<ContextConfig>,
    pub style: Option<StyleConfig>,
    pub output: OutputConfig,
}

#[derive(Debug, Deserialize)]
pub struct PromptConfig {
    #[serde(rename = "type")]
    pub prompt_type: String,
    pub target: String,
}

#[derive(Debug, Deserialize)]
pub struct ContextConfig {
    pub config_path: Option<String>,
    pub palette_source: Option<String>,
    pub sprite_config: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct StyleConfig {
    pub description: String,
    pub pixel_perfect: Option<bool>,
    pub outline_width: Option<f32>,
    pub color_limit: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct OutputConfig {
    pub format: String,
    pub filename: String,
    pub layout: Option<String>,
    pub background: Option<String>,
}

pub struct TargetedGenerator {
    client: Client<async_openai::config::OpenAIConfig>,
    game_config: GameConfig,
    output_dir: PathBuf,
}

impl TargetedGenerator {
    pub fn new(output_dir: PathBuf) -> Self {
        Self {
            client: Client::new(),
            game_config: GameConfig::default(),
            output_dir,
        }
    }
    
    /// Generate assets based on a TOML prompt file
    pub async fn generate_from_prompt(&self, prompt_path: &Path) -> Result<()> {
        let content = fs::read_to_string(prompt_path).await
            .context("Failed to read prompt file")?;
            
        let prompt: AssetPrompt = toml::from_str(&content)
            .context("Failed to parse prompt TOML")?;
            
        match prompt.prompt.prompt_type.as_str() {
            "sprite_sheet" => self.generate_sprite_sheet(&prompt).await?,
            "sprite_batch" => self.generate_sprite_batch(&prompt).await?,
            "audio_spec" => self.generate_audio_spec(&prompt).await?,
            "level_layout" => self.generate_level_layout(&prompt).await?,
            _ => anyhow::bail!("Unknown prompt type: {}", prompt.prompt.prompt_type),
        }
        
        Ok(())
    }
    
    async fn generate_sprite_sheet(&self, prompt: &AssetPrompt) -> Result<()> {
        // Build the system prompt with our embedded style configuration
        let system_prompt = format!(
            "You are a pixel art generator for a game called {}. \
             The art style is {}. \
             Use these exact colors from the palette: {:?}",
            self.game_config.metadata.name,
            self.game_config.style.visual.art_style.description,
            self.game_config.style.visual.palette
        );
        
        // Build the user prompt from the TOML specification
        let user_prompt = format!(
            "Generate a sprite sheet for: {}\n\n{}",
            prompt.prompt.target,
            prompt.style.as_ref().map(|s| &s.description).unwrap_or(&String::new())
        );
        
        // For now, we'll generate a specification rather than actual images
        let spec = self.generate_sprite_specification(&system_prompt, &user_prompt).await?;
        
        // Save the specification
        let output_path = self.output_dir.join(&prompt.output.filename)
            .with_extension("yaml");
        fs::write(&output_path, spec).await?;
        
        Ok(())
    }
    
    async fn generate_sprite_batch(&self, _prompt: &AssetPrompt) -> Result<()> {
        // Similar to sprite_sheet but for multiple sprites
        todo!("Implement sprite batch generation")
    }
    
    async fn generate_audio_spec(&self, _prompt: &AssetPrompt) -> Result<()> {
        // Generate audio specifications
        todo!("Implement audio specification generation")
    }
    
    async fn generate_level_layout(&self, _prompt: &AssetPrompt) -> Result<()> {
        // Generate level layouts using bevy-yoleck format
        todo!("Implement level layout generation")
    }
    
    async fn generate_sprite_specification(&self, system: &str, user: &str) -> Result<String> {
        use async_openai::types::{
            CreateChatCompletionRequestArgs,
            ChatCompletionRequestSystemMessageArgs,
            ChatCompletionRequestUserMessageArgs,
        };
        
        let request = CreateChatCompletionRequestArgs::default()
            .model("gpt-4")
            .messages([
                ChatCompletionRequestSystemMessageArgs::default()
                    .content(system)
                    .build()?
                    .into(),
                ChatCompletionRequestUserMessageArgs::default()
                    .content(user)
                    .build()?
                    .into(),
            ])
            .temperature(0.7)
            .build()?;
            
        let response = self.client.chat().create(request).await?;
        
        Ok(response.choices[0].message.content.clone().unwrap_or_default())
    }
}

/// Scan for generate.toml files and process them
pub async fn scan_and_generate(root_dir: &Path) -> Result<()> {
    let generator = TargetedGenerator::new(root_dir.to_path_buf());
    
    // Find all generate.toml files
    let walker = walkdir::WalkDir::new(root_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_name() == "generate.toml");
        
    for entry in walker {
        println!("Processing: {}", entry.path().display());
        if let Err(e) = generator.generate_from_prompt(entry.path()).await {
            eprintln!("Failed to process {}: {}", entry.path().display(), e);
        }
    }
    
    Ok(())
}