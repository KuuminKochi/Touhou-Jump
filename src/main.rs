use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
    time::{FixedTimestep, Stopwatch},
};

// System Constant
const TIME_STEP: f64 = 1. / 60.;
const TIME_SCALE: f32 = 120.;
const GRAVITY: f32 = 10.;

// Entity Constant
const PLAYERSPEED: f32 = 20.;

// Color
const BACKGROUND_COLOR: Color = Color::rgb(1., 0.5, 0.5);
const GROUND_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
const PLATFORM_COLOR: Color = Color::rgb(0., 1., 0.);

// Sprite
const PLAYER_SPRITE: &str = "satorineutral.png";

// Sprite Sizes
const PLAYER_SIZE_X: f32 = 0.5;
const PLAYER_SIZE_Y: f32 = 0.5;
const GROUND_SIZE_X: f32 = 1500.;
const GROUND_SIZE_Y: f32 = 300.;

#[derive(Component)]
struct Collider;
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

#[derive(Component)]
struct Platform;
#[derive(Default)]
struct CollisionEvent;

fn main() {
    App::new()
        .add_startup_system(setup)
        .add_startup_system(player_spawn)
        .add_startup_system(platform_spawn)
        .add_startup_system(ground_spawn)
        .add_startup_system(access_window_system)
        .add_event::<CollisionEvent>()
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(player_controller)
                .with_system(gravity)
                .with_system(collision_detection)
                .with_system(jump),
        )
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
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
        .spawn()
        .insert_bundle(SpriteBundle {
            texture: asset_server.load(PLAYER_SPRITE),
            transform: Transform {
                translation: Vec3::new(0.0, 500.0, 0.0),
                scale: Vec3::new(PLAYER_SIZE_X, PLAYER_SIZE_Y, 0.0),
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
        })
        .insert(Player)
        .insert(Collider);
}

fn ground_spawn(mut commands: Commands) {
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
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
        .insert(Collider);
}

// Move player X translation left and right
fn player_controller(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_positions: Query<&mut Transform, With<Player>>,
) {
    for mut transform in player_positions.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            transform.translation.x -= PLAYERSPEED;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            transform.translation.x += PLAYERSPEED;
        }
    }
}

// Add jumping mechanics to the player
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
            // Check whether or not the player is on the ground first, if not
            // lock Jump. if it is, unlock player jump
            if status.on_ground {
                status.is_jump = true;
                jump.time.reset();
            }
        }

        if status.is_jump {
            // Make sure player jump only lasts for 0.2 seconds before descending
            jump.time.tick(time.delta());
            if jump.time.elapsed_secs() < 0.2 {
                transform.translation.y +=
                    player_velocity * time.delta_seconds_f64() as f32 * TIME_SCALE;
                println!("{}", jump.time.elapsed_secs());
            } else {
                status.is_jump = false;
                // Heavier gravity when falling down
                player_velocity = -10.;
            }
        }
    }
}

fn gravity(time: Res<Time>, mut player_positions: Query<&mut Transform, With<Player>>) {
    for mut transform in player_positions.iter_mut() {
        // Basing the gravity off the frame rate
        transform.translation.y -= GRAVITY * time.delta_seconds_f64() as f32 * TIME_SCALE;
    }
}

fn collision_detection(
    time: Res<Time>,
    collider_query: Query<(Entity, &Transform), (With<Collider>, Without<Player>)>,
    mut player_query: Query<(&mut Transform, &mut PlayerStatus), With<Player>>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    let (mut player_transform, mut status) = player_query.single_mut();
    let player_size = player_transform.scale.truncate();

    for (collider_entity, transform) in collider_query.iter() {
        let collision = collide(
            player_transform.translation,
            player_size,
            transform.translation,
            transform.scale.truncate(),
        );

        if let Some(collision) = collision {
            // Check what type of collisions we're dealing with
            match collision {
                Collision::Left => {
                    player_transform.translation.x -= PLAYERSPEED;
                    println!("Left");
                }
                Collision::Right => {
                    player_transform.translation.x += PLAYERSPEED;
                    println!("Right");
                }
                Collision::Top => {
                    status.on_ground = true;
                    println!("Top");
                }
                Collision::Bottom => println!("Bottom"),
                Collision::Inside => status.on_ground = true,
            }

            // Sends a collision event so that other systems can react to the collision
            collision_events.send_default();
        } else {
            status.on_ground = false;
        }

        // if the player is on the ground, cancel out the gravity by adding up inverted gravity to existing gravity
        if status.on_ground {
            player_transform.translation.y +=
                GRAVITY * time.delta_seconds_f64() as f32 * TIME_SCALE;
            println!("GROUND");
        }
    }
}

fn platform_spawn(mut commands: Commands) {
    // commands
    //     .spawn()
    //     .insert_bundle(SpriteBundle {
    //         sprite: Sprite {
    //             color: PLATFORM_COLOR,
    //             ..default()
    //         },
    //         transform: Transform {
    //             translation: Vec3::new(0., 0., 0.),
    //             scale: Vec3::new(100., 10., 0.),
    //             ..default()
    //         },
    //         ..default()
    //     })
    //     .insert(Ground);
}
