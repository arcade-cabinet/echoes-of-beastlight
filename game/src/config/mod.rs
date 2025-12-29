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
    pub name: String,
    pub version: String,
    pub genre: String,
    pub description: String,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            metadata: GameMetadata {
                name: "Echoes of Beastlight".to_string(),
                version: "0.1.0".to_string(),
                genre: "JRPG with Monster Taming".to_string(),
                description: "A mystical journey through a world where light itself has become corrupted".to_string(),
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
