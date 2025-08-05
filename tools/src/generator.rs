use anyhow::{Result, Context};
use async_openai::{
    Client,
    config::OpenAIConfig,
    types::{
        CreateChatCompletionRequestArgs,
        CreateImageRequestArgs,
        ChatCompletionRequestMessage,
        ChatCompletionRequestSystemMessageArgs,
        ChatCompletionRequestUserMessageArgs,
        ImageSize,
        ImageModel,
        ResponseFormat,
    },
};
use serde::{Serialize, Deserialize};
use std::path::{Path, PathBuf};
use std::collections::HashSet;
use std::fs;
use tracing::{info, warn, debug};
use indicatif::{ProgressBar, ProgressStyle};
use crate::config::GameConfig;
use crate::templates::Templates;
use crate::git_tracker::{GitGenerationTracker, GenerationManifest, PromptNode};

#[derive(Debug)]
pub struct AIGameGenerator {
    client: Client<OpenAIConfig>,
    config: Option<GameConfig>,
    cache_dir: PathBuf,
    dry_run: bool,
    use_cache: bool,
    generated_files: HashSet<PathBuf>,
    templates: Templates,
    tokenizer: tiktoken_rs::CoreBPE,
    git_tracker: Option<GitGenerationTracker>,
    manifest: Option<GenerationManifest>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CachedResponse {
    content: String,
    timestamp: u64,
}

impl AIGameGenerator {
    pub fn new() -> Self {
        let client = Client::new();
        let cache_dir = PathBuf::from(".cache/ai-gen");
        let tokenizer = tiktoken_rs::cl100k_base().unwrap();
        
        // Try to initialize git tracker
        let git_tracker = GitGenerationTracker::new(".").ok();
        
        Self {
            client,
            config: None,
            cache_dir,
            dry_run: false,
            use_cache: true,
            generated_files: HashSet::new(),
            templates: Templates::new(),
            tokenizer,
            git_tracker,
            manifest: None,
        }
    }
    
    pub fn with_use_cache(mut self, use_cache: bool) -> Self {
        self.use_cache = use_cache;
        self
    }
    
    pub fn with_dry_run(mut self, dry_run: bool) -> Self {
        self.dry_run = dry_run;
        self
    }
    
    pub async fn initialize(&mut self) -> Result<()> {
        info!("🤖 AI Game Generator - Initializing...");
        
        // Load game configuration
        self.config = Some(GameConfig::load("game-config.yaml").await?);
        
        // Check if we can skip generation entirely
        if let (Some(tracker), Some(config)) = (&self.git_tracker, &self.config) {
            let config_json = serde_json::to_value(config)?;
            if tracker.can_skip_generation(&config_json)? {
                info!("✅ All files are up to date. Skipping generation.");
                info!("   Run with --force to regenerate anyway.");
                return Ok(());
            }
            
            // Start new generation manifest
            self.manifest = Some(tracker.start_generation(&config_json)?);
        }
        
        // Create directory structure
        self.setup_directories().await?;
        
        // Load templates
        self.templates.load().await?;
        
        info!("✓ Initialization complete");
        Ok(())
    }
    
    async fn setup_directories(&self) -> Result<()> {
        let dirs = vec![
            "src",
            "src/components",
            "src/systems",
            "src/tilemaps",
            "src/levels",
            "src/ai",
            "assets/sprites",
            "assets/audio",
            "assets/data",
            "assets/levels",
            "assets/quests",
            ".cache/ai-gen",
            ".ai-generation",
        ];
        
        for dir in dirs {
            fs::create_dir_all(dir)?;
        }
        
        Ok(())
    }
    
    async fn generate_with_ai(&self, system_prompt: &str, user_prompt: &str) -> Result<String> {
        // Count tokens
        let system_tokens = self.tokenizer.encode_with_special_tokens(system_prompt).len();
        let user_tokens = self.tokenizer.encode_with_special_tokens(user_prompt).len();
        let total_tokens = system_tokens + user_tokens;
        
        debug!("Token count - System: {}, User: {}, Total: {}", 
               system_tokens, user_tokens, total_tokens);
        
        // Generate cache key
        let cache_key = format!("{:x}", md5::compute(format!("{}{}", system_prompt, user_prompt)));
        let cache_file = self.cache_dir.join(format!("{}.json", cache_key));
        let cache_hit = cache_file.exists() && self.use_cache;
        
        // Track in manifest
        if let Some(manifest) = &self.manifest {
            let prompt_node = PromptNode {
                id: cache_key.clone(),
                prompt_type: "generation".to_string(),
                prompt_hash: cache_key.clone(),
                system_prompt: system_prompt.to_string(),
                user_prompt: user_prompt.to_string(),
                children: vec![],
                generated_files: vec![],
                cache_hit,
            };
            
            // For now, add to root - in real implementation, we'd track the cascade path
            if let Some(tracker) = &self.git_tracker {
                let mut manifest_mut = manifest.clone();
                tracker.track_prompt(&mut manifest_mut, vec![], prompt_node)?;
            }
        }
        
        // Check cache
        if cache_hit {
            let cached_data = fs::read_to_string(&cache_file)?;
            let cached: CachedResponse = serde_json::from_str(&cached_data)?;
            debug!("Cache hit for prompt");
            return Ok(cached.content);
        }
        
        // Create messages
        let messages = vec![
            ChatCompletionRequestMessage::System(
                ChatCompletionRequestSystemMessageArgs::default()
                    .content(system_prompt)
                    .build()?
            ),
            ChatCompletionRequestMessage::User(
                ChatCompletionRequestUserMessageArgs::default()
                    .content(user_prompt)
                    .build()?
            ),
        ];
        
        // Create request
        let request = CreateChatCompletionRequestArgs::default()
            .model("gpt-4-turbo-preview")
            .messages(messages)
            .temperature(0.7)
            .max_tokens(4000)
            .build()?;
        
        // Make API call
        let response = self.client.chat().create(request).await?;
        
        // Extract content
        let content = response.choices
            .first()
            .and_then(|choice| choice.message.content.clone())
            .context("No response content")?;
        
        // Cache response
        if self.use_cache {
            let cached = CachedResponse {
                content: content.clone(),
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)?
                    .as_secs(),
            };
            
            fs::write(cache_file, serde_json::to_string(&cached)?)?;
        }
        
        Ok(content)
    }
    
    async fn generate_image(&self, prompt: &str, filename: &str) -> Result<()> {
        info!("🎨 Generating image: {}", filename);
        
        let request = CreateImageRequestArgs::default()
            .model(ImageModel::DallE3)
            .prompt(prompt)
            .n(1)
            .size(ImageSize::S1024x1024)
            .response_format(ResponseFormat::Url)
            .build()?;
        
        let response = self.client.images().create(request).await?;
        
        if let Some(image_data) = response.data.first() {
            // DALL-E 3 returns URLs, not base64
            if let Some(url) = &image_data.url {
                // Download image from URL
                let image_bytes = reqwest::get(url).await?.bytes().await?;
                let path = PathBuf::from("assets/sprites").join(filename);
                self.write_file(&path, &image_bytes).await?;
            } else {
                warn!("No URL in image response");
            }
        }
        
        Ok(())
    }
    
    async fn write_file<P: AsRef<Path>>(&mut self, path: P, content: &[u8]) -> Result<()> {
        let path = path.as_ref();
        
        if self.dry_run {
            info!("  [DRY RUN] Would write: {}", path.display());
            return Ok(());
        }
        
        fs::write(path, content)?;
        self.generated_files.insert(path.to_path_buf());
        info!("  ✓ {}", path.display());
        
        // Track in manifest
        if let (Some(manifest), Some(tracker)) = (&mut self.manifest, &self.git_tracker) {
            let prompt_hash = format!("{:x}", md5::compute(content));
            tracker.track_file(
                manifest,
                path.to_path_buf(),
                content,
                prompt_hash,
                vec![], // Parent assets would be tracked in real implementation
            )?;
        }
        
        Ok(())
    }
    
    pub async fn generate_game(&mut self, force: bool) -> Result<()> {
        // Check git tracker for incremental generation
        if !force {
            if let (Some(tracker), Some(manifest)) = (&self.git_tracker, &self.manifest) {
                let stale_files = tracker.get_stale_files(manifest)?;
                if stale_files.is_empty() {
                    info!("✅ All files are up to date!");
                    return Ok(());
                } else {
                    info!("📝 {} files need regeneration", stale_files.len());
                    for file in &stale_files {
                        info!("  - {}", file.display());
                    }
                }
            }
        }
        
        let pb = ProgressBar::new(6);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} {msg}")
                .unwrap()
                .progress_chars("#>-")
        );
        
        pb.set_message("Generating core files...");
        self.generate_core().await?;
        pb.inc(1);
        
        pb.set_message("Generating components...");
        self.generate_components().await?;
        pb.inc(1);
        
        pb.set_message("Generating systems...");
        self.generate_systems().await?;
        pb.inc(1);
        
        pb.set_message("Generating levels...");
        self.generate_levels().await?;
        pb.inc(1);
        
        pb.set_message("Generating sprites...");
        self.generate_sprites().await?;
        pb.inc(1);
        
        pb.set_message("Generating audio...");
        self.generate_audio().await?;
        pb.inc(1);
        
        pb.finish_with_message("✅ Game generation complete!");
        
        self.generate_summary().await?;
        
        // Commit to git if tracker is available
        if let (Some(tracker), Some(manifest)) = (&self.git_tracker, &self.manifest) {
            let commit_message = format!(
                "AI Generation: {} files generated",
                self.generated_files.len()
            );
            
            // Preview changes
            let preview = tracker.preview_changes(manifest)?;
            info!("\n{}", preview);
            
            // Commit
            let commit_id = tracker.commit_generation(manifest, &commit_message)?;
            info!("📝 Committed generation: {}", commit_id);
        }
        
        Ok(())
    }
    
    pub async fn generate_core(&mut self) -> Result<()> {
        info!("📦 Generating core files...");
        
        let config = self.config.as_ref().context("Config not loaded")?;
        let game_title = config.game.title.clone();
        let tile_size = config.graphics.tile_size;
        let perspective = config.graphics.perspective.clone();
        
        // Generate Cargo.toml
        let cargo_prompt = format!(
            "Generate a Cargo.toml for a Bevy game called '{}'. \
            Include dependencies for bevy, bevy_ecs_tilemap, bevy-yoleck, and bevy-inspector-egui. \
            Add workspace configuration, WASM support, and optimized release profiles. \
            Output only the Cargo.toml content, no explanations.",
            game_title
        );
        
        let cargo_content = self.generate_with_ai(
            "You are a Rust and Bevy expert.",
            &cargo_prompt
        ).await?;
        
        self.write_file("Cargo.toml", cargo_content.as_bytes()).await?;
        
        // Generate main.rs
        let main_prompt = format!(
            "Generate a main.rs for a Bevy game with: \
            - Title: '{}' \
            - Window size based on {}px tiles \
            - States: Menu, Playing, Paused \
            - Plugins: DefaultPlugins, TilemapPlugin, YoleckPlugin, WorldInspectorPlugin \
            - Basic camera setup for {} view \
            Output only Rust code, no explanations.",
            game_title,
            tile_size,
            perspective
        );
        
        let main_content = self.generate_with_ai(
            "You are a Rust and Bevy game engine expert.",
            &main_prompt
        ).await?;
        
        self.write_file("src/main.rs", main_content.as_bytes()).await?;
        
        Ok(())
    }
    
    pub async fn generate_components(&mut self) -> Result<()> {
        info!("🧩 Generating ECS components...");
        
        let components_code = self.templates.render_components()?;
        self.write_file("src/components.rs", components_code.as_bytes()).await?;
        
        Ok(())
    }
    
    pub async fn generate_systems(&mut self) -> Result<()> {
        info!("⚙️  Generating game systems...");
        
        let systems = vec![
            ("movement", "Basic movement system with velocity and position"),
            ("combat", "Turn-based combat with damage calculation"),
            ("taming", "Monster taming with success rates and party management"),
            ("inventory", "Item storage with equipment slots"),
        ];
        
        for (name, desc) in systems {
            let prompt = format!(
                "Generate a Bevy {} system for a JRPG. \
                {}. \
                Use proper Bevy ECS patterns with Query, Commands, etc. \
                Output only Rust code.",
                name, desc
            );
            
            let code = self.generate_with_ai(
                "You are a Bevy ECS expert.",
                &prompt
            ).await?;
            
            self.write_file(format!("src/systems/{}.rs", name), code.as_bytes()).await?;
        }
        
        Ok(())
    }
    
    pub async fn generate_levels(&mut self) -> Result<()> {
        info!("🗺️  Generating levels...");
        
        let config = self.config.as_ref().context("Config not loaded")?;
        let zones = config.environments.outdoor_zones[..3.min(config.environments.outdoor_zones.len())].to_vec();
        let dungeon_algorithm = config.environments.map_generation.mapgen_algorithms.dungeon.clone();
        let overworld_algorithm = config.environments.map_generation.mapgen_algorithms.overworld.clone();
        
        for zone in zones {
            let algorithm = if zone.zone_type == "dungeon" {
                &dungeon_algorithm
            } else {
                &overworld_algorithm
            };
            
            let level_prompt = format!(
                "Generate a level layout for '{}' using {} algorithm. \
                Size: 50x50 tiles. \
                Output a Bevy-Yoleck format with player spawn, monsters, treasures, and exit. \
                Format as YAML.",
                zone.name, algorithm
            );
            
            let level_data = self.generate_with_ai(
                "You are a game level designer.",
                &level_prompt
            ).await?;
            
            let filename = format!(
                "assets/levels/{}.yol",
                zone.name.to_lowercase().replace(' ', "_")
            );
            
            self.write_file(&filename, level_data.as_bytes()).await?;
        }
        
        Ok(())
    }
    
    pub async fn generate_sprites(&mut self) -> Result<()> {
        info!("🖼️  Generating sprites with DALL-E 3...");
        
        let config = self.config.as_ref().context("Config not loaded")?;
        
        // Generate hero sprite
        let hero_prompt = format!(
            "Pixel art character sprite for JRPG hero '{}': {}. \
            32x32 pixels, clean pixel art style, facing right.",
            config.hero.name, config.hero.description
        );
        
        match self.generate_image(&hero_prompt, "hero.png").await {
            Ok(_) => {
                // The write_file already tracks the file
            }
            Err(e) => {
                warn!("Failed to generate hero sprite: {}", e);
            }
        }
        
        // Generate tileset
        let tileset_prompt = "Pixel art tileset for JRPG, 2x2 grid: \
            grass (top-left), stone (top-right), water (bottom-left), dirt (bottom-right). \
            Each tile 16x16 pixels, vibrant retro style.";
        
        match self.generate_image(tileset_prompt, "tileset.png").await {
            Ok(_) => {
                // The write_file already tracks the file
            }
            Err(e) => {
                warn!("Failed to generate tileset: {}", e);
            }
        }
        
        Ok(())
    }
    
    pub async fn generate_audio(&mut self) -> Result<()> {
        info!("🎵 Generating audio specifications...");
        
        // Since OpenAI doesn't have music generation yet, create specs for procedural audio
        let audio_specs = serde_json::json!({
            "menu_theme": {
                "type": "melody",
                "tempo": 120,
                "key": "C_major",
                "pattern": [
                    {"note": "C4", "duration": 0.25, "time": 0},
                    {"note": "E4", "duration": 0.25, "time": 0.25},
                    {"note": "G4", "duration": 0.25, "time": 0.5},
                    {"note": "C5", "duration": 0.25, "time": 0.75}
                ]
            },
            "sfx": {
                "coin_pickup": {
                    "frequencies": [523.25, 659.25, 783.99],
                    "duration": 0.3
                },
                "menu_select": {
                    "frequency": 440,
                    "duration": 0.1
                }
            }
        });
        
        self.write_file(
            "assets/audio/audio_specs.json",
            serde_json::to_string_pretty(&audio_specs)?.as_bytes()
        ).await?;
        
        Ok(())
    }
    
    async fn generate_summary(&self) -> Result<()> {
        let summary = serde_json::json!({
            "game": self.config.as_ref().map(|c| &c.game),
            "generated": {
                "files": self.generated_files.iter().map(|p| p.display().to_string()).collect::<Vec<_>>(),
                "timestamp": chrono::Utc::now().to_rfc3339(),
            },
            "next_steps": [
                "Run 'cargo build' to compile",
                "Run 'cargo run' to test",
                "Use bevy-inspector-egui for runtime editing",
                "Generate more assets as needed"
            ]
        });
        
        fs::write(
            "GENERATION_SUMMARY.json",
            serde_json::to_string_pretty(&summary)?
        )?;
        
        info!("📊 Summary written to GENERATION_SUMMARY.json");
        
        Ok(())
    }
    
    pub async fn test(&self) -> Result<()> {
        let response = self.generate_with_ai(
            "You are a helpful assistant.",
            "Say 'Hello from Rust AI generator!'"
        ).await?;
        
        info!("Test response: {}", response);
        
        Ok(())
    }
    
    pub async fn clean(&self) -> Result<()> {
        warn!("Clean not implemented yet");
        Ok(())
    }
}