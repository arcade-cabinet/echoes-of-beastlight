use bevy::prelude::*;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// The master seed that drives all procedural generation
#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct WorldSeed {
    /// The three word components that make up the seed
    pub adjective: String,
    pub noun: String,
    pub verb: String,

    /// The numeric seed derived from the words
    pub numeric_seed: u64,

    /// The RNG for this world
    #[serde(skip)]
    pub rng: ChaCha8Rng,
}

impl WorldSeed {
    /// Create a new world seed from three words
    pub fn from_words(adjective: &str, noun: &str, verb: &str) -> Self {
        let seed_string = format!("{}-{}-{}", adjective, noun, verb);
        let numeric_seed = Self::hash_string(&seed_string);

        Self {
            adjective: adjective.to_string(),
            noun: noun.to_string(),
            verb: verb.to_string(),
            numeric_seed,
            rng: ChaCha8Rng::seed_from_u64(numeric_seed),
        }
    }

    /// Create a random world seed
    pub fn random() -> Self {
        use crate::config::generation::GenerationConfig;
        let config = GenerationConfig::default();
        let mut rng = rand::thread_rng();

        // Pick random words from our lexicon
        let adj_category = ["mystical", "corrupted", "natural", "ancient"]
            .choose(&mut rng)
            .unwrap();
        let noun_category = ["mystical", "corrupted", "natural", "ancient"]
            .choose(&mut rng)
            .unwrap();
        let verb_category = ["mystical", "corrupted", "natural", "ancient"]
            .choose(&mut rng)
            .unwrap();

        let adjective = match *adj_category {
            "mystical" => config.word_banks.adjectives.mystical.choose(&mut rng),
            "corrupted" => config.word_banks.adjectives.corrupted.choose(&mut rng),
            "natural" => config.word_banks.adjectives.natural.choose(&mut rng),
            "ancient" => config.word_banks.adjectives.ancient.choose(&mut rng),
            _ => unreachable!(),
        }
        .unwrap();

        let noun = match *noun_category {
            "mystical" => config.word_banks.nouns.mystical.choose(&mut rng),
            "corrupted" => config.word_banks.nouns.corrupted.choose(&mut rng),
            "natural" => config.word_banks.nouns.natural.choose(&mut rng),
            "ancient" => config.word_banks.nouns.ancient.choose(&mut rng),
            _ => unreachable!(),
        }
        .unwrap();

        let verb = match *verb_category {
            "mystical" => config.word_banks.verbs.mystical.choose(&mut rng),
            "corrupted" => config.word_banks.verbs.corrupted.choose(&mut rng),
            "natural" => config.word_banks.verbs.natural.choose(&mut rng),
            "ancient" => config.word_banks.verbs.ancient.choose(&mut rng),
            _ => unreachable!(),
        }
        .unwrap();

        Self::from_words(adjective, noun, verb)
    }

    /// Create a sub-seed for a specific system
    pub fn subseed(&self, context: &str) -> ChaCha8Rng {
        let combined = format!("{}-{}", self.numeric_seed, context);
        let subseed = Self::hash_string(&combined);
        ChaCha8Rng::seed_from_u64(subseed)
    }

    /// Get a deterministic value for a specific context
    pub fn get_value(&self, context: &str, max: u64) -> u64 {
        let mut rng = self.subseed(context);
        rng.gen_range(0..max)
    }

    /// Get a deterministic float for a specific context
    pub fn get_float(&self, context: &str) -> f32 {
        let mut rng = self.subseed(context);
        rng.gen()
    }

    /// Hash a string to a u64
    fn hash_string(s: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        s.hash(&mut hasher);
        hasher.finish()
    }
}

/// Component for entities that are procedurally generated
#[derive(Component, Debug, Clone)]
pub struct ProceduralEntity {
    /// The context used to generate this entity
    pub context: String,
    /// The seed value used
    pub seed_value: u64,
}

/// Event for when the world seed changes
#[derive(Event)]
pub struct WorldSeedChanged {
    pub old_seed: Option<WorldSeed>,
    pub new_seed: WorldSeed,
}

pub struct SeedPlugin;

impl Plugin for SeedPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<WorldSeedChanged>()
            .add_systems(Startup, initialize_seed);
    }
}

fn initialize_seed(mut commands: Commands, mut events: EventWriter<WorldSeedChanged>) {
    // Check for command line seed or create random
    let seed = if let Some(seed_arg) = std::env::args().find(|arg| arg.starts_with("--seed=")) {
        let seed_str = seed_arg.trim_start_matches("--seed=");
        let parts: Vec<&str> = seed_str.split('-').collect();
        if parts.len() == 3 {
            WorldSeed::from_words(parts[0], parts[1], parts[2])
        } else {
            eprintln!("Invalid seed format. Use: --seed=adjective-noun-verb");
            WorldSeed::random()
        }
    } else {
        WorldSeed::random()
    };

    info!("World Seed: {}-{}-{}", seed.adjective, seed.noun, seed.verb);

    events.send(WorldSeedChanged {
        old_seed: None,
        new_seed: seed.clone(),
    });

    commands.insert_resource(seed);
}
