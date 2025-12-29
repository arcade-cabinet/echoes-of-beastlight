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

mod config;
mod world;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Echoes of Beastlight".to_string(),
                    resolution: (1280, 720).into(),
                    canvas: Some("#bevy-canvas".to_string()),
                    ..default()
                }),
                ..default()
            }),
            config::ConfigPlugin,
            world::WorldPlugin,
        ))
        .add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
