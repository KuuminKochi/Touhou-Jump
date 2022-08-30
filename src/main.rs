use bevy::{prelude::*, sprite::collide_aabb::collide};

const BACKGROUND_COLOR: Color = Color::rgb(1., 0.5, 0.5);
const GROUND_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
const PLAYER_SPRITE: &str = "satorineutral.png";
const PLAYER_SIZE_X: f32 = 0.5;
const PLAYER_SIZE_Y: f32 = 0.5;
const GROUND_SIZE_X: f32 = 1500.;
const GROUND_SIZE_Y: f32 = 100.;
const GRAVITY: f32 = 5.;

#[derive(Component)]
struct Player;
#[derive(Component)]
struct Ground;

fn main() {
    App::new()
        .add_startup_system(setup)
        .add_startup_system(player_sprite)
        .add_startup_system(ground)
        .add_system(player_controller)
        .add_system(gravity)
        .add_system(collision_detection)
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(GRAVITY)
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
                translation: Vec3::new(0.0, 0.0, 0.0),
                scale: Vec3::new(PLAYER_SIZE_X, PLAYER_SIZE_Y, 0.0),
                ..default()
            },
            ..default()
        })
        .insert(Player);
}

fn ground(mut commands: Commands) {
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: GROUND_COLOR,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0., -400., 0.),
            scale: Vec3::new(GROUND_SIZE_X, GROUND_SIZE_Y, 0.),
            ..default()
        },
        ..default()
    })
    .insert(Ground);
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

fn gravity(mut player_positions: Query<&mut Transform, With<Player>>) {
    for mut transform in player_positions.iter_mut() {
        transform.translation.y -= GRAVITY;
    }
}

fn collision_detection(
    ground: Query<&Transform, With<Ground>>,
    mut player: Query<&mut Transform, With<Player>>
) {
    let player_size = Vec2::new(PLAYER_SIZE_X, PLAYER_SIZE_Y);
    let ground_size = Vec2::new(GROUND_SIZE_X, GROUND_SIZE_Y);

    for ground in ground.iter() {
        for mut player in player.iter_mut() {
            if collide(player.translation, player_size, ground.translation, ground_size).is_some() {
                player.translation.y += GRAVITY;
            }
        }
    }

}
