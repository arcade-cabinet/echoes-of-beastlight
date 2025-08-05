use bevy::prelude::*;

// Player component
pub struct Player {
    pub name: String,
    pub level: u32,
}

// Monster component
pub struct Monster {
    pub name: String,
    pub level: u32,
}

// Tile component
pub struct Tile {
    pub x: u32,
    pub y: u32,
    pub walkable: bool,
}

// Stats component
pub struct Stats {
    pub health: u32,
    pub strength: u32,
    pub defense: u32,
}

// Position component
pub struct Position {
    pub x: f32,
    pub y: f32,
}