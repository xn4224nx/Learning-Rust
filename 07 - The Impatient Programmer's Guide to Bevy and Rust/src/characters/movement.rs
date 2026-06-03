/*
 * Character Movement System
 * =========================
 */

use crate::characters::animations::*;
use crate::characters::config::{AnimationType, CharacterEntry};
use bevy::prelude::*;

/// Convert key presses to a direction vector
fn read_movement_input(key_presses: &ButtonInput<KeyCode>) -> Vec2 {
    const MOVEMENT_KEYS: [(KeyCode, Vec2); 4] = [
        (KeyCode::KeyA, Vec2::NEG_X),
        (KeyCode::KeyD, Vec2::X),
        (KeyCode::KeyW, Vec2::Y),
        (KeyCode::KeyS, Vec2::NEG_Y),
    ];

    return MOVEMENT_KEYS
        .iter()
        .filter(|(key, _)| key_presses.pressed(*key))
        .map(|(_, direct)| *direct)
        .sum();
}

/// Determine the character's movement speed.
fn calculate_movement_speed(character: &CharacterEntry, is_running: bool) -> f32 {
    return if is_running {
        character.base_move_speed * character.run_speed_multiplier
    } else {
        character.base_move_speed
    };
}

#[derive(Component)]
pub struct Player;

pub fn move_player(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<
        (
            &mut Transform,
            &mut AnimationController,
            &mut AnimationState,
            &CharacterEntry,
        ),
        With<Player>,
    >,
) {
    let Ok((mut transform, mut animated, mut state, character)) = query.single_mut() else {
        return;
    };

    let direct = read_movement_input(&input);

    /* Are they jumping. */
    if input.just_pressed(KeyCode::Space) {
        state.is_jumping = true;
        animated.current_animation = AnimationType::Jump;
    }

    /* Are they running. */
    let is_running = input.pressed(KeyCode::ShiftLeft) || input.pressed(KeyCode::ShiftRight);

    /* Are the moving in any direction? */
    if direct != Vec2::ZERO {
        let move_speed = calculate_movement_speed(character, is_running);
        let delta = direct.normalize() * move_speed * time.delta_secs();
        transform.translation += delta.extend(0.0);

        animated.facing = Facing::from_direction(direct);

        /* Only update the animation if not jumping. */
        if !state.is_jumping {
            state.is_moving = true;
            animated.current_animation = if is_running {
                AnimationType::Run
            } else {
                AnimationType::Walk
            };
        }
    } else if !state.is_jumping {
        state.is_moving = false;
        animated.current_animation = AnimationType::Walk;
    }
}

/// Monitor the jump animation and reset the state when it completes.
pub fn update_jump_state(
    mut query: Query<
        (
            &mut AnimationController,
            &mut AnimationState,
            &AnimationTimer,
            &Sprite,
            &CharacterEntry,
        ),
        With<Player>,
    >,
) {
    for (mut animated, mut state, timer, sprite, config) in query.iter_mut() {
        if !state.is_jumping {
            continue;
        }

        let Some(atlas) = sprite.texture_atlas.as_ref() else {
            continue;
        };

        let Some(clip) = animated.get_clip(config) else {
            continue;
        };

        /* Has the jump animation completed? */
        if clip.is_complete(atlas.index, timer.just_finished()) {
            state.is_jumping = false;
            animated.current_animation = AnimationType::Walk;
        }
    }
}
