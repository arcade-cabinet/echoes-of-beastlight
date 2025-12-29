// AI Game Generator - Procedural game generation using AI
// Copyright (C) 2024 AI Game Generator Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the MIT License as published by
// the Open Source Initiative.

use bevy::prelude::*;
use rand::Rng;

#[derive(Component)]
pub struct Character {
    pub hp: i32,
    pub attack: i32,
    pub defense: i32,
    pub crit_chance: f32,
    pub status: Status,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Status {
    Normal,
    Poisoned,
    Stunned,
}

#[derive(Event, Message)]
pub struct AttackEvent {
    pub attacker: Entity,
    pub defender: Entity,
}

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<AttackEvent>()
            .add_systems(Update, (attack, calculate_damage));
    }
}

fn attack(mut event_writer: MessageWriter<AttackEvent>, query: Query<Entity, With<Character>>) {
    let mut entities = query.iter();
    if let (Some(attacker), Some(defender)) = (entities.next(), entities.next()) {
        event_writer.write(AttackEvent { attacker, defender });
    }
}

fn calculate_damage(
    mut event_reader: MessageReader<AttackEvent>,
    mut characters: Query<&mut Character>,
) {
    for event in event_reader.read() {
        let attacker_stats = if let Ok(c) = characters.get(event.attacker) {
            (c.attack, c.crit_chance)
        } else {
            continue;
        };

        if let Ok(mut defender) = characters.get_mut(event.defender) {
            let mut damage = attacker_stats.0 - defender.defense;
            if damage < 0 {
                damage = 0;
            }

            let mut rng = rand::rng();
            if rng.random::<f32>() < attacker_stats.1 {
                damage *= 2;
            }

            match defender.status {
                Status::Poisoned => damage += 5,
                Status::Stunned => damage += 10,
                _ => (),
            }

            defender.hp -= damage;
        }
    }
}
