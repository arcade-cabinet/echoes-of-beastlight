use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use ai_game_generator::{PromptCascade, CascadeExecutor};

#[derive(Parser, Debug)]
#[command(name = "cascade")]
#[command(about = "Meta-prompt cascade executor for AI game generation")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
    
    /// Cache directory
    #[arg(long, default_value = ".cascade-cache")]
    cache_dir: PathBuf,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Execute a meta-prompt cascade
    Execute {
        /// Path to the cascade TOML file
        cascade_file: PathBuf,
        
        /// Output directory for generated files
        #[arg(short, long, default_value = ".")]
        output_dir: PathBuf,
        
        /// Dry run (don't actually call OpenAI)
        #[arg(long)]
        dry_run: bool,
    },
    
    /// Validate a cascade file
    Validate {
        /// Path to the cascade TOML file
        cascade_file: PathBuf,
    },
    
    /// Visualize cascade as a graph
    Visualize {
        /// Path to the cascade TOML file
        cascade_file: PathBuf,
        
        /// Output file for the graph (DOT format)
        #[arg(short, long, default_value = "cascade.dot")]
        output: PathBuf,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Set up logging
    let level = if cli.verbose { Level::DEBUG } else { Level::INFO };
    let subscriber = FmtSubscriber::builder()
        .with_max_level(level)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;
    
    match cli.command {
        Commands::Execute { cascade_file, output_dir, dry_run } => {
            execute_cascade(cascade_file, output_dir, cli.cache_dir, dry_run).await?;
        }
        Commands::Validate { cascade_file } => {
            validate_cascade(cascade_file)?;
        }
        Commands::Visualize { cascade_file, output } => {
            visualize_cascade(cascade_file, output)?;
        }
    }
    
    Ok(())
}

async fn execute_cascade(
    cascade_file: PathBuf,
    output_dir: PathBuf,
    cache_dir: PathBuf,
    dry_run: bool,
) -> Result<()> {
    info!("Loading cascade from {:?}", cascade_file);
    
    // Load cascade
    let content = std::fs::read_to_string(&cascade_file)?;
    let cascade: PromptCascade = toml::from_str(&content)?;
    
    info!("Loaded cascade: {}", cascade.name);
    info!("Version: {}", cascade.version);
    info!("Prompts: {}", cascade.prompts.len());
    
    // Create executor
    let mut executor = CascadeExecutor::new(cache_dir, dry_run)?;
    
    // Execute cascade
    executor.execute_cascade(&cascade, &output_dir).await?;
    
    info!("Cascade execution complete!");
    
    Ok(())
}

fn validate_cascade(cascade_file: PathBuf) -> Result<()> {
    info!("Validating cascade from {:?}", cascade_file);
    
    // Load cascade
    let content = std::fs::read_to_string(&cascade_file)?;
    let cascade: PromptCascade = toml::from_str(&content)?;
    
    // Validate structure
    info!("Cascade: {}", cascade.name);
    info!("Version: {}", cascade.version);
    info!("Root prompt: {}", cascade.root_prompt);
    info!("Total prompts: {}", cascade.prompts.len());
    
    // Check root exists
    if !cascade.prompts.contains_key(&cascade.root_prompt) {
        anyhow::bail!("Root prompt '{}' not found in prompts", cascade.root_prompt);
    }
    
    // Build DAG to check for cycles
    let dag = cascade.build_dag()?;
    info!("DAG has {} nodes and {} edges", dag.node_count(), dag.edge_count());
    
    // Get execution order
    let order = cascade.get_execution_order()?;
    info!("Execution order: {:?}", order);
    
    // Validate each prompt
    for (id, prompt) in &cascade.prompts {
        // Check children exist
        for child in &prompt.children {
            if !cascade.prompts.contains_key(child) {
                anyhow::bail!("Prompt '{}' references unknown child '{}'", id, child);
            }
        }
        
        // Check dependencies exist
        for dep in &prompt.depends_on {
            if !cascade.prompts.contains_key(dep) {
                anyhow::bail!("Prompt '{}' references unknown dependency '{}'", id, dep);
            }
        }
        
        // Check inheritance
        if let Some(parent) = &prompt.inherits {
            if !cascade.prompts.contains_key(parent) {
                anyhow::bail!("Prompt '{}' inherits from unknown prompt '{}'", id, parent);
            }
        }
    }
    
    info!("✅ Cascade is valid!");
    
    Ok(())
}

fn visualize_cascade(cascade_file: PathBuf, output: PathBuf) -> Result<()> {
    use petgraph::dot::{Dot, Config};
    
    info!("Visualizing cascade from {:?}", cascade_file);
    
    // Load cascade
    let content = std::fs::read_to_string(&cascade_file)?;
    let cascade: PromptCascade = toml::from_str(&content)?;
    
    // Build DAG
    let dag = cascade.build_dag()?;
    
    // Generate DOT format
    let dot = Dot::with_config(&dag, &[Config::EdgeNoLabel]);
    let dot_string = format!("{:?}", dot);
    
    // Write to file
    std::fs::write(&output, dot_string)?;
    
    info!("Wrote graph to {:?}", output);
    info!("To render: dot -Tpng {} -o cascade.png", output.display());
    
    Ok(())
}