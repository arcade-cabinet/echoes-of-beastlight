use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

// Tile type enum
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum CratervineChasmTile {
    ChasmEdge,
    VineBridge,
    DarkStone,
}

impl Default for CratervineChasmTile {
    fn default() -> Self {
        Self::DarkStone
    }
}

impl Tile for CratervineChasmTile {
    fn sprite(&self, coords: TilePos, _: &World) -> Option<usize> {
        match self {
            Self::ChasmEdge => Some(0),
            Self::VineBridge => Some(1),
            Self::DarkStone => Some(2),
        }
    }

    fn tint(&self, _: TilePos, _: &World) -> Color {
        match self {
            Self::ChasmEdge => Color::rgb(0.5, 0.5, 0.5),
            Self::VineBridge => Color::rgb(0.5, 1.0, 0.5),
            Self::DarkStone => Color::rgb(0.25, 0.25, 0.25),
        }
    }
}

// TilemapBundle setup function
pub fn setup_tilemap(
    commands: &mut Commands,
    assets: &mut Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = assets.load("textures/cratervine_chasm.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 3, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let map = TilemapBundle::default()
        .texture_atlas(texture_atlas_handle)
        .chunk_dimensions(32, 32)
        .tile_dimensions(32, 32)
        .dimensions(32, 32)
        .add_layer(TilemapLayer { kind: LayerKind::Dense, z_order: 0 }, 0)
        .add_layer(TilemapLayer { kind: LayerKind::Sparse, z_order: 1 }, 1)
        .add_layer(TilemapLayer { kind: LayerKind::Sparse, z_order: 2 }, 2);

    commands.spawn_bundle(map);
}

// Helper functions
pub fn spawn_tile(map_query: &mut MapQuery, pos: TilePos, tile: CratervineChasmTile, layer_id: u16) {
    map_query.set_tile(pos, tile, layer_id).unwrap();
}

pub fn check_collision(map_query: &MapQuery, pos: TilePos, layer_id: u16) -> bool {
    map_query.get_tile(pos, layer_id).is_some()
}

pub fn update_tile_graphics(map_query: &mut MapQuery, pos: TilePos, tile: CratervineChasmTile, layer_id: u16) {
    map_query.set_tile(pos, tile, layer_id).unwrap();
}

pub fn handle_chunk_loading(map_query: &mut MapQuery, pos: ChunkPos) {
    map_query.create_chunk(pos).unwrap();
}

pub fn handle_chunk_unloading(map_query: &mut MapQuery, pos: ChunkPos) {
    map_query.remove_chunk(pos).unwrap();
}