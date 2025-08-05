use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_yoleck::prelude::*;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Menu,
    Overworld,
    Battle,
    Shop,
    Dungeon,
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(TilemapPlugin)
        .add_plugin(YoleckPlugin)
        .add_state(GameState::Menu)
        .add_startup_system(setup.system())
        .add_system_set(
            SystemSet::on_update(GameState::Overworld)
                .with_system(overworld_system.system())
        )
        .add_system_set(
            SystemSet::on_update(GameState::Battle)
                .with_system(battle_system.system())
        )
        .add_system_set(
            SystemSet::on_update(GameState::Shop)
                .with_system(shop_system.system())
        )
        .add_system_set(
            SystemSet::on_update(GameState::Dungeon)
                .with_system(dungeon_system.system())
        )
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    // TODO: Load and initialize your assets here
}

fn overworld_system() {
    // TODO: Implement your overworld logic here
}

fn battle_system() {
    // TODO: Implement your battle logic here
}

fn shop_system() {
    // TODO: Implement your shop logic here
}

fn dungeon_system() {
    // TODO: Implement your dungeon logic here
}