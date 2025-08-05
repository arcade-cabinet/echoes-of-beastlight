pub mod style;
pub mod game;
pub mod generation;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Core game configuration that drives all generation and gameplay
#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct GameConfig {
    pub metadata: GameMetadata,
    pub style: style::StyleConfig,
    pub gameplay: game::GameplayConfig,
    pub generation: generation::GenerationConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameMetadata {
    pub name: &'static str,
    pub version: &'static str,
    pub genre: &'static str,
    pub description: &'static str,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            metadata: GameMetadata {
                name: "Echoes of Beastlight",
                version: "0.1.0",
                genre: "JRPG with Monster Taming",
                description: "A mystical journey through a world where light itself has become corrupted",
            },
            style: style::StyleConfig::default(),
            gameplay: game::GameplayConfig::default(),
            generation: generation::GenerationConfig::default(),
        }
    }
}

pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameConfig::default());
    }
}