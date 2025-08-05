use ai_game_generator::{CascadeExecutor, PromptCascade};
use anyhow::Result;
use std::env;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<()> {
    // Only run cascade in release builds or when explicitly requested
    let should_generate = env::var("ECHOES_GENERATE").is_ok() || 
                         env::var("PROFILE").map(|p| p == "release").unwrap_or(false);
    
    if !should_generate {
        println!("cargo:warning=Skipping AI generation. Set ECHOES_GENERATE=1 to enable.");
        return Ok(());
    }

    println!("cargo:warning=Running AI game generation cascade...");
    
    // Get paths
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    let metaprompt_path = manifest_dir.join("metaprompts/root.toml");
    let output_dir = manifest_dir.clone();
    let cache_dir = manifest_dir.join(".cascade-cache");
    
    // Check if we're in dry-run mode
    let dry_run = env::var("ECHOES_DRY_RUN").is_ok();
    
    // Load the cascade
    let cascade_content = std::fs::read_to_string(&metaprompt_path)?;
    let cascade: PromptCascade = toml::from_str(&cascade_content)?;
    
    // Create executor
    let mut executor = CascadeExecutor::new(cache_dir, dry_run)?;
    
    // Execute the cascade
    executor.execute_cascade(&cascade, &output_dir).await?;
    
    println!("cargo:warning=AI generation cascade completed successfully!");
    
    // Tell Cargo to re-run this build script if the cascade file changes
    println!("cargo:rerun-if-changed=metaprompts/root.toml");
    println!("cargo:rerun-if-env-changed=ECHOES_GENERATE");
    println!("cargo:rerun-if-env-changed=ECHOES_DRY_RUN");
    
    Ok(())
}