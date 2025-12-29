use crate::world::{lexicon::*, seed::*, tiles::*};
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

/// Component for a procedurally generated map section
#[derive(Component, Debug)]
pub struct ProceduralMap {
    pub map_id: String,
    pub connections: MapConnections,
    pub biome: BiomeType,
    pub corruption_level: f32,
    pub has_dungeon: bool,
    pub has_shop: bool,
}

#[derive(Debug, Clone)]
pub struct MapConnections {
    pub north: Option<String>,
    pub south: Option<String>,
    pub east: Option<String>,
    pub west: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BiomeType {
    Forest,
    Desert,
    Mountain,
    Swamp,
    Corrupted,
    Mixed,
}

/// Component for procedurally generated monsters
#[derive(Component, Debug, Clone)]
pub struct ProceduralMonster {
    pub name: GeneratedName,
    pub base_type: String,
    pub abilities: Vec<GeneratedAbility>,
    pub stats: GeneratedStats,
    pub loot_table: Vec<GeneratedLoot>,
}

#[derive(Debug, Clone)]
pub struct GeneratedAbility {
    pub name: String,
    pub damage_type: String,
    pub power: f32,
    pub cost: f32,
}

#[derive(Debug, Clone)]
pub struct GeneratedStats {
    pub health: f32,
    pub attack: f32,
    pub defense: f32,
    pub speed: f32,
    pub corruption_resistance: f32,
}

#[derive(Debug, Clone)]
pub struct GeneratedLoot {
    pub item_name: String,
    pub drop_chance: f32,
    pub quantity_range: (u32, u32),
}

/// System to generate a map from seed
pub fn generate_map_from_seed(
    mut commands: Commands,
    seed: Res<WorldSeed>,
    lexicon: Res<Lexicon>,
    asset_server: Res<AssetServer>,
) {
    let map_id = format!(
        "map_{}_{}",
        seed.get_value("map_x", 1000),
        seed.get_value("map_y", 1000)
    );

    // Generate map name
    let map_name = generate_name_from_lexicon(&lexicon, &seed, &map_id, &["adjective", "noun"]);

    // Determine biome based on corruption and seed
    let corruption = seed.get_float(&format!("{}_corruption", map_id));
    let biome = determine_biome(&seed, &map_id, corruption);

    // Determine if special structures exist
    let has_dungeon = seed.get_float(&format!("{}_dungeon", map_id)) > 0.8;
    let has_shop = seed.get_float(&format!("{}_shop", map_id)) > 0.9;

    // Create tilemap
    let map_size = TilemapSize { x: 32, y: 32 };
    let tile_size = TilemapTileSize { x: 32.0, y: 32.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(map_size);

    // Generate tiles based on biome and seed
    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let world_pos = IVec2::new(x as i32, y as i32);

            // Determine tile type
            let tile_type = generate_tile_type(&seed, &map_id, biome, world_pos, corruption);
            let tile_index = generate_tile_variant(&seed, tile_type, world_pos);

            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: TileTextureIndex(tile_index),
                    ..Default::default()
                })
                .insert(GameTile {
                    tile_type,
                    base_id: tile_index,
                    recolor: None,
                    corruption_level: corruption,
                })
                .id();

            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    // Add special structures
    if has_dungeon {
        place_structure(
            &mut commands,
            &mut tile_storage,
            &seed,
            &map_id,
            StructureType::DungeonEntrance,
        );
    }

    if has_shop {
        place_structure(
            &mut commands,
            &mut tile_storage,
            &seed,
            &map_id,
            StructureType::ShopIcon,
        );
    }

    // Spawn the tilemap
    let texture_handle = asset_server.load("sprites/tilemap.png");

    commands
        .entity(tilemap_entity)
        .insert(TilemapBundle {
            grid_size,
            map_type,
            size: map_size,
            storage: tile_storage,
            texture: TilemapTexture::Single(texture_handle),
            tile_size,
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..Default::default()
        })
        .insert(ProceduralMap {
            map_id: map_id.clone(),
            connections: MapConnections {
                north: None,
                south: None,
                east: None,
                west: None,
            },
            biome,
            corruption_level: corruption,
            has_dungeon,
            has_shop,
        })
        .insert(Name::new(map_name.full_name));
}

fn determine_biome(seed: &WorldSeed, context: &str, corruption: f32) -> BiomeType {
    if corruption > 0.7 {
        return BiomeType::Corrupted;
    }

    let biome_roll = seed.get_value(&format!("{}_biome", context), 100) as f32 / 100.0;

    match biome_roll {
        x if x < 0.3 => BiomeType::Forest,
        x if x < 0.5 => BiomeType::Desert,
        x if x < 0.7 => BiomeType::Mountain,
        x if x < 0.85 => BiomeType::Swamp,
        _ => BiomeType::Mixed,
    }
}

fn generate_tile_type(
    seed: &WorldSeed,
    map_id: &str,
    biome: BiomeType,
    pos: IVec2,
    _corruption: f32,
) -> TileType {
    let context = format!("{}_tile_{}_{}", map_id, pos.x, pos.y);
    let noise = seed.get_float(&context);

    // Apply biome-specific generation
    match biome {
        BiomeType::Forest => {
            if noise < 0.1 {
                TileType::Terrain(TerrainType::Water)
            } else if noise < 0.7 {
                TileType::Terrain(TerrainType::Grass)
            } else {
                TileType::Terrain(TerrainType::Forest)
            }
        }
        BiomeType::Desert => {
            if noise < 0.05 {
                TileType::Terrain(TerrainType::Water)
            } else if noise < 0.9 {
                TileType::Terrain(TerrainType::Sand)
            } else {
                TileType::Terrain(TerrainType::Stone)
            }
        }
        BiomeType::Mountain => {
            if noise < 0.3 {
                TileType::Terrain(TerrainType::Stone)
            } else if noise < 0.8 {
                TileType::Terrain(TerrainType::Mountain)
            } else {
                TileType::Terrain(TerrainType::Grass)
            }
        }
        BiomeType::Swamp => {
            if noise < 0.4 {
                TileType::Terrain(TerrainType::Water)
            } else if noise < 0.8 {
                TileType::Terrain(TerrainType::Swamp)
            } else {
                TileType::Terrain(TerrainType::Grass)
            }
        }
        BiomeType::Corrupted => {
            if noise < 0.9 {
                TileType::Terrain(TerrainType::Corrupted)
            } else {
                TileType::Terrain(TerrainType::Stone)
            }
        }
        BiomeType::Mixed => {
            // Mix of different terrain types
            if noise < 0.2 {
                TileType::Terrain(TerrainType::Water)
            } else if noise < 0.4 {
                TileType::Terrain(TerrainType::Sand)
            } else if noise < 0.6 {
                TileType::Terrain(TerrainType::Grass)
            } else if noise < 0.8 {
                TileType::Terrain(TerrainType::Stone)
            } else {
                TileType::Terrain(TerrainType::Forest)
            }
        }
    }
}

fn place_structure(
    commands: &mut Commands,
    tile_storage: &mut TileStorage,
    seed: &WorldSeed,
    map_id: &str,
    structure_type: StructureType,
) {
    // Find a suitable location
    let x = seed.get_value(&format!("{}_struct_x", map_id), 30) as u32 + 1;
    let y = seed.get_value(&format!("{}_struct_y", map_id), 30) as u32 + 1;

    let tile_pos = TilePos { x, y };
    if let Some(entity) = tile_storage.get(&tile_pos) {
        commands.entity(entity).insert(GameTile {
            tile_type: TileType::Structure(structure_type),
            base_id: get_base_tile_index(TileType::Structure(structure_type)),
            recolor: None,
            corruption_level: 0.0,
        });
    }
}

/// Generate a monster from word combinations
pub fn generate_monster_from_seed(
    seed: &WorldSeed,
    lexicon: &Lexicon,
    level: u32,
    context: &str,
) -> ProceduralMonster {
    // Generate name
    let name = generate_name_from_lexicon(
        lexicon,
        seed,
        &format!("{}_monster", context),
        &["adjective", "noun", "verb"],
    );

    // Determine base type
    let base_types = ["beast", "spirit", "construct", "dragon", "fey"];
    let base_type =
        base_types[seed.get_value(&format!("{}_base", context), base_types.len() as u64) as usize];

    // Generate abilities
    let num_abilities = 1 + (level / 5).min(4);
    let mut abilities = Vec::new();

    for i in 0..num_abilities {
        let ability_name = generate_name_from_lexicon(
            lexicon,
            seed,
            &format!("{}_ability_{}", context, i),
            &["verb", "noun"],
        );

        abilities.push(GeneratedAbility {
            name: ability_name.full_name,
            damage_type: ["physical", "fire", "ice", "lightning", "shadow", "light"]
                [seed.get_value(&format!("{}_dmg_{}", context, i), 6) as usize]
                .to_string(),
            power: 10.0
                + (level as f32 * 2.0)
                + seed.get_float(&format!("{}_pow_{}", context, i)) * 10.0,
            cost: 5.0 + seed.get_float(&format!("{}_cost_{}", context, i)) * 5.0,
        });
    }

    // Generate stats based on level and corruption
    let corruption_modifier = name.corruption_score;
    let stats = GeneratedStats {
        health: 50.0 + (level as f32 * 10.0) * (1.0 + corruption_modifier * 0.5),
        attack: 10.0 + (level as f32 * 2.0) * (1.0 + corruption_modifier * 0.3),
        defense: 8.0 + (level as f32 * 1.5) * (1.0 - corruption_modifier * 0.2),
        speed: 10.0 + seed.get_float(&format!("{}_speed", context)) * 5.0,
        corruption_resistance: 0.5 - corruption_modifier,
    };

    // Generate loot
    let loot_table = vec![
        GeneratedLoot {
            item_name: "Gold".to_string(),
            drop_chance: 0.8,
            quantity_range: (level, level * 3),
        },
        GeneratedLoot {
            item_name: generate_name_from_lexicon(
                lexicon,
                seed,
                &format!("{}_loot", context),
                &["adjective", "noun"],
            )
            .full_name,
            drop_chance: 0.2 + (level as f32 * 0.01),
            quantity_range: (1, 1),
        },
    ];

    ProceduralMonster {
        name,
        base_type: base_type.to_string(),
        abilities,
        stats,
        loot_table,
    }
}

pub struct GenerationPlugin;

impl Plugin for GenerationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, generate_initial_world);
    }
}

fn generate_initial_world(
    commands: Commands,
    seed: Res<WorldSeed>,
    lexicon: Res<Lexicon>,
    asset_server: Res<AssetServer>,
) {
    info!(
        "Generating world from seed: {}-{}-{}",
        seed.adjective, seed.noun, seed.verb
    );

    // Generate the starting map
    generate_map_from_seed(commands, seed, lexicon, asset_server);
}
