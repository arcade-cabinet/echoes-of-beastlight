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

#[cfg(feature = "studio")]
use bevy::prelude::*;

mod config;
mod generator;
mod git_tracker;
mod studio;
mod templates;

#[cfg(feature = "studio")]
use studio::GameGeneratorStudioPlugin;

#[cfg(feature = "studio")]
fn main() {
    // Set up logging
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info,studio=debug");
    }

    App::new().add_plugins(GameGeneratorStudioPlugin).run();
}

#[cfg(not(feature = "studio"))]
fn main() {
    eprintln!("The studio binary requires the 'studio' feature to be enabled.");
    eprintln!("Try: cargo run --bin studio --features studio");
    std::process::exit(1);
}
