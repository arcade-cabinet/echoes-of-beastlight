// AI Game Generator - Procedural game generation using AI
// Copyright (C) 2024 AI Game Generator Contributors

use crate::components::{Position, Velocity};
use bevy::prelude::*;

pub fn movement_system(mut query: Query<(&mut Position, &Velocity)>, time: Res<Time>) {
    for (mut pos, vel) in query.iter_mut() {
        pos.x += vel.x * time.delta_seconds();
        pos.y += vel.y * time.delta_seconds();
    }
}
