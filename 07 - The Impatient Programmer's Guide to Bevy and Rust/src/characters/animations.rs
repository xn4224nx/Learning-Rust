/*
 * Animation Engine
 * ================
 */

use crate::characters::config::{AnimationType, CharacterEntry};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/* Seconds per frame or 1/FPS */
pub const DEFAULT_ANIMATION_FRAME_TIME: f32 = 0.1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Facing {
    Up,
    Left,
    Down,
    Right,
}

impl Facing {
    /// Convert a velocity vector into a discrete direction.
    pub fn from_direction(direct: Vec2) -> Self {
        return if direct.x.abs() > direct.y.abs() {
            if direct.x > 0.0 {
                Facing::Right
            } else {
                Facing::Left
            }
        } else {
            if direct.y > 0.0 {
                Facing::Up
            } else {
                Facing::Down
            }
        };
    }

    /// Map a direction to the row offset.
    fn direction_idx(self) -> usize {
        match self {
            Facing::Up => 0,
            Facing::Left => 1,
            Facing::Down => 2,
            Facing::Right => 3,
        }
    }
}

#[derive(Component)]
pub struct AnimationController {
    pub current_animation: AnimationType,
    pub facing: Facing,
}

impl Default for AnimationController {
    fn default() -> Self {
        return Self {
            current_animation: AnimationType::Walk,
            facing: Facing::Down,
        };
    }
}

#[derive(Component, Default)]
pub struct AnimationState {
    pub is_moving: bool,
    pub was_moving: bool,
    pub is_jumping: bool,
    pub was_jumping: bool,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Clone, Copy)]
pub struct AnimationClip {
    first: usize,
    last: usize,
}

impl AnimationClip {
    pub fn new(row: usize, frame_count: usize, atlas_columns: usize) -> Self {
        let first = row * atlas_columns;

        return Self {
            first,
            last: first + frame_count - 1,
        };
    }

    pub fn start(self) -> usize {
        return self.first;
    }

    /// Check if a frame index belongs to this clip.
    pub fn contains(self, idx: usize) -> bool {
        return idx >= self.first && idx <= self.last;
    }

    /// Determine the next frame index in the animation
    pub fn next(self, idx: usize) -> usize {
        if idx >= self.last {
            self.first
        } else {
            idx + 1
        }
    }

    /// Check if the animation has completed.
    pub fn is_complete(self, curr_idx: usize, timer_finished: bool) -> bool {
        return curr_idx >= self.last && timer_finished;
    }
}

impl AnimationController {
    pub fn get_clip(&self, config: &CharacterEntry) -> Option<AnimationClip> {
        let def = config.animations.get(&self.current_animation)?;

        /* Based on the direction determine the row of animation. */
        let row = if def.directional {
            def.start_row + self.facing.direction_idx()
        } else {
            def.start_row
        };

        /* Create the clip. */
        return Some(AnimationClip::new(
            row,
            def.frame_count,
            config.atlas_columns,
        ));
    }
}

/// Generic Animation Function
pub fn animate_characters(
    time: Res<Time>,
    mut query: Query<(
        &AnimationController,
        &AnimationState,
        &mut AnimationTimer,
        &mut Sprite,
        &CharacterEntry,
    )>,
) {
    for (animated, state, mut timer, mut sprite, config) in query.iter_mut() {
        let Some(atlas) = sprite.texture_atlas.as_mut() else {
            continue;
        };

        /* Extract the correct clip for the current direction. */
        let Some(clip) = animated.get_clip(config) else {
            continue;
        };

        /* Extract the timing info. */
        let Some(anim_def) = config.animations.get(&animated.current_animation) else {
            continue;
        };

        /* Ensure that we are in a frame in the clip. */
        if !clip.contains(atlas.index) {
            atlas.index = clip.start();
            timer.0.reset();
        }

        /* Has the animation changed at all? If so reset it. */
        if (state.is_moving && !state.was_moving)
            || (!state.is_moving && state.was_moving)
            || (state.is_jumping && !state.was_jumping)
            || (!state.is_jumping && state.was_jumping)
        {
            atlas.index = clip.start();
            timer
                .0
                .set_duration(std::time::Duration::from_secs_f32(anim_def.frame_time));
            timer.0.reset();

        /* Should this be animated? Then advance the animation.*/
        } else if state.is_jumping || state.is_moving {
            timer.tick(time.delta());

            if timer.just_finished() {
                atlas.index = clip.next(atlas.index);
            }

        /* An idle state. */
        } else {
            if atlas.index != clip.start() {
                atlas.index = clip.start();
            }
        }
    }
}

/// Update the `was_moving` flags at the end of the frame.
pub fn update_animation_flags(mut query: Query<&mut AnimationState>) {
    for mut state in query.iter_mut() {
        state.was_moving = state.is_moving;
        state.was_jumping = state.is_jumping;
    }
}
