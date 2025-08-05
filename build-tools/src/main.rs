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

use anyhow::Result;
use clap::{Parser, Subcommand};
use tracing::info;
use tracing_subscriber::EnvFilter;

mod config;
mod generator;
mod templates;
mod git_tracker;

use generator::AIGameGenerator;

#[derive(Parser)]
#[command(name = "generator-debug")]
#[command(about = "Debug tool for AI game generator - for testing individual components", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Use cache for AI responses
    #[arg(long, default_value_t = true)]
    cache: bool,

    /// Dry run (don't write files)
    #[arg(long)]
    dry_run: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Test individual generator components
    Component {
        /// Component type to generate
        #[arg(value_enum)]
        component: ComponentType,
    },
    /// Run a simple test
    Test,
    /// Clean generated files
    Clean,
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum ComponentType {
    Core,
    Components,
    Systems,
    Levels,
    Sprites,
    Audio,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env()
            .add_directive("ai_game_generator=info".parse()?)
            .add_directive("async_openai=warn".parse()?))
        .init();

    let cli = Cli::parse();

    // Create generator
    let mut generator = AIGameGenerator::new()
        .with_use_cache(cli.cache)
        .with_dry_run(cli.dry_run);

    generator.initialize().await?;

    match cli.command {
        Commands::Component { component } => {
            info!("🧩 Testing {:?} component generation", component);
            match component {
                ComponentType::Core => generator.generate_core().await?,
                ComponentType::Components => generator.generate_components().await?,
                ComponentType::Systems => generator.generate_systems().await?,
                ComponentType::Levels => generator.generate_levels().await?,
                ComponentType::Sprites => generator.generate_sprites().await?,
                ComponentType::Audio => generator.generate_audio().await?,
            }
        }
        Commands::Test => {
            info!("🧪 Running generator test");
            generator.test().await?;
        }
        Commands::Clean => {
            info!("🧹 Cleaning generated files");
            generator.clean().await?;
        }
    }

    Ok(())
}
