mod components;
mod systems;
mod tilemaps;
mod levels;
mod mapgen;
mod world_generation;
mod tilemap_generation;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;
use bevy_yoleck::{YoleckPluginForGame, YoleckLoadLevel};
use bevy_ecs_tilemap::TilemapPlugin;

use components::*;
use systems::*;
use world_generation::{WorldGenerationPlugin, WorldSeed};
use tilemap_generation::TilemapGenerationPlugin;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    MainMenu,
    WorldGeneration,
    Playing,
    Paused,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Echoes of Beastlight".into(),
                resolution: (1280., 720.).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(InputManagerPlugin::<PlayerAction>::default())
        .add_plugins(YoleckPluginForGame)
        .add_plugins(TilemapPlugin)
        .add_plugins(WorldGenerationPlugin)
        .add_plugins(TilemapGenerationPlugin)
        .init_state::<GameState>()
        .add_event::<CombatEvent>()
        .add_event::<TamingEvent>()
        .add_systems(Startup, setup)
        .add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
        .add_systems(Update, main_menu_system.run_if(in_state(GameState::MainMenu)))
        .add_systems(OnEnter(GameState::WorldGeneration), start_world_generation)
        .add_systems(OnExit(GameState::WorldGeneration), finalize_world_generation)
        .add_systems(
            Update,
            (
                movement_system,
                combat_system,
                taming_system,
                inventory_system,
                save_load_system,
            )
            .run_if(in_state(GameState::Playing)),
        )
        .run();
}

fn setup(mut commands: Commands) {
    // Basic 2D camera
    commands.spawn(Camera2dBundle::default());
}

fn setup_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Main menu UI
    commands.spawn(
        TextBundle::from_section(
            "Echoes of Beastlight",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 60.0,
                color: Color::WHITE,
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(100.0),
            left: Val::Px(100.0),
            ..default()
        }),
    );
    
    // Seed input UI
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(300.0),
                left: Val::Px(100.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        },
        MainMenuUI,
    )).with_children(|parent| {
        parent.spawn(
            TextBundle::from_section(
                "Press SPACE for random world",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                    font_size: 30.0,
                    color: Color::WHITE,
                },
            )
        );
        parent.spawn(
            TextBundle::from_section(
                "Press S to enter seed",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                    font_size: 30.0,
                    color: Color::WHITE,
                },
            )
        );
    });
}

#[derive(Component)]
struct MainMenuUI;

fn main_menu_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    menu_query: Query<Entity, With<MainMenuUI>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        // Generate random world
        let world_seed = WorldSeed::new_random();
        info!("Starting new world: {} (seed: {})", world_seed.world_name, world_seed.seed);
        
        commands.insert_resource(world_seed);
        
        // Clean up menu
        for entity in menu_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
        
        next_state.set(GameState::WorldGeneration);
    } else if keyboard.just_pressed(KeyCode::KeyS) {
        // TODO: Show seed input dialog
        // For now, use a hardcoded seed
        let world_seed = WorldSeed::from_seed(12345);
        info!("Starting world: {} (seed: {})", world_seed.world_name, world_seed.seed);
        
        commands.insert_resource(world_seed);
        
        // Clean up menu
        for entity in menu_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
        
        next_state.set(GameState::WorldGeneration);
    }
}

fn start_world_generation(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Show loading screen
    commands.spawn((
        TextBundle::from_section(
            "Generating World...",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 40.0,
                color: Color::WHITE,
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(300.0),
            left: Val::Px(500.0),
            ..default()
        }),
        LoadingText,
    ));
}

#[derive(Component)]
struct LoadingText;

fn finalize_world_generation(
    mut commands: Commands,
    loading_query: Query<Entity, With<LoadingText>>,
    world_seed: Res<WorldSeed>,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // Clean up loading screen
    for entity in loading_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    
    // Load the starting level
    // In a real implementation, this would load from the generated world DAG
    let level_handle = asset_server.load("levels/starting_area.yol");
    commands.spawn(YoleckLoadLevel(level_handle));
    
    // Spawn player
    spawn_player(&mut commands, &asset_server);
    
    // Show world name
    commands.spawn((
        TextBundle::from_section(
            format!("Welcome to {}", world_seed.world_name),
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 30.0,
                color: Color::WHITE,
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(20.0),
            left: Val::Px(20.0),
            ..default()
        }),
        WorldNameText,
    ));
    
    next_state.set(GameState::Playing);
}

#[derive(Component)]
struct WorldNameText;

fn spawn_player(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/hero.png"),
            transform: Transform::from_xyz(0.0, 0.0, 10.0),
            ..default()
        },
        Player {
            name: "Hero".to_string(),
            level: 1,
            experience: 0,
        },
        Health {
            current: 100,
            maximum: 100,
        },
        CombatStats {
            attack: 10,
            defense: 5,
            speed: 7,
        },
        Inventory::default(),
        InputManagerBundle::<PlayerAction> {
            action_state: ActionState::default(),
            input_map: InputMap::default()
                .insert(PlayerAction::MoveUp, KeyCode::KeyW)
                .insert(PlayerAction::MoveDown, KeyCode::KeyS)
                .insert(PlayerAction::MoveLeft, KeyCode::KeyA)
                .insert(PlayerAction::MoveRight, KeyCode::KeyD)
                .insert(PlayerAction::Attack, KeyCode::Space)
                .insert(PlayerAction::Interact, KeyCode::KeyE)
                .insert(PlayerAction::OpenInventory, KeyCode::KeyI)
                .insert(PlayerAction::Save, KeyCode::F5)
                .insert(PlayerAction::Load, KeyCode::F9)
                .build(),
        },
        RigidBody::Dynamic,
        Collider::ball(16.0),
        Velocity::zero(),
        LockedAxes::ROTATION_LOCKED,
    ));
}