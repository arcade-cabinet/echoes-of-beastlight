use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use crate::world_generation::{ProceduralLocation, MapGenAlgorithm, BiomeType};
use crate::mapgen::{MapBuilder, Map, TileType as MapGenTileType};

/// Resource containing the style-transferred tile textures
#[derive(Resource)]
pub struct StyledTileAssets {
    pub texture_handle: Handle<Image>,
    pub atlas_handle: Handle<TextureAtlasLayout>,
    pub tile_mappings: TileMappings,
}

#[derive(Debug, Clone)]
pub struct TileMappings {
    pub floor_tiles: Vec<u32>,      // Multiple variations
    pub wall_tiles: Vec<u32>,       // Multiple variations
    pub water_tiles: Vec<u32>,      // Animated frames
    pub special_tiles: Vec<u32>,    // Decorative elements
    pub transition_tiles: TransitionTiles,
}

#[derive(Debug, Clone)]
pub struct TransitionTiles {
    pub floor_to_wall: Vec<u32>,
    pub floor_to_water: Vec<u32>,
    pub biome_transitions: Vec<u32>,
}

/// Component for animated tiles
#[derive(Component)]
pub struct AnimatedTile {
    pub frames: Vec<u32>,
    pub current_frame: usize,
    pub timer: Timer,
}

/// System to generate a tilemap from a procedural location
pub fn generate_location_tilemap(
    location: &ProceduralLocation,
    styled_assets: &StyledTileAssets,
    commands: &mut Commands,
    rng: &mut ChaCha8Rng,
) -> Entity {
    let map_size = TilemapSize { x: 50, y: 50 };
    let tile_size = TilemapTileSize { x: 32.0, y: 32.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();
    
    // Generate the base map using mapgen
    let mapgen_map = generate_base_map(&location.map_algorithm, map_size, rng);
    
    // Create tilemap entity
    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(map_size);
    
    // Convert mapgen tiles to styled tiles
    for y in 0..map_size.y {
        for x in 0..map_size.x {
            let pos = TilePos { x, y };
            let idx = (y * map_size.x + x) as usize;
            let mapgen_tile = mapgen_map.tiles[idx];
            
            let (tile_texture_index, is_animated) = select_styled_tile(
                mapgen_tile,
                &location.biome,
                &styled_assets.tile_mappings,
                rng,
                &mapgen_map,
                x,
                y,
            );
            
            let tile_entity = if is_animated {
                // Create animated tile
                commands.spawn((
                    TileBundle {
                        position: pos,
                        tilemap_id: TilemapId(tilemap_entity),
                        texture_index: TileTextureIndex(tile_texture_index),
                        ..Default::default()
                    },
                    AnimatedTile {
                        frames: match mapgen_tile {
                            MapGenTileType::Water => styled_assets.tile_mappings.water_tiles.clone(),
                            _ => vec![tile_texture_index],
                        },
                        current_frame: 0,
                        timer: Timer::from_seconds(0.5, TimerMode::Repeating),
                    },
                )).id()
            } else {
                // Create static tile
                commands.spawn(TileBundle {
                    position: pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: TileTextureIndex(tile_texture_index),
                    ..Default::default()
                }).id()
            };
            
            tile_storage.set(&pos, tile_entity);
        }
    }
    
    // Add special features based on location
    add_location_features(
        &location.special_features,
        &mut tile_storage,
        commands,
        tilemap_entity,
        &styled_assets.tile_mappings,
        rng,
    );
    
    // Finalize tilemap
    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(styled_assets.texture_handle.clone()),
        tile_size,
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        ..Default::default()
    });
    
    tilemap_entity
}

fn generate_base_map(
    algorithm: &MapGenAlgorithm,
    size: TilemapSize,
    rng: &mut ChaCha8Rng,
) -> Map {
    let mut builder = MapBuilder::new(size.x as usize, size.y as usize);
    
    match algorithm {
        MapGenAlgorithm::CellularAutomata => {
            builder.with_cellular_automata(55, 5, rng);
        }
        MapGenAlgorithm::DrunkenWalk => {
            builder.with_drunkard(2000, rng);
        }
        MapGenAlgorithm::RoomAndCorridor => {
            builder.with_rooms_and_corridors(10, 6, 10, rng);
        }
        MapGenAlgorithm::MazeGenerator => {
            builder.with_maze(rng);
        }
        MapGenAlgorithm::VoronoiRegions => {
            builder.with_voronoi(25, rng);
        }
        MapGenAlgorithm::WaveFunctionCollapse => {
            // For now, use cellular automata as placeholder
            builder.with_cellular_automata(45, 4, rng);
        }
    }
    
    builder.build()
}

fn select_styled_tile(
    mapgen_tile: MapGenTileType,
    biome: &BiomeType,
    mappings: &TileMappings,
    rng: &mut ChaCha8Rng,
    map: &Map,
    x: u32,
    y: u32,
) -> (u32, bool) {
    // Check for transitions
    if is_transition_tile(map, x, y) {
        return select_transition_tile(map, x, y, mappings, biome);
    }
    
    // Select base tile with variation
    match mapgen_tile {
        MapGenTileType::Floor => {
            let variations = &mappings.floor_tiles;
            let idx = rng.gen_range(0..variations.len());
            (variations[idx], false)
        }
        MapGenTileType::Wall => {
            let variations = &mappings.wall_tiles;
            let idx = rng.gen_range(0..variations.len());
            (variations[idx], false)
        }
        MapGenTileType::Water => {
            // Water is animated
            (mappings.water_tiles[0], true)
        }
        MapGenTileType::Exit => {
            // Use special tile for exits
            (mappings.special_tiles[0], false)
        }
        _ => {
            // Default to floor
            (mappings.floor_tiles[0], false)
        }
    }
}

fn is_transition_tile(map: &Map, x: u32, y: u32) -> bool {
    let current = map.tiles[(y * map.width as u32 + x) as usize];
    
    // Check adjacent tiles
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 { continue; }
            
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            
            if nx >= 0 && nx < map.width as i32 && ny >= 0 && ny < map.height as i32 {
                let neighbor = map.tiles[(ny * map.width as i32 + nx) as usize];
                if neighbor != current {
                    return true;
                }
            }
        }
    }
    
    false
}

fn select_transition_tile(
    map: &Map,
    x: u32,
    y: u32,
    mappings: &TileMappings,
    biome: &BiomeType,
) -> (u32, bool) {
    // Simplified transition logic - in production this would be more sophisticated
    let current = map.tiles[(y * map.width as u32 + x) as usize];
    
    // Check what we're transitioning between
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 { continue; }
            
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            
            if nx >= 0 && nx < map.width as i32 && ny >= 0 && ny < map.height as i32 {
                let neighbor = map.tiles[(ny * map.width as i32 + nx) as usize];
                
                match (current, neighbor) {
                    (MapGenTileType::Floor, MapGenTileType::Wall) |
                    (MapGenTileType::Wall, MapGenTileType::Floor) => {
                        return (mappings.transition_tiles.floor_to_wall[0], false);
                    }
                    (MapGenTileType::Floor, MapGenTileType::Water) |
                    (MapGenTileType::Water, MapGenTileType::Floor) => {
                        return (mappings.transition_tiles.floor_to_water[0], false);
                    }
                    _ => {}
                }
            }
        }
    }
    
    // Default to base tile if no transition found
    select_styled_tile(current, biome, mappings, &mut rand::thread_rng(), map, x, y)
}

fn add_location_features(
    features: &[crate::world_generation::LocationFeature],
    tile_storage: &mut TileStorage,
    commands: &mut Commands,
    tilemap_entity: Entity,
    mappings: &TileMappings,
    rng: &mut ChaCha8Rng,
) {
    use crate::world_generation::LocationFeature;
    
    for feature in features {
        match feature {
            LocationFeature::BossArena => {
                // Place boss arena marker in center
                let center_x = tile_storage.size.x / 2;
                let center_y = tile_storage.size.y / 2;
                // Would place special boss arena tiles
            }
            LocationFeature::TreasureVault => {
                // Place treasure chests randomly
                for _ in 0..3 {
                    let x = rng.gen_range(5..tile_storage.size.x - 5);
                    let y = rng.gen_range(5..tile_storage.size.y - 5);
                    // Would place treasure tile
                }
            }
            LocationFeature::HealingSpring => {
                // Place healing spring
                let x = rng.gen_range(10..tile_storage.size.x - 10);
                let y = rng.gen_range(10..tile_storage.size.y - 10);
                // Would place healing spring tiles
            }
            _ => {
                // Other features would be implemented similarly
            }
        }
    }
}

/// System to animate tiles
pub fn animate_tiles(
    time: Res<Time>,
    mut query: Query<(&mut TileTextureIndex, &mut AnimatedTile)>,
) {
    for (mut texture_index, mut animated_tile) in query.iter_mut() {
        animated_tile.timer.tick(time.delta());
        
        if animated_tile.timer.finished() {
            animated_tile.current_frame = (animated_tile.current_frame + 1) % animated_tile.frames.len();
            texture_index.0 = animated_tile.frames[animated_tile.current_frame];
        }
    }
}

/// System to apply style transfer to tile textures
pub async fn apply_style_transfer_to_tiles(
    base_tileset: Handle<Image>,
    style_guide: Handle<Image>,
    images: &Assets<Image>,
) -> Result<Handle<Image>, String> {
    // This would integrate with the style transfer system in studio/generator.rs
    // For now, we'll use the base tileset
    Ok(base_tileset)
}

/// Plugin for tilemap generation
pub struct TilemapGenerationPlugin;

impl Plugin for TilemapGenerationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TilemapPlugin)
            .add_systems(Update, animate_tiles);
    }
}