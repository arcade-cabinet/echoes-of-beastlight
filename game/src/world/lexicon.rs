use bevy::prelude::*;
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
    pub connectors: Vec<&'static str>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordCategory {
    pub categories: HashMap<String, Vec<Word>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Word {
    pub text: &'static str,
    pub tags: Vec<&'static str>,
    pub weight: f32,
    pub corruption_modifier: f32,
}

impl Default for Lexicon {
    fn default() -> Self {
        let mut lexicon = Self {
            adjectives: WordCategory { categories: HashMap::new() },
            nouns: WordCategory { categories: HashMap::new() },
            verbs: WordCategory { categories: HashMap::new() },
            adverbs: WordCategory { categories: HashMap::new() },
            prefixes: WordCategory { categories: HashMap::new() },
            suffixes: WordCategory { categories: HashMap::new() },
            connectors: vec!["of", "the", "and", "in", "at", "by", "with", "from"],
        };
        
        // Build comprehensive adjective categories
        lexicon.adjectives.categories.insert("size".to_string(), vec![
            Word { text: "Tiny", tags: vec!["small"], weight: 1.0, corruption_modifier: -0.2 },
            Word { text: "Small", tags: vec!["small"], weight: 1.0, corruption_modifier: -0.1 },
            Word { text: "Large", tags: vec!["big"], weight: 1.0, corruption_modifier: 0.1 },
            Word { text: "Massive", tags: vec!["big"], weight: 0.8, corruption_modifier: 0.2 },
            Word { text: "Colossal", tags: vec!["big", "epic"], weight: 0.5, corruption_modifier: 0.3 },
        ]);
        
        lexicon.adjectives.categories.insert("element".to_string(), vec![
            Word { text: "Burning", tags: vec!["fire"], weight: 1.0, corruption_modifier: 0.0 },
            Word { text: "Frozen", tags: vec!["ice"], weight: 1.0, corruption_modifier: 0.0 },
            Word { text: "Electric", tags: vec!["lightning"], weight: 1.0, corruption_modifier: 0.0 },
            Word { text: "Earthen", tags: vec!["earth"], weight: 1.0, corruption_modifier: -0.1 },
            Word { text: "Ethereal", tags: vec!["spirit"], weight: 0.8, corruption_modifier: -0.2 },
            Word { text: "Shadow", tags: vec!["dark"], weight: 0.9, corruption_modifier: 0.3 },
            Word { text: "Luminous", tags: vec!["light"], weight: 0.9, corruption_modifier: -0.3 },
        ]);
        
        lexicon.adjectives.categories.insert("condition".to_string(), vec![
            Word { text: "Ancient", tags: vec!["old"], weight: 1.0, corruption_modifier: 0.1 },
            Word { text: "Pristine", tags: vec!["pure"], weight: 0.8, corruption_modifier: -0.4 },
            Word { text: "Corrupted", tags: vec!["tainted"], weight: 0.9, corruption_modifier: 0.5 },
            Word { text: "Forgotten", tags: vec!["lost"], weight: 0.9, corruption_modifier: 0.2 },
            Word { text: "Blessed", tags: vec!["holy"], weight: 0.7, corruption_modifier: -0.5 },
            Word { text: "Cursed", tags: vec!["evil"], weight: 0.7, corruption_modifier: 0.5 },
        ]);
        
        // Build noun categories
        lexicon.nouns.categories.insert("creature_base".to_string(), vec![
            Word { text: "Wolf", tags: vec!["beast", "predator"], weight: 1.0, corruption_modifier: 0.0 },
            Word { text: "Bear", tags: vec!["beast", "strong"], weight: 1.0, corruption_modifier: 0.0 },
            Word { text: "Sprite", tags: vec!["fey", "small"], weight: 1.0, corruption_modifier: -0.2 },
            Word { text: "Drake", tags: vec!["dragon", "flying"], weight: 0.8, corruption_modifier: 0.1 },
            Word { text: "Golem", tags: vec!["construct"], weight: 0.9, corruption_modifier: 0.0 },
            Word { text: "Wisp", tags: vec!["spirit", "light"], weight: 1.0, corruption_modifier: -0.1 },
            Word { text: "Shade", tags: vec!["spirit", "dark"], weight: 1.0, corruption_modifier: 0.2 },
        ]);
        
        lexicon.nouns.categories.insert("location_type".to_string(), vec![
            Word { text: "Grove", tags: vec!["nature"], weight: 1.0, corruption_modifier: -0.2 },
            Word { text: "Cavern", tags: vec!["underground"], weight: 1.0, corruption_modifier: 0.1 },
            Word { text: "Peak", tags: vec!["mountain"], weight: 1.0, corruption_modifier: 0.0 },
            Word { text: "Marsh", tags: vec!["water", "dark"], weight: 1.0, corruption_modifier: 0.2 },
            Word { text: "Temple", tags: vec!["structure", "holy"], weight: 0.8, corruption_modifier: -0.3 },
            Word { text: "Ruins", tags: vec!["structure", "old"], weight: 0.9, corruption_modifier: 0.3 },
        ]);
        
        lexicon.nouns.categories.insert("item_base".to_string(), vec![
            Word { text: "Blade", tags: vec!["weapon", "slash"], weight: 1.0, corruption_modifier: 0.0 },
            Word { text: "Staff", tags: vec!["weapon", "magic"], weight: 1.0, corruption_modifier: -0.1 },
            Word { text: "Orb", tags: vec!["magic", "round"], weight: 0.9, corruption_modifier: 0.0 },
            Word { text: "Ring", tags: vec!["accessory"], weight: 1.0, corruption_modifier: 0.0 },
            Word { text: "Amulet", tags: vec!["accessory", "magic"], weight: 0.9, corruption_modifier: -0.1 },
            Word { text: "Tome", tags: vec!["magic", "knowledge"], weight: 0.8, corruption_modifier: 0.0 },
        ]);
        
        // Build verb categories
        lexicon.verbs.categories.insert("action".to_string(), vec![
            Word { text: "Strikes", tags: vec!["attack"], weight: 1.0, corruption_modifier: 0.0 },
            Word { text: "Guards", tags: vec!["defend"], weight: 1.0, corruption_modifier: -0.1 },
            Word { text: "Hunts", tags: vec!["pursue"], weight: 1.0, corruption_modifier: 0.1 },
            Word { text: "Seeks", tags: vec!["search"], weight: 1.0, corruption_modifier: 0.0 },
            Word { text: "Devours", tags: vec!["consume"], weight: 0.8, corruption_modifier: 0.3 },
            Word { text: "Protects", tags: vec!["defend", "holy"], weight: 0.9, corruption_modifier: -0.3 },
        ]);
        
        lexicon.verbs.categories.insert("state".to_string(), vec![
            Word { text: "Slumbers", tags: vec!["rest"], weight: 1.0, corruption_modifier: 0.0 },
            Word { text: "Wanders", tags: vec!["move"], weight: 1.0, corruption_modifier: 0.0 },
            Word { text: "Watches", tags: vec!["observe"], weight: 1.0, corruption_modifier: 0.0 },
            Word { text: "Corrupts", tags: vec!["taint"], weight: 0.8, corruption_modifier: 0.5 },
            Word { text: "Purifies", tags: vec!["cleanse"], weight: 0.8, corruption_modifier: -0.5 },
        ]);
        
        // Prefixes and suffixes for variety
        lexicon.prefixes.categories.insert("monster".to_string(), vec![
            Word { text: "Alpha", tags: vec!["leader"], weight: 0.7, corruption_modifier: 0.1 },
            Word { text: "Elder", tags: vec!["old"], weight: 0.8, corruption_modifier: 0.2 },
            Word { text: "Young", tags: vec!["small"], weight: 1.0, corruption_modifier: -0.1 },
        ]);
        
        lexicon.suffixes.categories.insert("monster".to_string(), vec![
            Word { text: "ling", tags: vec!["small"], weight: 1.0, corruption_modifier: -0.1 },
            Word { text: "lord", tags: vec!["boss"], weight: 0.5, corruption_modifier: 0.3 },
            Word { text: "spawn", tags: vec!["offspring"], weight: 0.9, corruption_modifier: 0.1 },
        ]);
        
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
                let category = categories[rng.gen_range(0..categories.len())];
                let word_list = &lexicon.adjectives.categories[category];
                word_list[rng.gen_range(0..word_list.len())].clone()
            }
            "noun" => {
                let categories: Vec<_> = lexicon.nouns.categories.keys().collect();
                let category = categories[rng.gen_range(0..categories.len())];
                let word_list = &lexicon.nouns.categories[category];
                word_list[rng.gen_range(0..word_list.len())].clone()
            }
            "verb" => {
                let categories: Vec<_> = lexicon.verbs.categories.keys().collect();
                let category = categories[rng.gen_range(0..categories.len())];
                let word_list = &lexicon.verbs.categories[category];
                word_list[rng.gen_range(0..word_list.len())].clone()
            }
            _ => continue,
        };
        
        words.push(word.text.to_string());
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