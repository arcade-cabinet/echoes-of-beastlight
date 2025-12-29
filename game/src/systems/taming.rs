// AI Game Generator - Procedural game generation using AI
// Copyright (C) 2024 AI Game Generator Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the MIT License as published by
// the Open Source Initiative.

use bevy::prelude::*;
use rand::Rng;

#[derive(Component)]
pub struct TameablePlayer {
    pub level: u32,
    pub party: Vec<Entity>,
}

#[derive(Component)]
pub struct TameableMonster {
    pub health: u32,
    pub max_health: u32,
}

#[derive(Component)]
pub struct Bait {
    pub success_rate: f32,
}

pub struct TamingPlugin;

impl Plugin for TamingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (taming_system, experience_sharing_system));
    }
}

fn taming_system(
    mut commands: Commands,
    mut player_query: Query<&mut TameablePlayer>,
    monster_query: Query<(Entity, &TameableMonster)>,
    bait_query: Query<&Bait>,
) {
    let mut rng = rand::rng();

    for mut player in player_query.iter_mut() {
        if player.party.len() < 6 {
            for (monster_entity, monster) in monster_query.iter() {
                let taming_chance = (monster.health as f32 / monster.max_health as f32)
                    * (player.level as f32 / 100.0);

                let bait_bonus = bait_query
                    .iter()
                    .fold(0.0, |acc, bait| acc + bait.success_rate);

                let final_chance = taming_chance + bait_bonus;

                if rng.random::<f32>() < final_chance {
                    commands.entity(monster_entity).despawn();
                    player.party.push(monster_entity);
                    break;
                }
            }
        }
    }
}

fn experience_sharing_system(
    player_query: Query<&TameablePlayer>,
    mut monster_query: Query<&mut TameableMonster>,
) {
    for player in player_query.iter() {
        let experience = player.level * 10;

        for monster_entity in &player.party {
            if let Ok(mut monster) = monster_query.get_mut(*monster_entity) {
                monster.health = (monster.health as f32 + experience as f32 * 0.1) as u32;
                if monster.health > monster.max_health {
                    monster.health = monster.max_health;
                }
            }
        }
    }
}
