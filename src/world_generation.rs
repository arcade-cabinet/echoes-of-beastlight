use bevy::prelude::*;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The world seed that determines all procedural generation
#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct WorldSeed {
    pub seed: u64,
    pub world_name: String,
}

impl WorldSeed {
    pub fn new_random() -> Self {
        let seed = rand::thread_rng().gen();
        Self {
            seed,
            world_name: Self::generate_world_name(seed),
        }
    }

    pub fn from_seed(seed: u64) -> Self {
        Self {
            seed,
            world_name: Self::generate_world_name(seed),
        }
    }

    fn generate_world_name(seed: u64) -> String {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        let adjective = WORLD_ADJECTIVES[rng.gen_range(0..WORLD_ADJECTIVES.len())];
        let noun = WORLD_NOUNS[rng.gen_range(0..WORLD_NOUNS.len())];
        format!("{} {}", adjective, noun)
    }

    pub fn get_rng(&self) -> ChaCha8Rng {
        ChaCha8Rng::seed_from_u64(self.seed)
    }
}

/// Mad libs word banks for procedural generation
const WORLD_ADJECTIVES: &[&str] = &[
    "Crimson", "Azure", "Verdant", "Golden", "Shadow", "Crystal", "Ancient", "Mystic",
    "Forgotten", "Eternal", "Shattered", "Frozen", "Burning", "Whispering", "Silent",
];

const WORLD_NOUNS: &[&str] = &[
    "Realm", "Expanse", "Dominion", "Sanctuary", "Wastes", "Peaks", "Depths", "Haven",
    "Frontier", "Nexus", "Citadel", "Wilds", "Shores", "Highlands", "Abyss",
];

const MONSTER_PREFIXES: &[&str] = &[
    "Flame", "Frost", "Storm", "Shadow", "Crystal", "Venom", "Thunder", "Earth",
    "Void", "Light", "Dark", "Wild", "Ancient", "Mystic", "Cursed",
];

const MONSTER_BASES: &[&str] = &[
    "Wolf", "Drake", "Serpent", "Golem", "Sprite", "Wraith", "Beast", "Elemental",
    "Guardian", "Phantom", "Chimera", "Wyrm", "Shade", "Titan", "Spirit",
];

const MONSTER_SUFFIXES: &[&str] = &[
    "ling", "lord", "spawn", "kin", "touched", "born", "blessed", "cursed",
    "heart", "soul", "mind", "claw", "fang", "wing", "eye",
];

const LOCATION_DESCRIPTORS: &[&str] = &[
    "Abandoned", "Sacred", "Cursed", "Hidden", "Lost", "Forbidden", "Ancient", "Ruined",
    "Mystical", "Forgotten", "Eternal", "Shattered", "Frozen", "Burning", "Whispering",
];

const LOCATION_TYPES: &[&str] = &[
    "Temple", "Fortress", "Grove", "Cavern", "Tower", "Sanctum", "Ruins", "Citadel",
    "Labyrinth", "Catacombs", "Gardens", "Observatory", "Library", "Forge", "Prison",
];

/// Procedurally generated monster with unique traits
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct ProceduralMonster {
    pub name: String,
    pub base_type: String,
    pub element: ElementType,
    pub traits: Vec<MonsterTrait>,
    pub stats: MonsterStats,
    pub tameable: bool,
    pub evolution_paths: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ElementType {
    Fire,
    Water,
    Earth,
    Air,
    Light,
    Dark,
    Neutral,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MonsterTrait {
    Swift,
    Sturdy,
    Magical,
    Venomous,
    Regenerating,
    Ethereal,
    Berserker,
    Guardian,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonsterStats {
    pub health: u32,
    pub attack: u32,
    pub defense: u32,
    pub speed: u32,
    pub magic: u32,
}

/// Procedurally generated location with unique properties
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct ProceduralLocation {
    pub name: String,
    pub biome: BiomeType,
    pub difficulty_tier: u32,
    pub special_features: Vec<LocationFeature>,
    pub dominant_element: ElementType,
    pub map_algorithm: MapGenAlgorithm,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BiomeType {
    Forest,
    Desert,
    Tundra,
    Swamp,
    Mountain,
    Volcanic,
    Crystal,
    Void,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LocationFeature {
    BossArena,
    TreasureVault,
    MerchantPost,
    HealingSpring,
    AncientShrine,
    MonsterNest,
    PuzzleRoom,
    SecretPassage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MapGenAlgorithm {
    CellularAutomata,
    DrunkenWalk,
    RoomAndCorridor,
    MazeGenerator,
    VoronoiRegions,
    WaveFunctionCollapse,
}

/// The world generation DAG (Directed Acyclic Graph)
#[derive(Resource, Debug, Serialize, Deserialize)]
pub struct WorldDAG {
    pub nodes: HashMap<String, WorldNode>,
    pub edges: Vec<(String, String)>,
    pub starting_location: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldNode {
    pub id: String,
    pub location: ProceduralLocation,
    pub monsters: Vec<ProceduralMonster>,
    pub connections: Vec<String>,
    pub unlock_requirements: Vec<UnlockRequirement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UnlockRequirement {
    DefeatBoss(String),
    CollectItem(String),
    TameMonsterType(String),
    ReachLevel(u32),
}

/// System to generate the entire world from a seed
pub fn generate_world_from_seed(
    seed: Res<WorldSeed>,
    mut commands: Commands,
) {
    let mut rng = seed.get_rng();
    
    // Generate the world DAG
    let world_dag = generate_world_dag(&mut rng);
    
    // Generate all locations
    for (node_id, node) in &world_dag.nodes {
        info!("Generating location: {} - {}", node_id, node.location.name);
        
        // Each location gets its own sub-seed for consistent generation
        let location_seed = rng.gen::<u64>();
        let mut location_rng = ChaCha8Rng::seed_from_u64(location_seed);
        
        // Generate the map using the appropriate algorithm
        generate_location_map(&node.location, &mut location_rng);
        
        // Generate monsters for this location
        for monster in &node.monsters {
            spawn_procedural_monster(&mut commands, monster.clone(), &mut location_rng);
        }
    }
    
    commands.insert_resource(world_dag);
}

fn generate_world_dag(rng: &mut ChaCha8Rng) -> WorldDAG {
    let mut nodes = HashMap::new();
    let mut edges = Vec::new();
    
    // Generate 10-15 unique locations
    let location_count = rng.gen_range(10..=15);
    let mut location_ids = Vec::new();
    
    for i in 0..location_count {
        let location = generate_procedural_location(rng, i as u32);
        let node_id = format!("loc_{}", i);
        
        // Generate 3-8 monsters per location
        let monster_count = rng.gen_range(3..=8);
        let mut monsters = Vec::new();
        
        for _ in 0..monster_count {
            monsters.push(generate_procedural_monster(rng, location.difficulty_tier));
        }
        
        let node = WorldNode {
            id: node_id.clone(),
            location,
            monsters,
            connections: Vec::new(),
            unlock_requirements: generate_unlock_requirements(rng, i as u32),
        };
        
        nodes.insert(node_id.clone(), node);
        location_ids.push(node_id);
    }
    
    // Create connections between locations (ensuring it's a DAG)
    for i in 0..location_ids.len() {
        let from = &location_ids[i];
        
        // Connect to 1-3 future locations
        let connection_count = rng.gen_range(1..=3).min(location_ids.len() - i - 1);
        
        for _ in 0..connection_count {
            let to_index = rng.gen_range(i + 1..location_ids.len());
            let to = &location_ids[to_index];
            
            edges.push((from.clone(), to.clone()));
            nodes.get_mut(from).unwrap().connections.push(to.clone());
        }
    }
    
    WorldDAG {
        nodes,
        edges,
        starting_location: location_ids[0].clone(),
    }
}

fn generate_procedural_location(rng: &mut ChaCha8Rng, tier: u32) -> ProceduralLocation {
    let descriptor = LOCATION_DESCRIPTORS[rng.gen_range(0..LOCATION_DESCRIPTORS.len())];
    let location_type = LOCATION_TYPES[rng.gen_range(0..LOCATION_TYPES.len())];
    let name = format!("{} {}", descriptor, location_type);
    
    let biome = match rng.gen_range(0..8) {
        0 => BiomeType::Forest,
        1 => BiomeType::Desert,
        2 => BiomeType::Tundra,
        3 => BiomeType::Swamp,
        4 => BiomeType::Mountain,
        5 => BiomeType::Volcanic,
        6 => BiomeType::Crystal,
        _ => BiomeType::Void,
    };
    
    let element = match rng.gen_range(0..7) {
        0 => ElementType::Fire,
        1 => ElementType::Water,
        2 => ElementType::Earth,
        3 => ElementType::Air,
        4 => ElementType::Light,
        5 => ElementType::Dark,
        _ => ElementType::Neutral,
    };
    
    let algorithm = match &biome {
        BiomeType::Forest | BiomeType::Swamp => MapGenAlgorithm::VoronoiRegions,
        BiomeType::Desert | BiomeType::Tundra => MapGenAlgorithm::CellularAutomata,
        BiomeType::Mountain | BiomeType::Volcanic => MapGenAlgorithm::DrunkenWalk,
        BiomeType::Crystal | BiomeType::Void => MapGenAlgorithm::WaveFunctionCollapse,
    };
    
    let mut features = Vec::new();
    
    // Higher tier locations get more special features
    let feature_count = rng.gen_range(1..=(tier + 1).min(4));
    for _ in 0..feature_count {
        let feature = match rng.gen_range(0..8) {
            0 => LocationFeature::BossArena,
            1 => LocationFeature::TreasureVault,
            2 => LocationFeature::MerchantPost,
            3 => LocationFeature::HealingSpring,
            4 => LocationFeature::AncientShrine,
            5 => LocationFeature::MonsterNest,
            6 => LocationFeature::PuzzleRoom,
            _ => LocationFeature::SecretPassage,
        };
        if !features.contains(&feature) {
            features.push(feature);
        }
    }
    
    ProceduralLocation {
        name,
        biome,
        difficulty_tier: tier,
        special_features: features,
        dominant_element: element,
        map_algorithm: algorithm,
    }
}

fn generate_procedural_monster(rng: &mut ChaCha8Rng, tier: u32) -> ProceduralMonster {
    let prefix = MONSTER_PREFIXES[rng.gen_range(0..MONSTER_PREFIXES.len())];
    let base = MONSTER_BASES[rng.gen_range(0..MONSTER_BASES.len())];
    let suffix = if rng.gen_bool(0.3) {
        MONSTER_SUFFIXES[rng.gen_range(0..MONSTER_SUFFIXES.len())]
    } else {
        ""
    };
    
    let name = format!("{} {}{}", prefix, base, suffix).trim().to_string();
    
    let element = match prefix {
        "Flame" | "Burning" => ElementType::Fire,
        "Frost" | "Frozen" => ElementType::Water,
        "Storm" | "Thunder" => ElementType::Air,
        "Earth" | "Stone" => ElementType::Earth,
        "Shadow" | "Dark" => ElementType::Dark,
        "Light" | "Crystal" => ElementType::Light,
        _ => ElementType::Neutral,
    };
    
    let base_stats = 10 + (tier * 5);
    let stats = MonsterStats {
        health: base_stats + rng.gen_range(0..20),
        attack: base_stats + rng.gen_range(0..15),
        defense: base_stats + rng.gen_range(0..15),
        speed: base_stats + rng.gen_range(0..10),
        magic: base_stats + rng.gen_range(0..15),
    };
    
    let mut traits = Vec::new();
    let trait_count = rng.gen_range(1..=3);
    for _ in 0..trait_count {
        let trait_type = match rng.gen_range(0..8) {
            0 => MonsterTrait::Swift,
            1 => MonsterTrait::Sturdy,
            2 => MonsterTrait::Magical,
            3 => MonsterTrait::Venomous,
            4 => MonsterTrait::Regenerating,
            5 => MonsterTrait::Ethereal,
            6 => MonsterTrait::Berserker,
            _ => MonsterTrait::Guardian,
        };
        if !traits.iter().any(|t| std::mem::discriminant(t) == std::mem::discriminant(&trait_type)) {
            traits.push(trait_type);
        }
    }
    
    // Higher tier monsters are less likely to be tameable
    let tameable = rng.gen_bool(0.7 / (tier as f64 + 1.0));
    
    // Generate evolution paths
    let evolution_paths = if tameable && rng.gen_bool(0.5) {
        vec![format!("Mega {}", name), format!("{} Lord", name)]
    } else {
        vec![]
    };
    
    ProceduralMonster {
        name,
        base_type: base.to_string(),
        element,
        traits,
        stats,
        tameable,
        evolution_paths,
    }
}

fn generate_unlock_requirements(rng: &mut ChaCha8Rng, tier: u32) -> Vec<UnlockRequirement> {
    if tier == 0 {
        return vec![]; // Starting location has no requirements
    }
    
    let mut requirements = Vec::new();
    
    // Higher tiers have more requirements
    let req_count = rng.gen_range(1..=(tier.min(3)));
    
    for _ in 0..req_count {
        let req = match rng.gen_range(0..4) {
            0 => UnlockRequirement::DefeatBoss(format!("Boss_{}", tier - 1)),
            1 => UnlockRequirement::CollectItem(format!("Key_{}", tier)),
            2 => UnlockRequirement::TameMonsterType("Any".to_string()),
            _ => UnlockRequirement::ReachLevel(tier * 5),
        };
        requirements.push(req);
    }
    
    requirements
}

fn generate_location_map(location: &ProceduralLocation, rng: &mut ChaCha8Rng) {
    info!("Generating map for {} using {:?}", location.name, location.map_algorithm);
    
    // This would integrate with your mapgen.rs algorithms
    // Each algorithm generates different tile patterns
    match location.map_algorithm {
        MapGenAlgorithm::CellularAutomata => {
            // Generate cave-like structures
        }
        MapGenAlgorithm::DrunkenWalk => {
            // Generate winding paths
        }
        MapGenAlgorithm::RoomAndCorridor => {
            // Generate dungeon rooms
        }
        MapGenAlgorithm::MazeGenerator => {
            // Generate maze patterns
        }
        MapGenAlgorithm::VoronoiRegions => {
            // Generate organic regions
        }
        MapGenAlgorithm::WaveFunctionCollapse => {
            // Generate complex patterns
        }
    }
}

fn spawn_procedural_monster(
    commands: &mut Commands,
    monster: ProceduralMonster,
    rng: &mut ChaCha8Rng,
) {
    // Spawn the monster entity with all its components
    commands.spawn((
        monster.clone(),
        Transform::from_xyz(
            rng.gen_range(-100.0..100.0),
            rng.gen_range(-100.0..100.0),
            0.0
        ),
        GlobalTransform::default(),
    ));
}

/// Plugin to handle world generation
pub struct WorldGenerationPlugin;

impl Plugin for WorldGenerationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, generate_world_from_seed);
    }
}