use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

// Tile variants
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum TileType {
    Grass,
    Flowers,
    DirtPath,
}

// Layer identifiers
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum LayerType {
    Terrain,
    Decoration,
    Collision,
}

// Tilemap setup function
pub fn setup_tilemap(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Load textures and create texture atlas
    let texture_handle = asset_server.load("textures/verdant_flats_tiles.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 3, 3);
    let texture_atlas_handle = commands.insert_resource(texture_atlas);

    // Create tilemap with specified chunk size
    let tilemap = Tilemap::builder()
        .texture_atlas(texture_atlas_handle)
        .chunk_dimensions(32, 32, 1)
        .tile_dimensions(32, 32)
        .dimensions(10, 10)
        .add_layer(TilemapLayer { kind: LayerKind::Dense }, 0)
        .add_layer(TilemapLayer { kind: LayerKind::Sparse }, 1)
        .add_layer(TilemapLayer { kind: LayerKind::Sparse }, 2)
        .build();

    // Spawn tilemap bundle
    commands.spawn_bundle(TilemapBundle {
        tilemap,
        visible: Visible {
            is_visible: true,
            is_transparent: true,
        },
        transform: Default::default(),
        global_transform: Default::default(),
    });
}

// Helper functions
pub fn spawn_tile(
    mut commands: Commands,
    tilemap_query: Query<&Handle<Tilemap>>,
    position: UVec2,
    tile: TileType,
    layer: LayerType,
) {
    let tilemap_handle = tilemap_query.single().unwrap();
    let tile_index = match tile {
        TileType::Grass => 0,
        TileType::Flowers => 1,
        TileType::DirtPath => 2,
    };
    let tile = Tile {
        point: position,
        sprite_order: tile_index,
        sprite_index: 0,
        tint: Color::WHITE,
    };
    commands.entity(tilemap_handle.id()).insert_bundle((layer, tile));
}

pub fn check_collision(
    tilemap_query: Query<&Handle<Tilemap>>,
    position: UVec2,
) -> bool {
    let tilemap_handle = tilemap_query.single().unwrap();
    let collision_layer = tilemap_handle.get_layer(&LayerType::Collision).unwrap();
    collision_layer.get_tile(position).is_some()
}

pub fn update_tile_graphics(
    mut commands: Commands,
    tilemap_query: Query<&Handle<Tilemap>>,
    position: UVec2,
    tile: TileType,
) {
    let tilemap_handle = tilemap_query.single().unwrap();
    let tile_index = match tile {
        TileType::Grass => 0,
        TileType::Flowers => 1,
        TileType::DirtPath => 2,
    };
    commands.entity(tilemap_handle.id()).insert((position, tile_index));
}

pub fn handle_chunk_loading(
    mut commands: Commands,
    tilemap_query: Query<&Handle<Tilemap>>,
    chunk_position: UVec2,
) {
    let tilemap_handle = tilemap_query.single().unwrap();
    commands.entity(tilemap_handle.id()).insert(chunk_position);
}

pub fn handle_chunk_unloading(
    mut commands: Commands,
    tilemap_query: Query<&Handle<Tilemap>>,
    chunk_position: UVec2,
) {
    let tilemap_handle = tilemap_query.single().unwrap();
    commands.entity(tilemap_handle.id()).remove(chunk_position);
}