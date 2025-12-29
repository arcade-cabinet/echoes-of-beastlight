use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configuration for AI-driven content generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationConfig {
    pub world_generation: WorldGenConfig,
    pub monster_generation: MonsterGenConfig,
    pub item_generation: ItemGenConfig,
    pub dungeon_generation: DungeonGenConfig,
    pub word_banks: WordBanks,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldGenConfig {
    pub method: String,
    pub seed_components: Vec<String>,
    pub biome_transition: String,
    pub landmark_density: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonsterGenConfig {
    pub base_types: Vec<MonsterArchetype>,
    pub corruption_variants: bool,
    pub evolution_chains: bool,
    pub regional_variants: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonsterArchetype {
    pub name: String,
    pub base_element: String,
    pub body_type: String,
    pub behavior: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemGenConfig {
    pub rarity_distribution: HashMap<String, f32>,
    pub enchantment_chance: f32,
    pub corruption_items: bool,
    pub legendary_uniques: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DungeonGenConfig {
    pub algorithm: String,
    pub room_types: Vec<String>,
    pub puzzle_complexity: f32,
    pub treasure_density: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordBanks {
    pub adjectives: WordBank,
    pub nouns: WordBank,
    pub verbs: WordBank,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordBank {
    pub mystical: Vec<String>,
    pub corrupted: Vec<String>,
    pub natural: Vec<String>,
    pub ancient: Vec<String>,
}

impl Default for GenerationConfig {
    fn default() -> Self {
        Self {
            world_generation: WorldGenConfig {
                method: "mad_libs_dag".to_string(),
                seed_components: vec!["adjective".to_string(), "noun".to_string(), "verb".to_string()],
                biome_transition: "corruption_gradient".to_string(),
                landmark_density: 0.15,
            },
            monster_generation: MonsterGenConfig {
                base_types: vec![
                    MonsterArchetype {
                        name: "Wisp".to_string(),
                        base_element: "Light".to_string(),
                        body_type: "ethereal".to_string(),
                        behavior: "curious".to_string(),
                    },
                    MonsterArchetype {
                        name: "Shade".to_string(),
                        base_element: "Shadow".to_string(),
                        body_type: "amorphous".to_string(),
                        behavior: "stalker".to_string(),
                    },
                    MonsterArchetype {
                        name: "Treant".to_string(),
                        base_element: "Nature".to_string(),
                        body_type: "plant".to_string(),
                        behavior: "guardian".to_string(),
                    },
                    MonsterArchetype {
                        name: "Crystal Beast".to_string(),
                        base_element: "Earth".to_string(),
                        body_type: "mineral".to_string(),
                        behavior: "territorial".to_string(),
                    },
                ],
                corruption_variants: true,
                evolution_chains: true,
                regional_variants: true,
            },
            item_generation: ItemGenConfig {
                rarity_distribution: HashMap::from([
                    ("common".to_string(), 0.6),
                    ("uncommon".to_string(), 0.25),
                    ("rare".to_string(), 0.12),
                    ("epic".to_string(), 0.03),
                ]),
                enchantment_chance: 0.2,
                corruption_items: true,
                legendary_uniques: true,
            },
            dungeon_generation: DungeonGenConfig {
                algorithm: "wave_function_collapse".to_string(),
                room_types: vec!["entrance".to_string(), "combat".to_string(), "puzzle".to_string(), "treasure".to_string(), "boss".to_string()],
                puzzle_complexity: 0.7,
                treasure_density: 0.3,
            },
            word_banks: WordBanks {
                adjectives: WordBank {
                    mystical: vec!["Ethereal".to_string(), "Luminous".to_string(), "Arcane".to_string(), "Celestial".to_string(), "Astral".to_string()],
                    corrupted: vec!["Twisted".to_string(), "Blighted".to_string(), "Tainted".to_string(), "Forsaken".to_string(), "Hollow".to_string()],
                    natural: vec!["Verdant".to_string(), "Ancient".to_string(), "Primal".to_string(), "Wild".to_string(), "Untamed".to_string()],
                    ancient: vec!["Forgotten".to_string(), "Elder".to_string(), "Primordial".to_string(), "Timeless".to_string(), "Eternal".to_string()],
                },
                nouns: WordBank {
                    mystical: vec!["Crystal".to_string(), "Beacon".to_string(), "Nexus".to_string(), "Sanctum".to_string(), "Altar".to_string()],
                    corrupted: vec!["Void".to_string(), "Blight".to_string(), "Maw".to_string(), "Scar".to_string(), "Rift".to_string()],
                    natural: vec!["Grove".to_string(), "Spring".to_string(), "Glade".to_string(), "Hollow".to_string(), "Vale".to_string()],
                    ancient: vec!["Ruins".to_string(), "Temple".to_string(), "Citadel".to_string(), "Monument".to_string(), "Archive".to_string()],
                },
                verbs: WordBank {
                    mystical: vec!["Shimmers".to_string(), "Resonates".to_string(), "Pulses".to_string(), "Glows".to_string(), "Hums".to_string()],
                    corrupted: vec!["Festers".to_string(), "Consumes".to_string(), "Corrupts".to_string(), "Devours".to_string(), "Spreads".to_string()],
                    natural: vec!["Blooms".to_string(), "Flows".to_string(), "Grows".to_string(), "Thrives".to_string(), "Whispers".to_string()],
                    ancient: vec!["Slumbers".to_string(), "Watches".to_string(), "Remembers".to_string(), "Endures".to_string(), "Awaits".to_string()],
                },
            },
        }
    }
}
