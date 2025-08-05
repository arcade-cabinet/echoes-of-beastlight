mod studio;
mod config;
mod generator;
mod templates;
mod git_tracker;

use bevy::prelude::*;
use studio::GameGeneratorStudioPlugin;

fn main() {
    // Set up logging
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info,studio=debug");
    }
    
    App::new()
        .add_plugins(GameGeneratorStudioPlugin)
        .run();
}