// AI Game Generator - Procedural game generation using AI
// Copyright (C) 2024 AI Game Generator Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the MIT License as published by
// the Open Source Initiative.

use bevy::prelude::*;
use std::collections::HashMap;

// Define the maximum stack size for items
pub const MAX_STACK_SIZE: u32 = 99;

// Define the types of equipment slots available
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EquipmentSlot {
    Head,
    Body,
    Legs,
    Feet,
    Hands,
    Weapon,
}

// Define the types of items available
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ItemType {
    HealthPotion,
    Sword,
    Shield,
}

// Define the effects that items can have when used
#[derive(Debug, Clone)]
pub enum ItemEffect {
    Heal(u32),
    Damage(u32),
    Defense(u32),
}

// Define an Item struct
#[derive(Debug, Clone)]
pub struct Item {
    pub item_type: ItemType,
    pub effect: ItemEffect,
    pub stack: u32,
}

// Define an Inventory struct
#[derive(Component, Debug, Default)]
pub struct Inventory {
    pub items: HashMap<ItemType, Item>,
    pub equipment: HashMap<EquipmentSlot, Item>,
}

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, _app: &mut App) {
        // Add inventory systems here
    }
}
