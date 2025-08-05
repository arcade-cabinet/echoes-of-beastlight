use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Only generate in release builds or when explicitly requested
    let should_generate = env::var("ECHOES_GENERATE").is_ok() || 
                         env::var("PROFILE").map(|p| p == "release").unwrap_or(false);
    
    if !should_generate {
        return;
    }

    println!("cargo:warning=Generating game assets...");
    
    // Create asset directories if they don't exist
    let asset_dirs = vec![
        "assets/sprites/characters",
        "assets/sprites/monsters", 
        "assets/sprites/tiles",
        "assets/sprites/ui",
        "assets/audio/music",
        "assets/audio/sfx",
        "assets/levels",
        "assets/data",
    ];
    
    for dir in &asset_dirs {
        fs::create_dir_all(dir).unwrap_or_else(|e| {
            println!("cargo:warning=Failed to create {}: {}", dir, e);
        });
    }
    
    // Create marker files for the generator to know what needs generation
    // The actual generation will be done by targeted TOML prompts in each directory
    let markers = vec![
        ("assets/sprites/characters/.generate", "hero_sprites"),
        ("assets/sprites/monsters/.generate", "monster_sprites"),
        ("assets/sprites/tiles/.generate", "tilemap_sprites"),
        ("assets/sprites/ui/.generate", "ui_elements"),
        ("assets/audio/music/.generate", "music_tracks"),
        ("assets/audio/sfx/.generate", "sound_effects"),
        ("assets/levels/.generate", "level_layouts"),
        ("assets/data/.generate", "game_data"),
    ];
    
    for (path, content) in &markers {
        if !Path::new(path).exists() {
            fs::write(path, content).unwrap_or_else(|e| {
                println!("cargo:warning=Failed to create marker {}: {}", path, e);
            });
        }
    }
    
    // Tell Cargo to re-run if generation is requested
    println!("cargo:rerun-if-env-changed=ECHOES_GENERATE");
}