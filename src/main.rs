use bevy::{prelude::*, input::keyboard::KeyboardInput};

const BACKGROUND_COLOR: Color = Color::rgb(1., 0.5, 0.5);
const PLAYER_SPRITE: &str = "satorineutral.png";

#[derive(Component)]
struct Player;

fn main() {
    App::new()
        .add_startup_system(setup)
        .add_startup_system(player_sprite)
        .add_system(player_controller)
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn player_sprite(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load(PLAYER_SPRITE),
            transform: Transform {
                translation: Vec3::new(0.0, 2.0, 0.0),
                scale: Vec3::new(1.0, 1.0, 0.0),
                ..default()
            },
            ..default()
        })
        .insert(Player);
}

fn player_controller(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_positions: Query<&mut Transform, With<Player>>,
) {
    for mut transform in player_positions.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            transform.translation.x -= 5.;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            transform.translation.x += 5.;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            transform.translation.y -= 5.;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            transform.translation.y += 5.;
        }
    } 
}