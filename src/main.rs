```rust
use bevy::prelude::*;
use bevy_tilemap::prelude::*;
use yoleck::YoleckPlugin;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Menu,
    Playing,
    Paused,
}

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Echoes of Beastlight".to_string(),
            width: 16.0 * 40.0,
            height: 16.0 * 30.0,
            ..Default::default()
        })
        .add_state(GameState::Menu)
        .add_plugins(DefaultPlugins)
        .add_plugin(TilemapPlugin)
        .add_plugin(YoleckPlugin)
        .add_startup_system(setup.system())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut state: ResMut<State<GameState>>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let map = Tilemap::builder()
        .topology(GridTopology::Square)
        .dimensions(40, 30)
        .chunk_dimensions(16, 16, 1)
        .texture_dimensions(16, 16)
        .finish()
        .unwrap();

    commands.spawn_bundle(TilemapBundle {
        tilemap: map,
        visible: Visible {
            is_visible: true,
            is_transparent: true,
        },
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        global_transform: GlobalTransform::default(),
    });
}
```