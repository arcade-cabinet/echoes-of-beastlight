use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Core gameplay configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameplayConfig {
    pub hero: HeroConfig,
    pub monsters: MonsterConfig,
    pub world: WorldConfig,
    pub combat: CombatConfig,
    pub progression: ProgressionConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeroConfig {
    pub name: String,
    pub class: String,
    pub base_stats: BaseStats,
    pub starting_abilities: Vec<String>,
    pub max_party_size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseStats {
    pub health: i32,
    pub mana: i32,
    pub attack: i32,
    pub defense: i32,
    pub speed: i32,
    pub spirit: i32, // For taming effectiveness
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonsterConfig {
    pub taming_enabled: bool,
    pub max_active_monsters: usize,
    pub evolution_enabled: bool,
    pub type_effectiveness: bool,
    pub rarity_tiers: Vec<RarityTier>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RarityTier {
    pub name: String,
    pub color: Color,
    pub spawn_weight: f32,
    pub stat_multiplier: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldConfig {
    pub seed_based: bool,
    pub zones: Vec<ZoneDefinition>,
    pub day_night_cycle: bool,
    pub corruption_spreading: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneDefinition {
    pub name: String,
    pub biome: String,
    pub corruption_level: f32,
    pub level_range: (u32, u32),
    pub key_features: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatConfig {
    pub system: String,
    pub action_points: bool,
    pub elemental_types: Vec<ElementType>,
    pub status_effects: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementType {
    pub name: String,
    pub color: Color,
    pub strong_against: Vec<String>,
    pub weak_against: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressionConfig {
    pub level_cap: u32,
    pub experience_curve: String,
    pub skill_trees: bool,
    pub equipment_slots: Vec<String>,
}

impl Default for GameplayConfig {
    fn default() -> Self {
        Self {
            hero: HeroConfig {
                name: "Luna".to_string(),
                class: "Lightkeeper".to_string(),
                base_stats: BaseStats {
                    health: 100,
                    mana: 50,
                    attack: 15,
                    defense: 10,
                    speed: 12,
                    spirit: 20,
                },
                starting_abilities: vec![
                    "Light Burst".to_string(),
                    "Tame".to_string(),
                    "Heal".to_string(),
                ],
                max_party_size: 6,
            },
            monsters: MonsterConfig {
                taming_enabled: true,
                max_active_monsters: 3,
                evolution_enabled: true,
                type_effectiveness: true,
                rarity_tiers: vec![
                    RarityTier {
                        name: "Common".to_string(),
                        color: Color::Srgba(Srgba::hex("#9e9e9e").unwrap()),
                        spawn_weight: 0.6,
                        stat_multiplier: 1.0,
                    },
                    RarityTier {
                        name: "Uncommon".to_string(),
                        color: Color::Srgba(Srgba::hex("#4caf50").unwrap()),
                        spawn_weight: 0.25,
                        stat_multiplier: 1.2,
                    },
                    RarityTier {
                        name: "Rare".to_string(),
                        color: Color::Srgba(Srgba::hex("#2196f3").unwrap()),
                        spawn_weight: 0.12,
                        stat_multiplier: 1.5,
                    },
                    RarityTier {
                        name: "Epic".to_string(),
                        color: Color::Srgba(Srgba::hex("#9c27b0").unwrap()),
                        spawn_weight: 0.03,
                        stat_multiplier: 2.0,
                    },
                ],
            },
            world: WorldConfig {
                seed_based: true,
                zones: vec![
                    ZoneDefinition {
                        name: "Glintrock Village".to_string(),
                        biome: "corrupted_forest".to_string(),
                        corruption_level: 0.2,
                        level_range: (1, 5),
                        key_features: vec![
                            "starting_town".to_string(),
                            "basic_shops".to_string(),
                            "taming_tutorial".to_string(),
                        ],
                    },
                    ZoneDefinition {
                        name: "Whispering Woods".to_string(),
                        biome: "dark_forest".to_string(),
                        corruption_level: 0.4,
                        level_range: (5, 10),
                        key_features: vec![
                            "first_dungeon".to_string(),
                            "corrupted_shrine".to_string(),
                        ],
                    },
                    ZoneDefinition {
                        name: "Crystal Caverns".to_string(),
                        biome: "underground".to_string(),
                        corruption_level: 0.6,
                        level_range: (10, 15),
                        key_features: vec![
                            "light_crystals".to_string(),
                            "ancient_machinery".to_string(),
                        ],
                    },
                    ZoneDefinition {
                        name: "Shadowpeak Mountains".to_string(),
                        biome: "corrupted_peaks".to_string(),
                        corruption_level: 0.8,
                        level_range: (15, 20),
                        key_features: vec!["boss_lair".to_string(), "light_temple".to_string()],
                    },
                ],
                day_night_cycle: true,
                corruption_spreading: true,
            },
            combat: CombatConfig {
                system: "turn_based_tactical".to_string(),
                action_points: true,
                elemental_types: vec![
                    ElementType {
                        name: "Light".to_string(),
                        color: Color::Srgba(Srgba::hex("#ffffff").unwrap()),
                        strong_against: vec!["Shadow".to_string(), "Corruption".to_string()],
                        weak_against: vec!["Void".to_string()],
                    },
                    ElementType {
                        name: "Shadow".to_string(),
                        color: Color::Srgba(Srgba::hex("#1a1a1a").unwrap()),
                        strong_against: vec!["Nature".to_string(), "Spirit".to_string()],
                        weak_against: vec!["Light".to_string()],
                    },
                    ElementType {
                        name: "Nature".to_string(),
                        color: Color::Srgba(Srgba::hex("#4caf50").unwrap()),
                        strong_against: vec!["Earth".to_string(), "Water".to_string()],
                        weak_against: vec!["Shadow".to_string(), "Corruption".to_string()],
                    },
                    ElementType {
                        name: "Corruption".to_string(),
                        color: Color::Srgba(Srgba::hex("#7b1fa2").unwrap()),
                        strong_against: vec!["Nature".to_string(), "Spirit".to_string()],
                        weak_against: vec!["Light".to_string(), "Pure".to_string()],
                    },
                ],
                status_effects: vec![
                    "Poisoned".to_string(),
                    "Blessed".to_string(),
                    "Corrupted".to_string(),
                    "Shielded".to_string(),
                    "Stunned".to_string(),
                ],
            },
            progression: ProgressionConfig {
                level_cap: 50,
                experience_curve: "exponential_smooth".to_string(),
                skill_trees: true,
                equipment_slots: vec![
                    "weapon".to_string(),
                    "armor".to_string(),
                    "accessory".to_string(),
                    "talisman".to_string(),
                ],
            },
        }
    }
}
