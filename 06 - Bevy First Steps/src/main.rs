/*
 * Bevy Tutorial
 * https://blog.jetbrains.com/rust/2025/02/04/first-steps-in-game-development-with-rust-and-bevy/
 */

use bevy::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::prelude::{EntropyPlugin, GlobalRng};
use rand_core::Rng;

const GAME_SPEED: f32 = 400.0;
const GRAVITY: f32 = -800.0;

const GROUND_LEVEL: f32 = -50.0;
const GROUND_SIZE: Vec2 = Vec2::new(900.0, 10.0);
const GROUND_EDGE: f32 = GROUND_SIZE.x / 2.0;

const PLAYER_X: f32 = -300.0;
const PLAYER_SIZE: Vec2 = Vec2::new(30.0, 50.0);
const JUMP_FORCE: f32 = 600.0;

const SPAWN_INTERVAL: f32 = 1.0;
const OBSTACLE_SIZE: Vec2 = Vec2::new(30.0, 30.0);
const OBSTACLE_COLOR: Color = Color::srgb(1.0, 0.0, 0.0);

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Velocity(Vec3);

#[derive(Component)]
struct Obstacle;

#[derive(Resource)]
struct ObstacleSpawningTimer(Timer);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EntropyPlugin::<WyRand>::default())
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                jump,
                apply_gravity_to_player,
                player_movement,
                spawn_obstacles,
                move_obstacles,
            ),
        )
        .insert_resource(ObstacleSpawningTimer(Timer::from_seconds(
            SPAWN_INTERVAL,
            TimerMode::Repeating,
        )))
        .run();
}

fn setup(mut cmds: Commands) {
    cmds.spawn(Camera2d::default());

    /* Spawn the player. */
    cmds.spawn((
        Player,
        Sprite {
            color: Color::srgb(0.5, 1.0, 1.0),
            custom_size: Some(PLAYER_SIZE),
            ..default()
        },
        Transform::from_xyz(PLAYER_X, GROUND_LEVEL, 0.0),
        Velocity(Vec3::ZERO),
    ));

    /* Spawn the ground. */
    cmds.spawn((
        Sprite {
            color: Color::srgb(0.5, 0.5, 0.5),
            custom_size: Some(GROUND_SIZE),
            ..default()
        },
        Transform::from_xyz(0.0, GROUND_LEVEL - 3.0 * GROUND_SIZE.y, 0.0),
    ));
}

/// Detect the jump button being pressed and apply the resultant up velocity.
fn jump(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &Transform), With<Player>>,
) {
    /* Apply the force from jumping if the right button is pressed. */
    if let Ok((mut velocity, transform)) = query.single_mut() {
        if keys.pressed(KeyCode::Space) && transform.translation.y <= GROUND_LEVEL {
            velocity.0.y = JUMP_FORCE;
        }
    }
}

/// Dictate how the player moves.
fn player_movement(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Velocity), With<Player>>,
) {
    for (mut transform, mut velocity) in query.iter_mut() {
        transform.translation.y += velocity.0.y * time.delta_secs();

        /* Ensure the player never falls through the ground and stops there. */
        if transform.translation.y <= GROUND_LEVEL {
            transform.translation.y = GROUND_LEVEL;
            velocity.0.y = 0.0;
        }
    }
}

/// Pull the player down all the time
fn apply_gravity_to_player(time: Res<Time>, mut query: Query<&mut Velocity, With<Player>>) {
    for mut vel in query.iter_mut() {
        vel.0.y += GRAVITY * time.delta_secs();
    }
}

/// Randomly place obstacles for the player to avoid
fn spawn_obstacles(
    mut cmds: Commands,
    time: Res<Time>,
    mut spawn_timer: ResMut<ObstacleSpawningTimer>,
    mut rng: Single<&mut WyRand, With<GlobalRng>>,
) {
    /* Advance the timer. */
    spawn_timer.0.tick(time.delta());

    if spawn_timer.0.is_finished() {
        let obst_x = GROUND_EDGE;
        let obst_y = GROUND_LEVEL + (rng.next_u32() % 100) as f32;

        cmds.spawn((
            Obstacle,
            Sprite {
                color: OBSTACLE_COLOR,
                custom_size: Some(OBSTACLE_SIZE),
                ..default()
            },
            Transform::from_xyz(obst_x, obst_y, 0.0),
        ));
    }
}

/// Move towards the player and remove them after they are off-screen.
fn move_obstacles(
    time: Res<Time>,
    mut cmds: Commands,
    mut query: Query<(Entity, &mut Transform), With<Obstacle>>,
) {
    for (ent_obj, mut transform) in query.iter_mut() {
        transform.translation.x -= GAME_SPEED * time.delta_secs();

        /* Remove obstacles that have disapeared off-screen. */
        if transform.translation.x < -GROUND_EDGE {
            cmds.entity(ent_obj).despawn();
        }
    }
}
