/*
 * Bevy Tutorial
 * https://blog.jetbrains.com/rust/2025/02/04/first-steps-in-game-development-with-rust-and-bevy/
 */

use bevy::prelude::*;

const GROUND_LEVEL: f32 = -100.0;
const PLAYER_X: f32 = -300.0;
const JUMP_FORCE: f32 = 600.0;
const GRAVITY: f32 = -800.0;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Velocity(Vec3);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (jump, apply_gravity_to_player, player_movement))
        .run();
}

fn setup(mut cmds: Commands) {
    cmds.spawn(Camera2d::default());

    /* Spawn the player. */
    cmds.spawn((
        Player,
        Sprite {
            color: Color::srgb(0.5, 1.0, 1.0),
            custom_size: Some(Vec2::new(30.0, 50.0)),
            ..default()
        },
        Transform::from_xyz(PLAYER_X, GROUND_LEVEL, 0.0),
        Velocity(Vec3::ZERO),
    ));

    /* Spawn the ground. */
    cmds.spawn((
        Sprite {
            color: Color::srgb(0.5, 0.5, 0.5),
            custom_size: Some(Vec2::new(800.0, 10.0)),
            ..default()
        },
        Transform::from_xyz(0.0, GROUND_LEVEL - 30.0, 0.0),
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
