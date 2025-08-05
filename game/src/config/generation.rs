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
    pub method: &'static str,
    pub seed_components: Vec<&'static str>,
    pub biome_transition: &'static str,
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
    pub name: &'static str,
    pub base_element: &'static str,
    pub body_type: &'static str,
    pub behavior: &'static str,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemGenConfig {
    pub rarity_distribution: HashMap<&'static str, f32>,
    pub enchantment_chance: f32,
    pub corruption_items: bool,
    pub legendary_uniques: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DungeonGenConfig {
    pub algorithm: &'static str,
    pub room_types: Vec<&'static str>,
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
    pub mystical: Vec<&'static str>,
    pub corrupted: Vec<&'static str>,
    pub natural: Vec<&'static str>,
    pub ancient: Vec<&'static str>,
}

impl Default for GenerationConfig {
    fn default() -> Self {
        Self {
            world_generation: WorldGenConfig {
                method: "mad_libs_dag",
                seed_components: vec!["adjective", "noun", "verb"],
                biome_transition: "corruption_gradient",
                landmark_density: 0.15,
            },
            monster_generation: MonsterGenConfig {
                base_types: vec![
                    MonsterArchetype {
                        name: "Wisp",
                        base_element: "Light",
                        body_type: "ethereal",
                        behavior: "curious",
                    },
                    MonsterArchetype {
                        name: "Shade",
                        base_element: "Shadow",
                        body_type: "amorphous",
                        behavior: "stalker",
                    },
                    MonsterArchetype {
                        name: "Treant",
                        base_element: "Nature",
                        body_type: "plant",
                        behavior: "guardian",
                    },
                    MonsterArchetype {
                        name: "Crystal Beast",
                        base_element: "Earth",
                        body_type: "mineral",
                        behavior: "territorial",
                    },
                ],
                corruption_variants: true,
                evolution_chains: true,
                regional_variants: true,
            },
            item_generation: ItemGenConfig {
                rarity_distribution: HashMap::from([
                    ("common", 0.6),
                    ("uncommon", 0.25),
                    ("rare", 0.12),
                    ("epic", 0.03),
                ]),
                enchantment_chance: 0.2,
                corruption_items: true,
                legendary_uniques: true,
            },
            dungeon_generation: DungeonGenConfig {
                algorithm: "wave_function_collapse",
                room_types: vec!["entrance", "combat", "puzzle", "treasure", "boss"],
                puzzle_complexity: 0.7,
                treasure_density: 0.3,
            },
            word_banks: WordBanks {
                adjectives: WordBank {
                    mystical: vec!["Ethereal", "Luminous", "Arcane", "Celestial", "Astral"],
                    corrupted: vec!["Twisted", "Blighted", "Tainted", "Forsaken", "Hollow"],
                    natural: vec!["Verdant", "Ancient", "Primal", "Wild", "Untamed"],
                    ancient: vec!["Forgotten", "Elder", "Primordial", "Timeless", "Eternal"],
                },
                nouns: WordBank {
                    mystical: vec!["Crystal", "Beacon", "Nexus", "Sanctum", "Altar"],
                    corrupted: vec!["Void", "Blight", "Maw", "Scar", "Rift"],
                    natural: vec!["Grove", "Spring", "Glade", "Hollow", "Vale"],
                    ancient: vec!["Ruins", "Temple", "Citadel", "Monument", "Archive"],
                },
                verbs: WordBank {
                    mystical: vec!["Shimmers", "Resonates", "Pulses", "Glows", "Hums"],
                    corrupted: vec!["Festers", "Consumes", "Corrupts", "Devours", "Spreads"],
                    natural: vec!["Blooms", "Flows", "Grows", "Thrives", "Whispers"],
                    ancient: vec!["Slumbers", "Watches", "Remembers", "Endures", "Awaits"],
                },
            },
        }
    }
}