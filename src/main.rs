use bevy::{
    input::keyboard::KeyboardInput, prelude::*, sprite::collide_aabb::collide, time::Stopwatch,
};

const TIME_SCALE: f32 = 120.;
const BACKGROUND_COLOR: Color = Color::rgb(1., 0.5, 0.5);
const GROUND_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
const PLAYER_SPRITE: &str = "satorineutral.png";
const PLAYER_SIZE_X: f32 = 1.;
const PLAYER_SIZE_Y: f32 = 1.;
const GROUND_SIZE_X: f32 = 1500.;
const GROUND_SIZE_Y: f32 = 300.;
const GRAVITY: f32 = 10.;

#[derive(Component)]
struct JumpDuration {
    time: Stopwatch,
}
#[derive(Component)]
struct PlayerStatus {
    on_ground: bool,
    is_jump: bool,
}
#[derive(Component)]
struct Player;
#[derive(Component)]
struct Ground;

fn main() {
    App::new()
        .add_startup_system(setup)
        .add_startup_system(player_spawn)
        .add_startup_system(ground)
        .add_startup_system(access_window_system)
        // .add_system(frame_per_seconds_info)
        .add_system(jump)
        .add_system(player_controller)
        .add_system(gravity)
        .add_system(collision_detection)
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(GRAVITY)
        .insert_resource(TIME_SCALE)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn access_window_system(mut windows: ResMut<Windows>) {
    for window in windows.iter_mut() {
        window.set_title(String::from("Touhou Jump"));
        window.set_resolution(1280., 960.);
        window.set_resizable(false);
    }
}

fn player_spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load(PLAYER_SPRITE),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                scale: Vec3::new(0.5, 0.5, 0.0),
                ..default()
            },
            ..default()
        })
        .insert(Player)
        .insert(PlayerStatus {
            on_ground: false,
            is_jump: false,
        })
        .insert(JumpDuration {
            time: Stopwatch::new(),
        });
}
//
fn ground(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: GROUND_COLOR,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0., -250., 0.),
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
            transform.translation.x -= 15.;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            transform.translation.x += 15.;
        }
    }
}

fn jump(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut player: Query<(&mut Transform, &mut PlayerStatus), With<Player>>,
    mut jump: Query<&mut JumpDuration, With<Player>>,
) {
    let mut player_velocity: f32 = 20.;
    let mut jump = jump.single_mut();

    for (mut transform, mut status) in player.iter_mut() { 

        if keyboard_input.just_pressed(KeyCode::Up) {
            if status.on_ground {
            status.is_jump = true;
            jump.time.reset();
            }
        }

            if status.is_jump {
                jump.time.tick(time.delta());
                if jump.time.elapsed_secs() < 0.2 {
                    transform.translation.y += player_velocity * time.delta_seconds_f64() as f32 * TIME_SCALE;
                    println!("{}", jump.time.elapsed_secs());
                } else {
                    status.is_jump = false;
                    player_velocity = -20.;
                }
            }
        }
    }

fn gravity(time: Res<Time>, mut player_positions: Query<&mut Transform, With<Player>>) {
    for mut transform in player_positions.iter_mut() {
        transform.translation.y -= GRAVITY * time.delta_seconds_f64() as f32 * TIME_SCALE;
    }
}

fn collision_detection(
    time: Res<Time>,
    ground: Query<&Transform, (With<Ground>, Without<Player>)>,
    mut player: Query<(&mut Transform, &mut PlayerStatus), With<Player>>,
) {
    let player_size = Vec2::new(PLAYER_SIZE_X, PLAYER_SIZE_Y);
    let ground_size = Vec2::new(GROUND_SIZE_X, GROUND_SIZE_Y);

    for ground in ground.iter() {
        for (mut player, mut status) in player.iter_mut() {
            if collide(
                player.translation,
                player_size,
                ground.translation,
                ground_size,
            ).is_some()
            {
                status.on_ground = true
            } else {
                status.on_ground = false
            }

            if status.on_ground {
                player.translation.y += GRAVITY * time.delta_seconds_f64() as f32 * TIME_SCALE;
            }
        }
    }
}

// fn frame_per_seconds_info(time: Res<Time>) {
//     info!("{}", time.delta_seconds_f64() as f32 * TIME_SCALE);
// }
