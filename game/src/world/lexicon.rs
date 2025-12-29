use bevy::prelude::*;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The master lexicon of all words used in procedural generation
#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct Lexicon {
    pub adjectives: WordCategory,
    pub nouns: WordCategory,
    pub verbs: WordCategory,
    pub adverbs: WordCategory,
    pub prefixes: WordCategory,
    pub suffixes: WordCategory,
    pub connectors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordCategory {
    pub categories: HashMap<String, Vec<Word>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Word {
    pub text: String,
    pub tags: Vec<String>,
    pub weight: f32,
    pub corruption_modifier: f32,
}

impl Default for Lexicon {
    fn default() -> Self {
        let mut lexicon = Self {
            adjectives: WordCategory {
                categories: HashMap::new(),
            },
            nouns: WordCategory {
                categories: HashMap::new(),
            },
            verbs: WordCategory {
                categories: HashMap::new(),
            },
            adverbs: WordCategory {
                categories: HashMap::new(),
            },
            prefixes: WordCategory {
                categories: HashMap::new(),
            },
            suffixes: WordCategory {
                categories: HashMap::new(),
            },
            connectors: vec![
                "of".to_string(),
                "the".to_string(),
                "and".to_string(),
                "in".to_string(),
                "at".to_string(),
                "by".to_string(),
                "with".to_string(),
                "from".to_string(),
            ],
        };

        // Build comprehensive adjective categories
        lexicon.adjectives.categories.insert(
            "size".to_string(),
            vec![
                Word {
                    text: "Tiny".to_string(),
                    tags: vec!["small".to_string()],
                    weight: 1.0,
                    corruption_modifier: -0.2,
                },
                Word {
                    text: "Small".to_string(),
                    tags: vec!["small".to_string()],
                    weight: 1.0,
                    corruption_modifier: -0.1,
                },
                Word {
                    text: "Large".to_string(),
                    tags: vec!["big".to_string()],
                    weight: 1.0,
                    corruption_modifier: 0.1,
                },
                Word {
                    text: "Massive".to_string(),
                    tags: vec!["big".to_string()],
                    weight: 0.8,
                    corruption_modifier: 0.2,
                },
                Word {
                    text: "Colossal".to_string(),
                    tags: vec!["big".to_string(), "epic".to_string()],
                    weight: 0.5,
                    corruption_modifier: 0.3,
                },
            ],
        );

        lexicon.adjectives.categories.insert(
            "element".to_string(),
            vec![
                Word {
                    text: "Burning".to_string(),
                    tags: vec!["fire".to_string()],
                    weight: 1.0,
                    corruption_modifier: 0.0,
                },
                Word {
                    text: "Frozen".to_string(),
                    tags: vec!["ice".to_string()],
                    weight: 1.0,
                    corruption_modifier: 0.0,
                },
                Word {
                    text: "Electric".to_string(),
                    tags: vec!["lightning".to_string()],
                    weight: 1.0,
                    corruption_modifier: 0.0,
                },
                Word {
                    text: "Earthen".to_string(),
                    tags: vec!["earth".to_string()],
                    weight: 1.0,
                    corruption_modifier: -0.1,
                },
                Word {
                    text: "Ethereal".to_string(),
                    tags: vec!["spirit".to_string()],
                    weight: 0.8,
                    corruption_modifier: -0.2,
                },
                Word {
                    text: "Shadow".to_string(),
                    tags: vec!["dark".to_string()],
                    weight: 0.9,
                    corruption_modifier: 0.3,
                },
                Word {
                    text: "Luminous".to_string(),
                    tags: vec!["light".to_string()],
                    weight: 0.9,
                    corruption_modifier: -0.3,
                },
            ],
        );

        lexicon.adjectives.categories.insert(
            "condition".to_string(),
            vec![
                Word {
                    text: "Ancient".to_string(),
                    tags: vec!["old".to_string()],
                    weight: 1.0,
                    corruption_modifier: 0.1,
                },
                Word {
                    text: "Pristine".to_string(),
                    tags: vec!["pure".to_string()],
                    weight: 0.8,
                    corruption_modifier: -0.4,
                },
                Word {
                    text: "Corrupted".to_string(),
                    tags: vec!["tainted".to_string()],
                    weight: 0.9,
                    corruption_modifier: 0.5,
                },
                Word {
                    text: "Forgotten".to_string(),
                    tags: vec!["lost".to_string()],
                    weight: 0.9,
                    corruption_modifier: 0.2,
                },
                Word {
                    text: "Blessed".to_string(),
                    tags: vec!["holy".to_string()],
                    weight: 0.7,
                    corruption_modifier: -0.5,
                },
                Word {
                    text: "Cursed".to_string(),
                    tags: vec!["evil".to_string()],
                    weight: 0.7,
                    corruption_modifier: 0.5,
                },
            ],
        );

        // Build noun categories
        lexicon.nouns.categories.insert(
            "creature_base".to_string(),
            vec![
                Word {
                    text: "Wolf".to_string(),
                    tags: vec!["beast".to_string(), "predator".to_string()],
                    weight: 1.0,
                    corruption_modifier: 0.0,
                },
                Word {
                    text: "Bear".to_string(),
                    tags: vec!["beast".to_string(), "strong".to_string()],
                    weight: 1.0,
                    corruption_modifier: 0.0,
                },
                Word {
                    text: "Sprite".to_string(),
                    tags: vec!["fey".to_string(), "small".to_string()],
                    weight: 1.0,
                    corruption_modifier: -0.2,
                },
                Word {
                    text: "Drake".to_string(),
                    tags: vec!["dragon".to_string(), "flying".to_string()],
                    weight: 0.8,
                    corruption_modifier: 0.1,
                },
                Word {
                    text: "Golem".to_string(),
                    tags: vec!["construct".to_string()],
                    weight: 0.9,
                    corruption_modifier: 0.0,
                },
                Word {
                    text: "Wisp".to_string(),
                    tags: vec!["spirit".to_string(), "light".to_string()],
                    weight: 1.0,
                    corruption_modifier: -0.1,
                },
                Word {
                    text: "Shade".to_string(),
                    tags: vec!["spirit".to_string(), "dark".to_string()],
                    weight: 1.0,
                    corruption_modifier: 0.2,
                },
            ],
        );

        lexicon.nouns.categories.insert(
            "location_type".to_string(),
            vec![
                Word {
                    text: "Grove".to_string(),
                    tags: vec!["nature".to_string()],
                    weight: 1.0,
                    corruption_modifier: -0.2,
                },
                Word {
                    text: "Cavern".to_string(),
                    tags: vec!["underground".to_string()],
                    weight: 1.0,
                    corruption_modifier: 0.1,
                },
                Word {
                    text: "Peak".to_string(),
                    tags: vec!["mountain".to_string()],
                    weight: 1.0,
                    corruption_modifier: 0.0,
                },
                Word {
                    text: "Marsh".to_string(),
                    tags: vec!["water".to_string(), "dark".to_string()],
                    weight: 1.0,
                    corruption_modifier: 0.2,
                },
                Word {
                    text: "Temple".to_string(),
                    tags: vec!["structure".to_string(), "holy".to_string()],
                    weight: 0.8,
                    corruption_modifier: -0.3,
                },
                Word {
                    text: "Ruins".to_string(),
                    tags: vec!["structure".to_string(), "old".to_string()],
                    weight: 0.9,
                    corruption_modifier: 0.3,
                },
            ],
        );

        lexicon.nouns.categories.insert(
            "item_base".to_string(),
            vec![
                Word {
                    text: "Blade".to_string(),
                    tags: vec!["weapon".to_string(), "slash".to_string()],
                    weight: 1.0,
                    corruption_modifier: 0.0,
                },
                Word {
                    text: "Staff".to_string(),
                    tags: vec!["weapon".to_string(), "magic".to_string()],
                    weight: 1.0,
                    corruption_modifier: -0.1,
                },
                Word {
                    text: "Orb".to_string(),
                    tags: vec!["magic".to_string(), "round".to_string()],
                    weight: 0.9,
                    corruption_modifier: 0.0,
                },
                Word {
                    text: "Ring".to_string(),
                    tags: vec!["accessory".to_string()],
                    weight: 1.0,
                    corruption_modifier: 0.0,
                },
                Word {
                    text: "Amulet".to_string(),
                    tags: vec!["accessory".to_string(), "magic".to_string()],
                    weight: 0.9,
                    corruption_modifier: -0.1,
                },
                Word {
                    text: "Tome".to_string(),
                    tags: vec!["magic".to_string(), "knowledge".to_string()],
                    weight: 0.8,
                    corruption_modifier: 0.0,
                },
            ],
        );

        // Build verb categories
        lexicon.verbs.categories.insert(
            "action".to_string(),
            vec![
                Word {
                    text: "Strikes".to_string(),
                    tags: vec!["attack".to_string()],
                    weight: 1.0,
                    corruption_modifier: 0.0,
                },
                Word {
                    text: "Guards".to_string(),
                    tags: vec!["defend".to_string()],
                    weight: 1.0,
                    corruption_modifier: -0.1,
                },
                Word {
                    text: "Hunts".to_string(),
                    tags: vec!["pursue".to_string()],
                    weight: 1.0,
                    corruption_modifier: 0.1,
                },
                Word {
                    text: "Seeks".to_string(),
                    tags: vec!["search".to_string()],
                    weight: 1.0,
                    corruption_modifier: 0.0,
                },
                Word {
                    text: "Devours".to_string(),
                    tags: vec!["consume".to_string()],
                    weight: 0.8,
                    corruption_modifier: 0.3,
                },
                Word {
                    text: "Protects".to_string(),
                    tags: vec!["defend".to_string(), "holy".to_string()],
                    weight: 0.9,
                    corruption_modifier: -0.3,
                },
            ],
        );

        lexicon.verbs.categories.insert(
            "state".to_string(),
            vec![
                Word {
                    text: "Slumbers".to_string(),
                    tags: vec!["rest".to_string()],
                    weight: 1.0,
                    corruption_modifier: 0.0,
                },
                Word {
                    text: "Wanders".to_string(),
                    tags: vec!["move".to_string()],
                    weight: 1.0,
                    corruption_modifier: 0.0,
                },
                Word {
                    text: "Watches".to_string(),
                    tags: vec!["observe".to_string()],
                    weight: 1.0,
                    corruption_modifier: 0.0,
                },
                Word {
                    text: "Corrupts".to_string(),
                    tags: vec!["taint".to_string()],
                    weight: 0.8,
                    corruption_modifier: 0.5,
                },
                Word {
                    text: "Purifies".to_string(),
                    tags: vec!["cleanse".to_string()],
                    weight: 0.8,
                    corruption_modifier: -0.5,
                },
            ],
        );

        // Prefixes and suffixes for variety
        lexicon.prefixes.categories.insert(
            "monster".to_string(),
            vec![
                Word {
                    text: "Alpha".to_string(),
                    tags: vec!["leader".to_string()],
                    weight: 0.7,
                    corruption_modifier: 0.1,
                },
                Word {
                    text: "Elder".to_string(),
                    tags: vec!["old".to_string()],
                    weight: 0.8,
                    corruption_modifier: 0.2,
                },
                Word {
                    text: "Young".to_string(),
                    tags: vec!["small".to_string()],
                    weight: 1.0,
                    corruption_modifier: -0.1,
                },
            ],
        );

        lexicon.suffixes.categories.insert(
            "monster".to_string(),
            vec![
                Word {
                    text: "ling".to_string(),
                    tags: vec!["small".to_string()],
                    weight: 1.0,
                    corruption_modifier: -0.1,
                },
                Word {
                    text: "lord".to_string(),
                    tags: vec!["boss".to_string()],
                    weight: 0.5,
                    corruption_modifier: 0.3,
                },
                Word {
                    text: "spawn".to_string(),
                    tags: vec!["offspring".to_string()],
                    weight: 0.9,
                    corruption_modifier: 0.1,
                },
            ],
        );

        lexicon
    }
}

/// Component for word-generated entities
#[derive(Component, Debug, Clone)]
pub struct GeneratedName {
    pub words: Vec<String>,
    pub full_name: String,
    pub corruption_score: f32,
}

/// System to generate names from lexicon
pub fn generate_name_from_lexicon(
    lexicon: &Lexicon,
    seed: &crate::world::seed::WorldSeed,
    context: &str,
    pattern: &[&str], // e.g. ["adjective", "noun", "verb"]
) -> GeneratedName {
    let mut rng = seed.subseed(context);
    let mut words = Vec::new();
    let mut corruption_score = 0.0;

    for part in pattern {
        let word = match *part {
            "adjective" => {
                let categories: Vec<_> = lexicon.adjectives.categories.keys().collect();
                let category = categories[rng.random_range(0..categories.len())];
                let word_list = &lexicon.adjectives.categories[category];
                word_list[rng.random_range(0..word_list.len())].clone()
            }
            "noun" => {
                let categories: Vec<_> = lexicon.nouns.categories.keys().collect();
                let category = categories[rng.random_range(0..categories.len())];
                let word_list = &lexicon.nouns.categories[category];
                word_list[rng.random_range(0..word_list.len())].clone()
            }
            "verb" => {
                let categories: Vec<_> = lexicon.verbs.categories.keys().collect();
                let category = categories[rng.random_range(0..categories.len())];
                let word_list = &lexicon.verbs.categories[category];
                word_list[rng.random_range(0..word_list.len())].clone()
            }
            _ => continue,
        };

        words.push(word.text.clone());
        corruption_score += word.corruption_modifier;
    }

    let full_name = words.join(" ");

    GeneratedName {
        words,
        full_name,
        corruption_score: corruption_score / pattern.len() as f32,
    }
}

pub struct LexiconPlugin;

impl Plugin for LexiconPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Lexicon::default());
    }
}
