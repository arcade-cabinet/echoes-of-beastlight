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

use anyhow::Result;
use handlebars::Handlebars;
use serde_json::json;

#[derive(Debug)]
pub struct Templates {
    handlebars: Handlebars<'static>,
}

impl Templates {
    pub fn new() -> Self {
        let mut handlebars = Handlebars::new();
        handlebars.register_escape_fn(handlebars::no_escape);
        Self {
            handlebars,
        }
    }

    pub async fn load(&mut self) -> Result<()> {
        // Register built-in templates
        self.register_component_template()?;
        self.register_system_template()?;
        self.register_tilemap_template()?;

        Ok(())
    }

    fn register_component_template(&mut self) -> Result<()> {
        let template = r#"use bevy::prelude::*;

#[derive(Component, Debug, Clone)]
pub struct Player {
    pub health: i32,
    pub mana: i32,
    pub level: u32,
}

#[derive(Component, Debug, Clone)]
pub struct Monster {
    pub species: String,
    pub health: i32,
    pub damage: i32,
    pub tameable: bool,
}

#[derive(Component, Debug, Clone)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Component, Debug, Clone)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component, Debug, Clone)]
pub struct Name(pub String);

#[derive(Component, Debug, Clone)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

#[derive(Component, Debug, Clone)]
pub struct Damage(pub i32);

#[derive(Component, Debug, Clone)]
pub struct Tameable {
    pub base_chance: f32,
    pub is_tamed: bool,
}

// Inspector support
#[cfg(feature = "inspector")]
impl bevy_inspector_egui::Inspectable for Player {
    type Attributes = ();

    fn ui(&mut self, ui: &mut egui::Ui, _: Self::Attributes, _: &mut bevy_inspector_egui::Context) -> bool {
        let mut changed = false;
        changed |= ui.add(egui::DragValue::new(&mut self.health).prefix("Health: ")).changed();
        changed |= ui.add(egui::DragValue::new(&mut self.mana).prefix("Mana: ")).changed();
        changed |= ui.add(egui::DragValue::new(&mut self.level).prefix("Level: ")).changed();
        changed
    }
}"#;

        self.handlebars.register_template_string("components", template)?;
        Ok(())
    }

    fn register_system_template(&mut self) -> Result<()> {
        let template = r#"use bevy::prelude::*;
use crate::components::*;

pub fn {{system_name}}_system(
    {{#each queries}}
    {{this.name}}: Query<{{this.types}}>,
    {{/each}}
) {
    {{body}}
}"#;

        self.handlebars.register_template_string("system", template)?;
        Ok(())
    }

    fn register_tilemap_template(&mut self) -> Result<()> {
        let template = r#"use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

pub fn setup_{{zone_name}}_tilemap(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let texture_handle = asset_server.load("sprites/{{zone_name}}_tiles.png");

    let map_size = TilemapSize { x: 32, y: 32 };
    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(map_size);

    // Generate tiles
    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: TileTextureIndex(0), // TODO: vary based on terrain
                    ..Default::default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size: TilemapGridSize { x: 16.0, y: 16.0 },
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size: TilemapTileSize { x: 16.0, y: 16.0 },
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    });
}"#;

        self.handlebars.register_template_string("tilemap", template)?;
        Ok(())
    }

    pub fn render_components(&self) -> Result<String> {
        Ok(self.handlebars.render("components", &json!({}))?)
    }

    pub fn render_system(&self, name: &str, queries: Vec<QueryDef>, body: &str) -> Result<String> {
        let data = json!({
            "system_name": name,
            "queries": queries,
            "body": body
        });

        Ok(self.handlebars.render("system", &data)?)
    }

    pub fn render_tilemap(&self, zone_name: &str) -> Result<String> {
        // Properly sanitize the zone name for Rust function names
        let sanitized_name = zone_name
            .to_lowercase()
            .chars()
            .map(|c| {
                if c.is_alphanumeric() {
                    c
                } else {
                    '_'
                }
            })
            .collect::<String>()
            .trim_matches('_')
            .to_string();

        let data = json!({
            "zone_name": sanitized_name
        });

        Ok(self.handlebars.render("tilemap", &data)?)
    }
}

#[derive(serde::Serialize)]
pub struct QueryDef {
    pub name: String,
    pub types: String,
}

#[cfg(test)]
mod tests {
    use super::*;


    #[tokio::test]
    async fn test_templates_new() {
        let templates = Templates::new();
        // Should create without errors
        assert!(format!("{:?}", templates).contains("Templates"));
    }

    #[tokio::test]
    async fn test_templates_load() {
        let mut templates = Templates::new();
        let result = templates.load().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_render_components() {
        let mut templates = Templates::new();
        templates.load().await.unwrap();

        let result = templates.render_components();
        assert!(result.is_ok());

        let rendered = result.unwrap();

        // Check for key component definitions
        assert!(rendered.contains("pub struct Player"));
        assert!(rendered.contains("pub struct Monster"));
        assert!(rendered.contains("pub struct Position"));
        assert!(rendered.contains("pub struct Velocity"));
        assert!(rendered.contains("pub struct Health"));
        assert!(rendered.contains("pub struct Tameable"));

        // Check for proper derives
        assert!(rendered.contains("#[derive(Component, Debug, Clone)]"));

        // Check for fields
        assert!(rendered.contains("pub health: i32"));
        assert!(rendered.contains("pub mana: i32"));
        assert!(rendered.contains("pub level: u32"));
        assert!(rendered.contains("pub species: String"));
        assert!(rendered.contains("pub tameable: bool"));

        // Check for inspector support
        assert!(rendered.contains("#[cfg(feature = \"inspector\")]"));
        assert!(rendered.contains("impl bevy_inspector_egui::Inspectable for Player"));
    }

    #[tokio::test]
    async fn test_render_system_simple() {
        let mut templates = Templates::new();
        templates.load().await.unwrap();

        let queries = vec![
            QueryDef {
                name: "mut player_query".to_string(),
                types: "&mut Transform, &Player".to_string(),
            }
        ];

        let body = "// System logic here";
        let result = templates.render_system("movement", queries, body);

        assert!(result.is_ok());
        let rendered = result.unwrap();

        println!("Rendered system template:\n{}", rendered);

        assert!(rendered.contains("pub fn movement_system"));
        assert!(rendered.contains("mut player_query: Query<&mut Transform, &Player>"));
        assert!(rendered.contains("// System logic here"));
    }

    #[tokio::test]
    async fn test_render_system_multiple_queries() {
        let mut templates = Templates::new();
        templates.load().await.unwrap();

        let queries = vec![
            QueryDef {
                name: "player_query".to_string(),
                types: "&Transform, &Player".to_string(),
            },
            QueryDef {
                name: "monster_query".to_string(),
                types: "&Transform, &Monster".to_string(),
            },
            QueryDef {
                name: "mut health_query".to_string(),
                types: "&mut Health".to_string(),
            }
        ];

        let body = r#"
    for (transform, player) in player_query.iter() {
        // Player logic
    }

    for (transform, monster) in monster_query.iter() {
        // Monster logic
    }
"#;

        let result = templates.render_system("combat", queries, body);
        assert!(result.is_ok());

        let rendered = result.unwrap();
        assert!(rendered.contains("pub fn combat_system"));
        assert!(rendered.contains("player_query: Query<&Transform, &Player>"));
        assert!(rendered.contains("monster_query: Query<&Transform, &Monster>"));
        assert!(rendered.contains("mut health_query: Query<&mut Health>"));
        assert!(rendered.contains("for (transform, player) in player_query.iter()"));
    }

    #[tokio::test]
    async fn test_render_tilemap() {
        let mut templates = Templates::new();
        templates.load().await.unwrap();

        let result = templates.render_tilemap("Forest Zone");
        assert!(result.is_ok());

        let rendered = result.unwrap();

        // Check function name is properly formatted
        assert!(rendered.contains("pub fn setup_forest_zone_tilemap"));

        // Check asset path is properly formatted
        assert!(rendered.contains("sprites/forest_zone_tiles.png"));

        // Check for tilemap setup code
        assert!(rendered.contains("TilemapSize { x: 32, y: 32 }"));
        assert!(rendered.contains("TileStorage::empty(map_size)"));
        assert!(rendered.contains("TileBundle"));
        assert!(rendered.contains("TilemapBundle"));

        // Check for proper grid and tile sizes
        assert!(rendered.contains("TilemapGridSize { x: 16.0, y: 16.0 }"));
        assert!(rendered.contains("TilemapTileSize { x: 16.0, y: 16.0 }"));
    }

    #[tokio::test]
    async fn test_render_tilemap_special_characters() {
        let mut templates = Templates::new();
        templates.load().await.unwrap();

        // Test with zone names containing special characters
        let test_cases = vec![
            ("Dark-Cave", "setup_dark_cave_tilemap"),
            ("Boss's Lair", "setup_boss_s_lair_tilemap"),
            ("Zone #1", "setup_zone__1_tilemap"),
            ("Multi  Space", "setup_multi__space_tilemap"),
        ];

        for (zone_name, expected_fn) in test_cases {
            let result = templates.render_tilemap(zone_name);
            assert!(result.is_ok());
            let rendered = result.unwrap();
            println!("Zone: '{}' -> Expected: '{}'\nRendered:\n{}", zone_name, expected_fn, rendered);
            assert!(rendered.contains(&format!("pub fn {}", expected_fn)));
        }
    }

    #[test]
    fn test_query_def_serialization() {
        let query_def = QueryDef {
            name: "test_query".to_string(),
            types: "&Transform, &Player".to_string(),
        };

        let json = serde_json::to_string(&query_def).unwrap();
        assert!(json.contains("\"name\":\"test_query\""));
        assert!(json.contains("\"types\":\"&Transform, &Player\""));
    }

    #[tokio::test]
    async fn test_handlebars_error_handling() {
        let mut templates = Templates::new();
        // Don't load templates

        let result = templates.render_components();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Template not found"));
    }

    #[tokio::test]
    async fn test_system_template_edge_cases() {
        let mut templates = Templates::new();
        templates.load().await.unwrap();

        // Test with empty queries
        let queries = vec![];
        let result = templates.render_system("empty", queries, "// No queries");
        assert!(result.is_ok());
        let rendered = result.unwrap();
        assert!(rendered.contains("pub fn empty_system("));
        assert!(rendered.contains(") {"));

        // Test with complex query types
        let queries = vec![
            QueryDef {
                name: "complex_query".to_string(),
                types: "(Entity, &Transform, &Player, Option<&Health>), With<Active>".to_string(),
            }
        ];
        let result = templates.render_system("complex", queries, "// Complex");
        assert!(result.is_ok());
        let rendered = result.unwrap();
        assert!(rendered.contains("complex_query: Query<(Entity, &Transform, &Player, Option<&Health>), With<Active>>"));
    }
}
