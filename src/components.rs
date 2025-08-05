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


#[derive(Component, Debug, Clone)]
pub struct Player {
    pub health: i32,
    pub mana: i32,
    pub level: u32,
}

impl Player {
    pub fn new(health: i32, mana: i32, level: u32) -> Self {
        Self { health, mana, level }
    }
}

#[derive(Component, Debug, Clone)]
pub struct Monster {
    pub species: String,
    pub health: i32,
    pub damage: i32,
    pub tameable: bool,
}

impl Monster {
    pub fn new(species: String, health: i32, damage: i32, tameable: bool) -> Self {
        Self { species, health, damage, tameable }
    }
}

#[derive(Component, Debug, Clone)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}
