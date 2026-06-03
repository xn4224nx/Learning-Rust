/*
 * Character Module
 * ================
 */

pub mod animations;
pub mod config;
pub mod movement;
pub mod spawn;

use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use config::CharactersList;

pub struct CharactersPlugin;

impl Plugin for CharactersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::<CharactersList>::new(&["characters.ron"]))
            .init_resource::<spawn::CurrentCharacterIndex>()
            .add_systems(Startup, spawn::spawn_player)
            .add_systems(
                Update,
                (
                    spawn::initialise_player_character,
                    spawn::switch_character,
                    movement::move_player,
                    movement::update_jump_state,
                    animations::animate_characters,
                    animations::update_animation_flags,
                ),
            );
    }
}
