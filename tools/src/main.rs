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
#[command(name = "ai-gen")]
#[command(about = "AI-powered game generator for Echoes of Beastlight", long_about = None)]
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
    /// Generate the complete game
    Generate {
        /// Force regeneration even if files exist
        #[arg(long)]
        force: bool,
    },
    /// Generate only specific components
    Component {
        /// Component type to generate
        #[arg(value_enum)]
        component: ComponentType,
    },
    /// Test the generator
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
        .with_cache(cli.cache)
        .with_dry_run(cli.dry_run);
    
    generator.initialize().await?;
    
    match cli.command {
        Commands::Generate { force } => {
            info!("🎮 Starting AI Game Generation");
            generator.generate_game(force).await?;
        }
        Commands::Component { component } => {
            info!("🧩 Generating {:?} component", component);
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