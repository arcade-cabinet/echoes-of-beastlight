// AI Game Generator - Procedural game generation using AI
// Copyright (C) 2024 AI Game Generator Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the MIT License as published by
// the Open Source Initiative.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_tilemap::Tile;

// Tile type enum
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum GlintrockTile {
    CrystalGrass,
    Stone,
    GlowingMoss,
}

impl Default for GlintrockTile {
    fn default() -> Self {
        Self::Stone
    }
}

impl Tile for GlintrockTile {
    fn sprite(
        &self,
        _: &mut bevy::render::renderer::RenderContext,
        _: &mut bevy::render::renderer::RenderResources,
    ) -> Option<usize> {
        match self {
            Self::CrystalGrass => Some(0),
            Self::Stone => Some(1),
            Self::GlowingMoss => Some(2),
        }
    }
}

// Tilemap setup function
pub fn setup_tilemap(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut tilemap_res: ResMut<TilemapResource<GlintrockTile>>,
) {
    let texture_handle = asset_server.load("textures/glintrock_tiles.png");
    let tilemap = Tilemap::builder()
        .texture_atlas(texture_handle)
        .dimensions(32, 32)
        .chunk_dimensions(32, 32)
        .tile_dimensions(16, 16)
        .auto_chunk()
        .auto_spawn(2, 2)
        .add_layer(
            TilemapLayer {
                kind: LayerKind::Dense,
                ..Default::default()
            },
            0,
        )
        .add_layer(
            TilemapLayer {
                kind: LayerKind::Sparse,
                ..Default::default()
            },
            1,
        )
        .add_layer(
            TilemapLayer {
                kind: LayerKind::Sparse,
                ..Default::default()
            },
            2,
        )
        .z_layers(3)
        .finish()
        .unwrap();

    commands.spawn().insert_bundle(tilemap_res.insert(tilemap));
}

// Helper functions
/// Spawns a tile at the given position.
pub fn spawn_tile(
    tile: GlintrockTile,
    x: u32,
    y: u32,
    z: u32,
    tilemap: &mut Tilemap<GlintrockTile>,
) {
    tilemap.insert_tile((x, y, z), tile).unwrap();
}

/// Checks if there is a collision at the given coordinates.
pub fn check_collision(x: u32, y: u32, tilemap: &Tilemap<GlintrockTile>) -> bool {
    tilemap.get_tile((x, y, 2), true).is_some()
}

/// Updates the graphics of a tile at the given position.
pub fn update_tile_graphics(
    x: u32,
    y: u32,
    z: u32,
    tile: GlintrockTile,
    tilemap: &mut Tilemap<GlintrockTile>,
) {
    tilemap.insert_tile((x, y, z), tile).unwrap();
}

/// Handles the loading and unloading of chunks.
pub fn handle_chunk_loading_unloading(
    camera_transform: &Transform,
    tilemap: &mut Tilemap<GlintrockTile>,
) {
    let center = camera_transform.translation;
    tilemap.auto_spawn(center, 2, 2);
}
