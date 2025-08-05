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

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use tempfile::TempDir;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_default_genre() {
        assert_eq!(default_genre(), "JRPG");
    }

    #[test]
    fn test_game_info_serialization() {
        let game_info = GameInfo {
            title: "Test Game".to_string(),
            codename: "test_game".to_string(),
            version: "1.0.0".to_string(),
            genre: "RPG".to_string(),
            theme: "Fantasy".to_string(),
            setting: "Medieval".to_string(),
        };

        let yaml = serde_yaml::to_string(&game_info).unwrap();
        let deserialized: GameInfo = serde_yaml::from_str(&yaml).unwrap();
        
        assert_eq!(game_info.title, deserialized.title);
        assert_eq!(game_info.codename, deserialized.codename);
        assert_eq!(game_info.version, deserialized.version);
        assert_eq!(game_info.genre, deserialized.genre);
        assert_eq!(game_info.theme, deserialized.theme);
        assert_eq!(game_info.setting, deserialized.setting);
    }

    #[test]
    fn test_game_info_with_defaults() {
        let yaml = r#"
title: "Test Game"
codename: "test_game"
version: "1.0.0"
"#;
        let game_info: GameInfo = serde_yaml::from_str(yaml).unwrap();
        
        assert_eq!(game_info.genre, "JRPG"); // Should use default
        assert_eq!(game_info.theme, ""); // Should be empty
        assert_eq!(game_info.setting, ""); // Should be empty
    }

    #[test]
    fn test_zone_serialization() {
        let zone = Zone {
            name: "Forest of Trials".to_string(),
            zone_type: "outdoor".to_string(),
            description: "A mysterious forest".to_string(),
            biome: "forest".to_string(),
            difficulty: 5,
            monsters: vec!["Goblin".to_string(), "Wolf".to_string()],
            features: vec!["river".to_string(), "cave".to_string()],
        };

        let yaml = serde_yaml::to_string(&zone).unwrap();
        let deserialized: Zone = serde_yaml::from_str(&yaml).unwrap();
        
        assert_eq!(zone.name, deserialized.name);
        assert_eq!(zone.zone_type, deserialized.zone_type);
        assert_eq!(zone.description, deserialized.description);
        assert_eq!(zone.biome, deserialized.biome);
        assert_eq!(zone.difficulty, deserialized.difficulty);
        assert_eq!(zone.monsters, deserialized.monsters);
        assert_eq!(zone.features, deserialized.features);
    }

    #[test]
    fn test_zone_with_type_field() {
        // Test that 'type' field is properly renamed to zone_type
        let yaml = r#"
name: "Test Zone"
type: "dungeon"
"#;
        let zone: Zone = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(zone.zone_type, "dungeon");
    }

    #[test]
    fn test_full_config_serialization() {
        let config = create_test_config();
        
        let yaml = serde_yaml::to_string(&config).unwrap();
        let deserialized: GameConfig = serde_yaml::from_str(&yaml).unwrap();
        
        assert_eq!(config.game.title, deserialized.game.title);
        assert_eq!(config.hero.name, deserialized.hero.name);
        assert_eq!(config.environments.outdoor_zones.len(), 
                   deserialized.environments.outdoor_zones.len());
        assert_eq!(config.graphics.tile_size, deserialized.graphics.tile_size);
        assert_eq!(config.audio.music_style, deserialized.audio.music_style);
    }

    #[tokio::test]
    async fn test_load_config_from_file() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("test-config.yaml");
        
        let yaml_content = r#"
game:
  title: "Test Game"
  codename: "test"
  version: "1.0.0"
hero:
  name: "Hero"
  description: "A brave hero"
environments:
  outdoor_zones:
    - name: "Starting Zone"
      type: "outdoor"
  map_generation:
    mapgen_algorithms:
      overworld: "cellular_automata"
      dungeon: "rooms_and_corridors"
generation_rules:
  adjectives: ["brave", "mighty"]
  nouns: ["warrior", "mage"]
build:
  dependencies: {}
graphics:
  tile_size: 16
  sprite_size: 32
  perspective: "top_down"
audio:
  music_style: "chiptune"
"#;
        
        let mut file = File::create(&config_path).unwrap();
        file.write_all(yaml_content.as_bytes()).unwrap();
        
        let config = GameConfig::load(&config_path).await.unwrap();
        
        assert_eq!(config.game.title, "Test Game");
        assert_eq!(config.hero.name, "Hero");
        assert_eq!(config.environments.outdoor_zones.len(), 1);
        assert_eq!(config.graphics.tile_size, 16);
        assert_eq!(config.audio.music_style, "chiptune");
    }

    #[tokio::test]
    async fn test_load_config_file_not_found() {
        let result = GameConfig::load("non_existent_file.yaml").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_load_config_invalid_yaml() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("invalid.yaml");
        
        let mut file = File::create(&config_path).unwrap();
        file.write_all(b"invalid: yaml: content: [").unwrap();
        
        let result = GameConfig::load(&config_path).await;
        assert!(result.is_err());
    }

    // Helper function to create a test config
    fn create_test_config() -> GameConfig {
        GameConfig {
            game: GameInfo {
                title: "Test Game".to_string(),
                codename: "test_game".to_string(),
                version: "1.0.0".to_string(),
                genre: "JRPG".to_string(),
                theme: "Fantasy".to_string(),
                setting: "Medieval".to_string(),
            },
            hero: HeroInfo {
                name: "Test Hero".to_string(),
                description: "A test hero".to_string(),
                class: "Warrior".to_string(),
                abilities: vec!["Slash".to_string(), "Block".to_string()],
            },
            environments: Environments {
                outdoor_zones: vec![
                    Zone {
                        name: "Test Zone".to_string(),
                        zone_type: "outdoor".to_string(),
                        description: "A test zone".to_string(),
                        biome: "forest".to_string(),
                        difficulty: 1,
                        monsters: vec!["Slime".to_string()],
                        features: vec!["tree".to_string()],
                    }
                ],
                dungeons: vec![],
                special_areas: vec![],
                map_generation: MapGeneration {
                    mapgen_algorithms: MapgenAlgorithms {
                        overworld: "cellular_automata".to_string(),
                        dungeon: "rooms_and_corridors".to_string(),
                        cave: "drunken_walk".to_string(),
                        special: "maze".to_string(),
                    }
                },
            },
            generation_rules: GenerationRules {
                adjectives: vec!["brave".to_string()],
                nouns: vec!["warrior".to_string()],
                verbs: vec!["attack".to_string()],
                abilities: vec!["slash".to_string()],
            },
            build: BuildConfig {
                dependencies: serde_yaml::Value::Null,
            },
            graphics: GraphicsConfig {
                tile_size: 16,
                sprite_size: 32,
                perspective: "top_down".to_string(),
            },
            audio: AudioConfig {
                music_style: "chiptune".to_string(),
            },
        }
    }
}