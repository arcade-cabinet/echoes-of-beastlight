use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use serde::{Deserialize, Serialize};

/// Everything in the game is conceptually a "tile" that can be composed
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct GameTile {
    pub tile_type: TileType,
    pub base_id: u32,
    pub recolor: Option<Color>,
    pub corruption_level: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TileType {
    // Map tiles
    Terrain(TerrainType),
    Structure(StructureType),
    Transition(TransitionType),

    // Entity tiles (conceptual)
    MonsterPart(MonsterPartType),
    ItemVisual(ItemVisualType),
    EffectLayer(EffectType),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TerrainType {
    Grass,
    Sand,
    Stone,
    Water,
    DeepWater,
    Mountain,
    Forest,
    Swamp,
    Corrupted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StructureType {
    DungeonEntrance,
    ShopIcon,
    Town,
    Bridge,
    Ruins,
    Shrine,
    Portal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TransitionType {
    MapEdgeNorth,
    MapEdgeSouth,
    MapEdgeEast,
    MapEdgeWest,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MonsterPartType {
    Body,
    Head,
    Wings,
    Tail,
    Aura,
    Eyes,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ItemVisualType {
    Weapon,
    Armor,
    Accessory,
    Consumable,
    QuestItem,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EffectType {
    Glow,
    Particle,
    Trail,
    Aura,
}

/// Resource for managing tile recoloring based on corruption
#[derive(Resource)]
pub struct TileRecolorSystem {
    pub corruption_gradient: Vec<(f32, Color)>,
    pub element_colors: HashMap<String, Color>,
}

impl Default for TileRecolorSystem {
    fn default() -> Self {
        use crate::config::style::StyleConfig;
        let style = StyleConfig::default();

        Self {
            corruption_gradient: vec![
                (0.0, style.visual.palette.primary_bright),
                (0.2, style.visual.palette.primary_light),
                (0.5, style.visual.palette.corruption_mid),
                (0.8, style.visual.palette.corruption_glow),
                (1.0, style.visual.palette.corruption_dark),
            ],
            element_colors: HashMap::from([
                ("fire".to_string(), Color::hex("#ff6b6b").unwrap()),
                ("water".to_string(), Color::hex("#4ecdc4").unwrap()),
                ("earth".to_string(), Color::hex("#8b6914").unwrap()),
                ("air".to_string(), Color::hex("#95e1d3").unwrap()),
                ("light".to_string(), Color::hex("#ffffff").unwrap()),
                ("shadow".to_string(), Color::hex("#2c2c2c").unwrap()),
            ]),
        }
    }
}

/// Component for tile-based monsters (composed of multiple tiles)
#[derive(Component, Debug, Clone)]
pub struct MonsterTileComposition {
    pub parts: Vec<MonsterTilePart>,
    pub base_size: TileSize,
}

#[derive(Debug, Clone)]
pub struct MonsterTilePart {
    pub part_type: MonsterPartType,
    pub tile_index: u32,
    pub offset: Vec2,
    pub layer: f32,
    pub animate: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum TileSize {
    Small,  // 1x1 tiles (16x16 px)
    Medium, // 2x2 tiles (32x32 px)
    Large,  // 3x3 tiles (48x48 px)
    Huge,   // 4x4 tiles (64x64 px)
}

/// System to recolor tiles based on corruption
pub fn recolor_tiles_by_corruption(
    mut query: Query<(&GameTile, &mut TileTextureIndex, &mut TileColor), Changed<GameTile>>,
    recolor_system: Res<TileRecolorSystem>,
) {
    for (game_tile, mut _texture, mut color) in query.iter_mut() {
        if let Some(recolor) = game_tile.recolor {
            color.0 = recolor;
        } else {
            // Apply corruption-based recoloring
            let corruption = game_tile.corruption_level.clamp(0.0, 1.0);

            // Find the two colors to interpolate between
            let mut lower_color = recolor_system.corruption_gradient[0].1;
            let mut upper_color = recolor_system.corruption_gradient[0].1;
            let mut t = 0.0;

            for i in 0..recolor_system.corruption_gradient.len() - 1 {
                let (lower_threshold, lower) = recolor_system.corruption_gradient[i];
                let (upper_threshold, upper) = recolor_system.corruption_gradient[i + 1];

                if corruption >= lower_threshold && corruption <= upper_threshold {
                    lower_color = lower;
                    upper_color = upper;
                    t = (corruption - lower_threshold) / (upper_threshold - lower_threshold);
                    break;
                }
            }

            // Interpolate between colors
            color.0 = Color::rgba(
                lower_color.r() + (upper_color.r() - lower_color.r()) * t,
                lower_color.g() + (upper_color.g() - lower_color.g()) * t,
                lower_color.b() + (upper_color.b() - lower_color.b()) * t,
                lower_color.a() + (upper_color.a() - lower_color.a()) * t,
            );
        }
    }
}

/// Generate a tile index based on seed and context
pub fn generate_tile_variant(
    seed: &crate::world::seed::WorldSeed,
    tile_type: TileType,
    position: IVec2,
) -> u32 {
    let context = format!("{:?}-{}-{}", tile_type, position.x, position.y);
    let base_variants = match tile_type {
        TileType::Terrain(TerrainType::Grass) => 4,
        TileType::Terrain(TerrainType::Stone) => 3,
        TileType::Terrain(TerrainType::Water) => 2,
        _ => 1,
    };

    (seed.get_value(&context, base_variants) as u32) + get_base_tile_index(tile_type)
}

fn get_base_tile_index(tile_type: TileType) -> u32 {
    match tile_type {
        TileType::Terrain(t) => match t {
            TerrainType::Grass => 0,
            TerrainType::Sand => 10,
            TerrainType::Stone => 20,
            TerrainType::Water => 30,
            TerrainType::DeepWater => 35,
            TerrainType::Mountain => 40,
            TerrainType::Forest => 50,
            TerrainType::Swamp => 60,
            TerrainType::Corrupted => 70,
        },
        TileType::Structure(s) => match s {
            StructureType::DungeonEntrance => 100,
            StructureType::ShopIcon => 110,
            StructureType::Town => 120,
            StructureType::Bridge => 130,
            StructureType::Ruins => 140,
            StructureType::Shrine => 150,
            StructureType::Portal => 160,
        },
        TileType::Transition(_) => 200,
        TileType::MonsterPart(_) => 300,
        TileType::ItemVisual(_) => 400,
        TileType::EffectLayer(_) => 500,
    }
}

pub struct TilePlugin;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TilemapPlugin)
            .insert_resource(TileRecolorSystem::default())
            .add_systems(Update, recolor_tiles_by_corruption);
    }
}
