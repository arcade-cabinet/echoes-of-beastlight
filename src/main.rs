use bevy::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;
use bevy_egui::EguiPlugin;
#[cfg(debug_assertions)]
use bevy_yoleck::YoleckPluginForEditor;

// Define game states
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Menu,
    Overworld,
    Battle,
    Shop,
    Dungeon,
    Inventory,
}

fn main() {
    App::build()
        // Set window configuration
        .insert_resource(WindowDescriptor {
            title: "".to_string(),
            width: 800.,
            height: 600.,
            ..Default::default()
        })
        // Add plugins
        .add_plugins(DefaultPlugins)
        .add_plugin(TilemapPlugin)
        .add_plugin(EguiPlugin)
        #[cfg(debug_assertions)]
        .add_plugin(YoleckPluginForEditor)
        // Add game state
        .add_state(GameState::Menu)
        // Add systems
        .add_system_set(SystemSet::on_update(GameState::Menu).with_system(menu_system.system()))
        .add_system_set(SystemSet::on_update(GameState::Overworld).with_system(overworld_system.system()))
        .add_system_set(SystemSet::on_update(GameState::Battle).with_system(battle_system.system()))
        .add_system_set(SystemSet::on_update(GameState::Shop).with_system(shop_system.system()))
        .add_system_set(SystemSet::on_update(GameState::Dungeon).with_system(dungeon_system.system()))
        .add_system_set(SystemSet::on_update(GameState::Inventory).with_system(inventory_system.system()))
        .run();
}

// Define your systems here
fn menu_system() {
    // TODO: Implement menu system
}

fn overworld_system() {
    // TODO: Implement overworld system
}

fn battle_system() {
    // TODO: Implement battle system
}

fn shop_system() {
    // TODO: Implement shop system
}

fn dungeon_system() {
    // TODO: Implement dungeon system
}

fn inventory_system() {
    // TODO: Implement inventory system
}