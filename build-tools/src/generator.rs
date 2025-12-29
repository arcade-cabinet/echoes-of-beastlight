// AI Game Generator - Procedural game generation using AI
// Copyright (C) 2024 AI Game Generator Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the MIT License as published by
// the Open Source Initiative.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.

use crate::config::GameConfig;
use crate::git_tracker::{GenerationManifest, GitGenerationTracker, PromptNode};
use crate::templates::Templates;
use anyhow::{Context, Result};
use async_openai::{
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestMessage, ChatCompletionRequestSystemMessageArgs,
        ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs,
        CreateImageRequestArgs, Image, ImageSize,
    },
    Client,
};
use indicatif::{ProgressBar, ProgressStyle};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use tracing::{debug, info, warn};

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

    async fn generate_with_ai(&mut self, system_prompt: &str, user_prompt: &str) -> Result<String> {
        // Count tokens
        let system_tokens = self
            .tokenizer
            .encode_with_special_tokens(system_prompt)
            .len();
        let user_tokens = self.tokenizer.encode_with_special_tokens(user_prompt).len();
        let total_tokens = system_tokens + user_tokens;

        debug!(
            "Token count - System: {}, User: {}, Total: {}",
            system_tokens, user_tokens, total_tokens
        );

        // Generate cache key
        let cache_key = format!(
            "{:x}",
            md5::compute(format!("{}{}", system_prompt, user_prompt))
        );
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
                self.manifest = Some(manifest_mut);
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
                    .build()?,
            ),
            ChatCompletionRequestMessage::User(
                ChatCompletionRequestUserMessageArgs::default()
                    .content(user_prompt)
                    .build()?,
            ),
        ];

        // Create request
        let request = CreateChatCompletionRequestArgs::default()
            .model("gpt-4-turbo-preview")
            .messages(messages)
            .temperature(0.7)
            .max_tokens(4000u32)
            .build()?;

        // Make API call
        let response = self.client.chat().create(request).await?;

        // Extract content
        let content = response
            .choices
            .first()
            .and_then(|choice| choice.message.content.clone())
            .context("No response content")?;

        // Cache response
        if self.use_cache {
            fs::create_dir_all(&self.cache_dir)?;
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

    async fn generate_image(&mut self, prompt: &str, filename: &str) -> Result<()> {
        info!("Generating image: {}", filename);

        let request = CreateImageRequestArgs::default()
            .prompt(prompt)
            .n(1)
            .size(ImageSize::S1024x1024)
            .build()?;

        let response = self.client.images().create(request).await?;

        if let Some(image_data) = response.data.first() {
            match &**image_data {
                Image::Url {
                    url,
                    revised_prompt: _,
                } => {
                    // Download image from URL
                    let image_bytes = reqwest::get(url).await?.bytes().await?;
                    let path = PathBuf::from("assets/sprites").join(filename);
                    self.write_file(&path, &image_bytes).await?;
                }
                Image::B64Json {
                    b64_json: _,
                    revised_prompt: _,
                } => {
                    warn!("Received base64 response but requested URL format");
                }
            }
        } else {
            warn!("No image in response");
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

        let pb = ProgressBar::new(8); // Increased for style guide steps
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} {msg}")
                .unwrap()
                .progress_chars("#>-"),
        );

        // Generate style guide FIRST
        pb.set_message("Generating style guide...");
        self.generate_style_guide().await?;
        pb.inc(1);

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

        pb.set_message("Generating UI assets...");
        self.generate_ui_assets().await?;
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

    pub async fn generate_style_guide(&mut self) -> Result<()> {
        info!("🎨 Generating visual style guide...");

        let config = self.config.as_ref().context("Config not loaded")?;
        let game_genre = config.game.genre.clone();
        let game_title = config.game.title.clone();
        let game_theme = config.game.theme.clone();
        let graphics_perspective = config.graphics.perspective.clone();
        let sprite_size = config.graphics.sprite_size;

        // Generate color palette
        let palette_prompt = format!(
            "Generate a cohesive color palette for a {} game called '{}'. \
            The game has a {} mood and {} visual style. \
            Output a JSON object with: \
            - primary_colors: 3-4 main colors with hex values and names \
            - secondary_colors: 2-3 accent colors \
            - ui_colors: background, text, highlight colors \
            - semantic_colors: health (red), mana (blue), poison (green), etc. \
            Each color should have 'hex', 'name', and 'usage' fields.",
            game_genre,
            game_title,
            if game_theme.is_empty() {
                &game_genre
            } else {
                &game_theme
            },
            graphics_perspective
        );

        let palette_json = self
            .generate_with_ai(
                "You are a game art director specializing in color theory and pixel art palettes.",
                &palette_prompt,
            )
            .await?;

        self.write_file("assets/style/color-palette.json", palette_json.as_bytes())
            .await?;

        // Generate style rules document
        let style_rules_prompt = format!(
            "Create comprehensive visual style guidelines for '{}'. \
            Include: \
            1. Pixel art specifications ({}x{} sprites) \
            2. Outline style (black, colored, or none) \
            3. Shading technique (flat, simple gradient, dithered) \
            4. Animation principles (frame counts, timing) \
            5. UI design patterns \
            6. Environmental art rules \
            Format as a markdown document.",
            game_title, sprite_size, sprite_size
        );

        let style_rules = self
            .generate_with_ai(
                "You are a pixel art expert and game art director.",
                &style_rules_prompt,
            )
            .await?;

        self.write_file("assets/style/style-rules.md", style_rules.as_bytes())
            .await?;

        // Generate reference sprite sheet prompt
        let reference_prompt = format!(
            "Design a style guide reference sprite sheet for '{}' that includes: \
            1. Color swatches from the palette \
            2. Example character in idle pose ({}x{} pixels) \
            3. Sample terrain tiles (grass, stone, water) \
            4. UI element examples (button, health bar, dialog box) \
            5. Lighting/shadow examples \
            6. Effect samples (fire, magic sparkle) \
            All sprites should be {}x{} pixels. \
            Describe each element in detail for DALL-E 3 generation.",
            game_title, sprite_size, sprite_size, sprite_size, sprite_size
        );

        let reference_description = self
            .generate_with_ai(
                "You are a pixel art designer creating visual references.",
                &reference_prompt,
            )
            .await?;

        // Generate the actual reference image
        match self
            .generate_image(&reference_description, "style-reference.png")
            .await
        {
            Ok(_) => info!("  ✓ Generated style reference image"),
            Err(e) => warn!("  ⚠ Failed to generate style reference: {}", e),
        }

        Ok(())
    }

    pub async fn generate_ui_assets(&mut self) -> Result<()> {
        info!("🖼️  Generating UI assets...");

        let config = self.config.as_ref().context("Config not loaded")?;

        // Load color palette
        let palette_path = PathBuf::from("assets/style/color-palette.json");
        let palette_json = if palette_path.exists() {
            fs::read_to_string(&palette_path)?
        } else {
            warn!("Color palette not found, using defaults");
            serde_json::json!({
                "ui_colors": {
                    "background": "#1a1c2c",
                    "text": "#f4f4f4",
                    "highlight": "#41a6f6"
                }
            })
            .to_string()
        };

        // Generate UI element specifications
        let ui_prompt = format!(
            "Design UI elements for '{}' using this color palette: {}. \
            Create specifications for: \
            1. Dialog boxes with 9-slice borders \
            2. Button states (normal, hover, pressed) \
            3. Health and mana bars \
            4. Inventory slots \
            5. Menu backgrounds \
            Use pixel art style matching {} perspective. \
            Output detailed descriptions for image generation.",
            config.game.title, palette_json, config.graphics.perspective
        );

        let ui_specs = self
            .generate_with_ai(
                "You are a UI/UX designer specializing in pixel art game interfaces.",
                &ui_prompt,
            )
            .await?;

        // Generate UI sprite sheet
        match self.generate_image(&ui_specs, "ui-elements.png").await {
            Ok(_) => info!("  ✓ Generated UI elements"),
            Err(e) => warn!("  ⚠ Failed to generate UI elements: {}", e),
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

        let cargo_content = self
            .generate_with_ai("You are a Rust and Bevy expert.", &cargo_prompt)
            .await?;

        self.write_file("Cargo.toml", cargo_content.as_bytes())
            .await?;

        // Generate main.rs
        let main_prompt = format!(
            "Generate a main.rs for a Bevy game with: \
            - Title: '{}' \
            - Window size based on {}px tiles \
            - States: Menu, Playing, Paused \
            - Plugins: DefaultPlugins, TilemapPlugin, YoleckPlugin, WorldInspectorPlugin \
            - Basic camera setup for {} view \
            Output only Rust code, no explanations.",
            game_title, tile_size, perspective
        );

        let main_content = self
            .generate_with_ai("You are a Rust and Bevy game engine expert.", &main_prompt)
            .await?;

        self.write_file("src/main.rs", main_content.as_bytes())
            .await?;

        Ok(())
    }

    pub async fn generate_components(&mut self) -> Result<()> {
        info!("🧩 Generating ECS components...");

        let components_code = self.templates.render_components()?;
        self.write_file("src/components.rs", components_code.as_bytes())
            .await?;

        Ok(())
    }

    pub async fn generate_systems(&mut self) -> Result<()> {
        info!("⚙️  Generating game systems...");

        let systems = vec![
            (
                "movement",
                "Basic movement system with velocity and position",
            ),
            ("combat", "Turn-based combat with damage calculation"),
            (
                "taming",
                "Monster taming with success rates and party management",
            ),
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

            let code = self
                .generate_with_ai("You are a Bevy ECS expert.", &prompt)
                .await?;

            self.write_file(format!("src/systems/{}.rs", name), code.as_bytes())
                .await?;
        }

        Ok(())
    }

    pub async fn generate_levels(&mut self) -> Result<()> {
        info!("🗺️  Generating level files...");

        let config = self.config.as_ref().context("Config not loaded")?;
        let tile_size = config.graphics.tile_size;
        let zones = config.environments.outdoor_zones.clone();

        // Create levels directory and Yoleck index
        fs::create_dir_all("assets/levels")?;

        let mut level_files = Vec::new();

        for zone in &zones {
            let zone_name_slug = zone.name.to_lowercase().replace(' ', "_");

            let level_prompt = format!(
                "Generate a Yoleck (.yol) format level file for '{}' zone. \
                Zone type: {}, biome: {}, description: {}. \
                Map size: 50x50 tiles, tile size: {}. \
                Include proper tilemap layout, player spawn, enemies, treasures, and zone transitions. \
                Output the complete JSON array following Yoleck format: [metadata, {{}}, entities]",
                zone.name,
                zone.zone_type,
                zone.biome,
                zone.description.as_str(),
                tile_size
            );

            let level_content = self
                .generate_with_ai(
                    "You are a level designer creating Bevy Yoleck format levels.",
                    &level_prompt,
                )
                .await?;

            let level_filename = format!("{}.yol", zone_name_slug);
            let level_path = format!("assets/levels/{}", level_filename);
            self.write_file(&level_path, level_content.as_bytes())
                .await?;

            level_files.push(serde_json::json!({
                "filename": level_filename
            }));
        }

        // Generate Yoleck index file
        let index_content = serde_json::json!([
            {
                "format_version": 1
            },
            level_files
        ]);

        let index_json = serde_json::to_string_pretty(&index_content)?;
        self.write_file("assets/levels/index.yoli", index_json.as_bytes())
            .await?;

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
        info!("🎵 Generating procedural audio specifications...");

        let config = self.config.as_ref().context("Config not loaded")?;
        let game_title = config.game.title.clone();
        let game_genre = config.game.genre.clone();

        // Generate comprehensive audio specifications
        let audio_prompt = format!(
            "Generate complete procedural audio specifications for '{}', a {} game. \
            Include specifications for: \
            1. Background music tracks (main theme, zone themes, battle, boss, victory, game over) \
            2. Sound effects (UI, player actions, combat, environmental) \
            3. Ambient sounds for each biome \
            Use retro JRPG style with chiptune aesthetics. \
            Output as JSON with synthesis parameters for Web Audio API.",
            game_title, game_genre
        );

        let audio_specs = self.generate_with_ai(
            "You are an audio designer specializing in procedural game audio and Web Audio API.",
            &audio_prompt
        ).await?;

        self.write_file("assets/audio/audio_specs.json", audio_specs.as_bytes())
            .await?;

        // Generate Web Audio implementation script
        let implementation_prompt = format!(
            "Generate a JavaScript module that implements the audio specifications for '{}' using Web Audio API. \
            The script should: \
            1. Load the audio_specs.json \
            2. Create synthesis functions for each sound \
            3. Provide play() methods for music and SFX \
            4. Handle looping for background music \
            5. Include volume and mixing controls \
            Make it modular and easy to integrate with the game.",
            game_title
        );

        let audio_script = self.generate_with_ai(
            "You are a JavaScript developer expert in Web Audio API and game audio programming.",
            &implementation_prompt
        ).await?;

        self.write_file("assets/audio/game-audio.js", audio_script.as_bytes())
            .await?;

        // Generate audio documentation
        let doc_content = format!(
            "# Audio System Documentation for {}\n\n\
            ## Overview\n\
            This game uses procedural audio generation via Web Audio API.\n\n\
            ## Audio Files\n\
            - `audio_specs.json`: Complete audio specifications\n\
            - `game-audio.js`: Web Audio API implementation\n\n\
            ## Integration\n\
            ```javascript\n\
            import {{ GameAudio }} from './assets/audio/game-audio.js';\n\
            const audio = new GameAudio();\n\
            await audio.init();\n\
            audio.playMusic('main_theme');\n\
            audio.playSFX('menu_select');\n\
            ```\n\n\
            ## Customization\n\
            Edit `audio_specs.json` to modify any sound parameters.",
            game_title
        );

        self.write_file("assets/audio/README.md", doc_content.as_bytes())
            .await?;

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
            serde_json::to_string_pretty(&summary)?,
        )?;

        info!("📊 Summary written to GENERATION_SUMMARY.json");

        Ok(())
    }

    pub async fn test(&mut self) -> Result<()> {
        let response = self
            .generate_with_ai(
                "You are a helpful assistant.",
                "Say 'Hello from Rust AI generator!'",
            )
            .await?;

        info!("Test response: {}", response);

        Ok(())
    }

    pub async fn clean(&self) -> Result<()> {
        warn!("Clean not implemented yet");
        Ok(())
    }
}
