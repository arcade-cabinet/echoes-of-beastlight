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
use bevy_tilemap::prelude::*;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture_handle = asset_server.load("tiles.png");

    let tilemap = Tilemap::builder()
        .auto_chunk()
        .auto_spawn(2, 2)
        .chunk_dimensions(32, 32)
        .texture_dimensions(32, 32)
        .texture_atlas(texture_handle.clone(), &[Tile{ sprite_index: 0, ..Default::default() }, Tile{ sprite_index: 1, ..Default::default() }, Tile{ sprite_index: 2, ..Default::default() }])
        .layers(vec![
            LayerBuilder::new("terrain", 0),
            LayerBuilder::new("decoration", 1),
            LayerBuilder::new("collision", 2),
        ])
        .z_layers(3)
        .finish()
        .unwrap();

    commands.spawn().insert(tilemap).insert(Transform::from_xyz(0.0, 0.0, 0.0)).insert(GlobalTransform::default());
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(TilemapPlugin)
        .add_startup_system(setup.system())
        .run();
}
