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

// Tile types for the Forgotten Roots zone
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum ForgottenRootsTile {
    AncientStone,
    RootWalls,
    TechnoRuins,
}

impl Default for ForgottenRootsTile {
    fn default() -> Self {
        Self::AncientStone
    }
}

// Tilemap layers
pub enum ForgottenRootsLayer {
    Floor,
    Walls,
    Tech,
}

impl Tile for ForgottenRootsTile {
    fn sprite(&self, coords: TilePos, _: &World) -> Option<usize> {
        match self {
            ForgottenRootsTile::AncientStone => Some(0),
            ForgottenRootsTile::RootWalls => Some(1),
            ForgottenRootsTile::TechnoRuins => Some(2),
        }
    }
}

pub fn setup_tilemap(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Load the texture atlas for the tilemap
    let texture_handle = asset_server.load("textures/forgotten_roots_tiles.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 3, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // Create the tilemap layers
    let floor_layer = LayerBuilder::<ForgottenRootsTile>::new(
        &mut commands,
        LayerSettings::new(
            MapSize(32, 32),
            ChunkSize(32, 32),
            TextureAtlasSettings::default(),
            0, // z-index for floor layer
        ),
    );

    let walls_layer = LayerBuilder::<ForgottenRootsTile>::new(
        &mut commands,
        LayerSettings::new(
            MapSize(32, 32),
            ChunkSize(32, 32),
            TextureAtlasSettings::default(),
            1, // z-index for walls layer
        ),
    );

    let tech_layer = LayerBuilder::<ForgottenRootsTile>::new(
        &mut commands,
        LayerSettings::new(
            MapSize(32, 32),
            ChunkSize(32, 32),
            TextureAtlasSettings::default(),
            2, // z-index for tech layer
        ),
    );

    // Create the tilemap
    let tilemap = TilemapBundle::builder()
        .add_layer(floor_layer)
        .add_layer(walls_layer)
        .add_layer(tech_layer)
        .texture_atlas(texture_atlas_handle)
        .finish_bundle();

    commands.spawn().insert_bundle(tilemap);
}

// Helper functions

/// Spawn a tile at the specified position in the specified layer.
pub fn spawn_tile(layer: &mut Layer<TileBundle>, tile: ForgottenRootsTile, pos: TilePos) {
    layer.spawn(tile, pos);
}

/// Check if a tile at the specified position is solid.
pub fn is_solid(tile: ForgottenRootsTile) -> bool {
    match tile {
        ForgottenRootsTile::RootWalls => true,
        _ => false,
    }
}

/// Update the graphics of a tile at the specified position.
pub fn update_tile_graphics(layer: &mut Layer<TileBundle>, pos: TilePos, tile: ForgottenRootsTile) {
    layer.set_tile(pos, tile);
}

/// Handle the loading and unloading of chunks based on the player's position.
pub fn handle_chunk_loading(player_pos: Vec2, map_query: &mut MapQuery<TileBundle>) {
    map_query.maintain(player_pos, 2);
}
