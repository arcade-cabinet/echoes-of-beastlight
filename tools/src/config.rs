use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct GameConfig {
    pub game: GameInfo,
    pub hero: HeroInfo,
    pub environments: Environments,
    pub generation_rules: GenerationRules,
    pub build: BuildConfig,
    pub graphics: GraphicsConfig,
    pub audio: AudioConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameInfo {
    pub title: String,
    pub codename: String,
    pub version: String,
    #[serde(default = "default_genre")]
    pub genre: String,
    #[serde(default)]
    pub theme: String,
    #[serde(default)]
    pub setting: String,
}

fn default_genre() -> String {
    "JRPG".to_string()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeroInfo {
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub class: String,
    #[serde(default)]
    pub abilities: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Environments {
    pub outdoor_zones: Vec<Zone>,
    #[serde(default)]
    pub dungeons: Vec<Zone>,
    #[serde(default)]
    pub special_areas: Vec<Zone>,
    pub map_generation: MapGeneration,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Zone {
    pub name: String,
    #[serde(rename = "type", default)]
    pub zone_type: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub biome: String,
    #[serde(default)]
    pub difficulty: u32,
    #[serde(default)]
    pub monsters: Vec<String>,
    #[serde(default)]
    pub features: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapGeneration {
    pub mapgen_algorithms: MapgenAlgorithms,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapgenAlgorithms {
    pub overworld: String,
    pub dungeon: String,
    #[serde(default)]
    pub cave: String,
    #[serde(default)]
    pub special: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerationRules {
    pub adjectives: Vec<String>,
    pub nouns: Vec<String>,
    #[serde(default)]
    pub verbs: Vec<String>,
    #[serde(default)]
    pub abilities: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BuildConfig {
    pub dependencies: serde_yaml::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GraphicsConfig {
    pub tile_size: u32,
    pub sprite_size: u32,
    pub perspective: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AudioConfig {
    pub music_style: String,
}

impl GameConfig {
    pub async fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let config = serde_yaml::from_str(&content)?;
        Ok(config)
    }
}