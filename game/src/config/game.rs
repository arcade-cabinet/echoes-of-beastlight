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
    pub name: &'static str,
    pub class: &'static str,
    pub base_stats: BaseStats,
    pub starting_abilities: Vec<&'static str>,
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
    pub name: &'static str,
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
    pub name: &'static str,
    pub biome: &'static str,
    pub corruption_level: f32,
    pub level_range: (u32, u32),
    pub key_features: Vec<&'static str>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatConfig {
    pub system: &'static str,
    pub action_points: bool,
    pub elemental_types: Vec<ElementType>,
    pub status_effects: Vec<&'static str>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementType {
    pub name: &'static str,
    pub color: Color,
    pub strong_against: Vec<&'static str>,
    pub weak_against: Vec<&'static str>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressionConfig {
    pub level_cap: u32,
    pub experience_curve: &'static str,
    pub skill_trees: bool,
    pub equipment_slots: Vec<&'static str>,
}

impl Default for GameplayConfig {
    fn default() -> Self {
        Self {
            hero: HeroConfig {
                name: "Luna",
                class: "Lightkeeper",
                base_stats: BaseStats {
                    health: 100,
                    mana: 50,
                    attack: 15,
                    defense: 10,
                    speed: 12,
                    spirit: 20,
                },
                starting_abilities: vec!["Light Burst", "Tame", "Heal"],
                max_party_size: 6,
            },
            monsters: MonsterConfig {
                taming_enabled: true,
                max_active_monsters: 3,
                evolution_enabled: true,
                type_effectiveness: true,
                rarity_tiers: vec![
                    RarityTier {
                        name: "Common",
                        color: Color::hex("#9e9e9e").unwrap(),
                        spawn_weight: 0.6,
                        stat_multiplier: 1.0,
                    },
                    RarityTier {
                        name: "Uncommon",
                        color: Color::hex("#4caf50").unwrap(),
                        spawn_weight: 0.25,
                        stat_multiplier: 1.2,
                    },
                    RarityTier {
                        name: "Rare",
                        color: Color::hex("#2196f3").unwrap(),
                        spawn_weight: 0.12,
                        stat_multiplier: 1.5,
                    },
                    RarityTier {
                        name: "Epic",
                        color: Color::hex("#9c27b0").unwrap(),
                        spawn_weight: 0.03,
                        stat_multiplier: 2.0,
                    },
                ],
            },
            world: WorldConfig {
                seed_based: true,
                zones: vec![
                    ZoneDefinition {
                        name: "Glintrock Village",
                        biome: "corrupted_forest",
                        corruption_level: 0.2,
                        level_range: (1, 5),
                        key_features: vec!["starting_town", "basic_shops", "taming_tutorial"],
                    },
                    ZoneDefinition {
                        name: "Whispering Woods",
                        biome: "dark_forest",
                        corruption_level: 0.4,
                        level_range: (5, 10),
                        key_features: vec!["first_dungeon", "corrupted_shrine"],
                    },
                    ZoneDefinition {
                        name: "Crystal Caverns",
                        biome: "underground",
                        corruption_level: 0.6,
                        level_range: (10, 15),
                        key_features: vec!["light_crystals", "ancient_machinery"],
                    },
                    ZoneDefinition {
                        name: "Shadowpeak Mountains",
                        biome: "corrupted_peaks",
                        corruption_level: 0.8,
                        level_range: (15, 20),
                        key_features: vec!["boss_lair", "light_temple"],
                    },
                ],
                day_night_cycle: true,
                corruption_spreading: true,
            },
            combat: CombatConfig {
                system: "turn_based_tactical",
                action_points: true,
                elemental_types: vec![
                    ElementType {
                        name: "Light",
                        color: Color::hex("#ffffff").unwrap(),
                        strong_against: vec!["Shadow", "Corruption"],
                        weak_against: vec!["Void"],
                    },
                    ElementType {
                        name: "Shadow",
                        color: Color::hex("#1a1a1a").unwrap(),
                        strong_against: vec!["Nature", "Spirit"],
                        weak_against: vec!["Light"],
                    },
                    ElementType {
                        name: "Nature",
                        color: Color::hex("#4caf50").unwrap(),
                        strong_against: vec!["Earth", "Water"],
                        weak_against: vec!["Shadow", "Corruption"],
                    },
                    ElementType {
                        name: "Corruption",
                        color: Color::hex("#7b1fa2").unwrap(),
                        strong_against: vec!["Nature", "Spirit"],
                        weak_against: vec!["Light", "Pure"],
                    },
                ],
                status_effects: vec!["Poisoned", "Blessed", "Corrupted", "Shielded", "Stunned"],
            },
            progression: ProgressionConfig {
                level_cap: 50,
                experience_curve: "exponential_smooth",
                skill_trees: true,
                equipment_slots: vec!["weapon", "armor", "accessory", "talisman"],
            },
        }
    }
}
