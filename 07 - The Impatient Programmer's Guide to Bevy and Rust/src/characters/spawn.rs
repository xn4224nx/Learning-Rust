/*
 * Spawning Characters
 * ===================
 */

use crate::characters::animations::*;
use crate::characters::config::{CharacterEntry, CharactersList};
use crate::characters::movement::Player;
use bevy::prelude::*;

const PLAYER_SCALE: f32 = 0.8;
const PLAYER_Z_POSITION: f32 = 20.0;

#[derive(Resource, Default)]
pub struct CurrentCharacterIndex {
    pub index: usize,
}

#[derive(Resource)]
pub struct CharactersListResource {
    pub handle: Handle<CharactersList>,
}

/// Create a texture atlas layout for a character.
fn create_character_atlas_layout(
    atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    character_entry: &CharacterEntry,
) -> Handle<TextureAtlasLayout> {
    let max_row = character_entry.calculate_max_animation();

    return atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::splat(character_entry.tile_size),
        character_entry.atlas_columns as u32,
        (max_row + 1) as u32,
        None,
        None,
    ));
}

pub fn spawn_player(
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
    mut character_idx: ResMut<CurrentCharacterIndex>,
) {
    let characters_list_handle: Handle<CharactersList> =
        asset_server.load("characters/characters.ron");

    cmds.insert_resource(CharactersListResource {
        handle: characters_list_handle,
    });

    /* Start with the same character, the first one. */
    character_idx.index = 0;

    /* Spawn the player asset. */
    cmds.spawn((
        Player,
        Transform::from_translation(Vec3::new(0.0, 0.0, PLAYER_Z_POSITION))
            .with_scale(Vec3::splat(PLAYER_SCALE)),
        Sprite::default(),
    ));
}

pub fn initialise_player_character(
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    characters_lists: Res<Assets<CharactersList>>,
    character_idx: Res<CurrentCharacterIndex>,
    characters_list_res: Option<Res<CharactersListResource>>,
    mut query: Query<Entity, (With<Player>, Without<AnimationController>)>,
) {
    let Some(characters_list_res) = characters_list_res else {
        return;
    };

    for entity in query.iter_mut() {
        let Some(characters_list) = characters_lists.get(&characters_list_res.handle) else {
            continue;
        };

        if character_idx.index >= characters_list.characters.len() {
            continue;
        }

        let character_entry = &characters_list.characters[character_idx.index];
        let texture = asset_server.load(&character_entry.texture_path);
        let layout = create_character_atlas_layout(&mut atlas_layouts, character_entry);

        let sprite = Sprite::from_atlas_image(texture, TextureAtlas { layout, index: 0 });

        cmds.entity(entity).insert((
            AnimationController::default(),
            AnimationState::default(),
            AnimationTimer(Timer::from_seconds(
                DEFAULT_ANIMATION_FRAME_TIME,
                TimerMode::Repeating,
            )),
            character_entry.clone(),
            sprite,
        ));
    }
}

pub fn switch_character(
    input: Res<ButtonInput<KeyCode>>,
    mut character_index: ResMut<CurrentCharacterIndex>,
    character_lists: Res<Assets<CharactersList>>,
    character_list_res: Option<Res<CharactersListResource>>,
    mut query: Query<(&mut CharacterEntry, &mut Sprite), With<Player>>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
) {
    /* Map the key to an index. */
    const DIGIT_KEYS: [KeyCode; 9] = [
        KeyCode::Digit1,
        KeyCode::Digit2,
        KeyCode::Digit3,
        KeyCode::Digit4,
        KeyCode::Digit5,
        KeyCode::Digit6,
        KeyCode::Digit7,
        KeyCode::Digit8,
        KeyCode::Digit9,
    ];

    /* Determine the digit key was pressed. */
    let Some(new_idx) = DIGIT_KEYS.iter().position(|&x| input.just_pressed(x)) else {
        return;
    };

    let Some(character_list_res) = character_list_res else {
        return;
    };

    let Some(character_list) = character_lists.get(&character_list_res.handle) else {
        return;
    };

    if new_idx >= character_list.characters.len() {
        return;
    };

    /* Update character index and player entity. */
    character_index.index = new_idx;
    let Ok((mut current_entry, mut sprite)) = query.single_mut() else {
        return;
    };

    /* Update the character entry. */
    let character_entry = &character_list.characters[new_idx];

    *current_entry = character_entry.clone();

    /* Update sprite with the new texture. */
    let texture = asset_server.load(&character_entry.texture_path);
    let layout = create_character_atlas_layout(&mut atlas_layouts, character_entry);

    *sprite = Sprite::from_atlas_image(texture, TextureAtlas { layout, index: 0 });
}
