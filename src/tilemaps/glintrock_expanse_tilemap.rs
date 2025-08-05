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

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut map_query: MapQuery,
    mut materials: ResMut<Assets<TilemapDefaultMaterial>>,
) {
    // Load the textures
    let crystal_grass_texture = asset_server.load("textures/crystal_grass.png");
    let stone_texture = asset_server.load("textures/stone.png");
    let glowing_moss_texture = asset_server.load("textures/glowing_moss.png");

    // Create a new tilemap layer
    let tilemap = Tilemap::builder()
        .texture_dimensions(32, 32)
        .chunk_dimensions(32, 32, 1)
        .tile_dimensions(32, 32)
        .dimensions(10, 10)
        .add_layer(TilemapLayer { kind: LayerKind::Dense, z_index: 0 }, "terrain")
        .add_layer(TilemapLayer { kind: LayerKind::Sparse, z_index: 1 }, "crystals")
        .add_layer(TilemapLayer { kind: LayerKind::Sparse, z_index: 2 }, "collision")
        .default_chunk_material(materials.add(TilemapDefaultMaterial::default()))
        .texture_atlas(Handle::from_untyped(asset_server.load("textures/tileset.atlas")))
        .finish()
        .unwrap();

    // Spawn the tilemap components
    commands.spawn().insert_bundle(tilemap.to_components());
}
