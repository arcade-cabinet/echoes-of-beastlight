use anyhow::Result;
use handlebars::Handlebars;
use serde_json::json;

#[derive(Debug)]
pub struct Templates {
    handlebars: Handlebars<'static>,
}

impl Templates {
    pub fn new() -> Self {
        Self {
            handlebars: Handlebars::new(),
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
        let data = json!({
            "zone_name": zone_name.to_lowercase().replace(' ', "_")
        });
        
        Ok(self.handlebars.render("tilemap", &data)?)
    }
}

#[derive(serde::Serialize)]
pub struct QueryDef {
    pub name: String,
    pub types: String,
}