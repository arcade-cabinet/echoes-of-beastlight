#[cfg(feature = "studio")]
use bevy::prelude::*;

mod studio;
mod config;
mod generator;
mod templates;
mod git_tracker;

#[cfg(feature = "studio")]
use studio::GameGeneratorStudioPlugin;

#[cfg(feature = "studio")]
fn main() {
    // Set up logging
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info,studio=debug");
    }
    
    App::new()
        .add_plugins(GameGeneratorStudioPlugin)
        .run();
}

#[cfg(not(feature = "studio"))]
fn main() {
    eprintln!("The studio binary requires the 'studio' feature to be enabled.");
    eprintln!("Try: cargo run --bin studio --features studio");
    std::process::exit(1);
}