use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

// Tile type enum
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum TileType {
    Ash,
    DeadTrees,
    LavaCracks,
}

impl Default for TileType {
    fn default() -> Self {
        Self::Ash
    }
}

// Tilemap configuration
pub struct TilemapConfig {
    pub chunk_size: UVec2,
    pub tile_size: UVec2,
    pub map_size: UVec2,
    pub texture_atlas: Handle<TextureAtlas>,
}

// Setup function for TilemapBundle
pub fn setup_tilemap(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
    config: Res<TilemapConfig>,
) {
    let texture_handle = asset_server.load("tiles/ashbarrow_wastes.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, config.tile_size, 3, 3);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let tilemap = Tilemap::builder()
        .topology(GridTopology::Square)
        .dimensions(config.map_size)
        .chunk_dimensions(config.chunk_size)
        .texture_dimensions(config.tile_size)
        .add_layer(TilemapLayer { kind: LayerKind::Dense }, "terrain", 0)
        .add_layer(TilemapLayer { kind: LayerKind::Sparse }, "hazards", 1)
        .add_layer(TilemapLayer { kind: LayerKind::Sparse }, "collision", 2)
        .texture_atlas(texture_atlas_handle.clone())
        .finish()
        .unwrap();

    commands.spawn().insert_bundle(TilemapBundle {
        tilemap,
        texture_atlas: texture_atlas_handle,
        visible: Visible {
            is_visible: true,
            is_transparent: true,
        },
        transform: Default::default(),
    });
}

// Helper functions
/// Spawn a tile at a given position
pub fn spawn_tile(
    tilemap: &mut Tilemap,
    position: UVec2,
    tile: Tile<TileType>,
    layer_id: u16,
) -> Result<(), bevy_ecs_tilemap::TilemapError> {
    tilemap.insert_tile(TileBundle {
        tile,
        coordinate: position,
        sprite_order: SpriteOrder(layer_id),
    })
}

/// Check if a tile at a given position is collidable
pub fn is_collidable(tilemap: &Tilemap, position: UVec2) -> bool {
    match tilemap.get_tile(position, 2) {
        Some(tile) => tile.tile.into().is_some(),
        None => false,
    }
}

/// Update a tile's graphic
pub fn update_tile_graphic(tilemap: &mut Tilemap, position: UVec2, new_tile: Tile<TileType>) {
    if let Some(mut tile) = tilemap.get_tile_mut(position, 0) {
        tile.tile = new_tile;
    }
}

/// Handle chunk loading and unloading
pub fn handle_chunk_loading(tilemap: &mut Tilemap, position: UVec2, load: bool) {
    if load {
        tilemap.spawn_chunk_containing_point(position).unwrap();
    } else {
        tilemap.despawn_chunk_containing_point(position).unwrap();
    }
}

// Texture atlas configuration
pub fn setup_texture_atlas(
    asset_server: Res<AssetServer>,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
) -> Handle<TextureAtlas> {
    let texture_handle = asset_server.load("textures/ashbarrow_wastes.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 3, 3);
    texture_atlases.add(texture_atlas)
}

// Palette swapping
pub fn swap_palette(tile: &mut Tile<TileType>, variant: u32) {
    tile.sprite_index = variant;
}