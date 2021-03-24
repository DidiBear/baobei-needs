//! Systems and components managing the happiness of Baobei.
use bevy::prelude::*;

use crate::{
    collisions::Position,
    constants::{GameState, HAPPINESS_DECREASE, STAGE},
    drawing::UIObject,
};

use super::materials::GameplayMaterials;

/// Plugin managing the happiness value.
pub struct HappinessPlugin;

impl Plugin for HappinessPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(HappinessTimer::default())
            .add_startup_system(spawn_happiness_smiley.system())
            .add_startup_system(spawn_debug_text.system())
            .on_state_update(STAGE, GameState::InGame, decrease_happiness_system.system())
            .on_state_update(STAGE, GameState::InGame, text_update_system.system())
            .on_state_update(
                STAGE,
                GameState::InGame,
                update_happiness_sprite_system.system(),
            );
    }
}

/// Component representing the  for the happiness of the entity (Baobei).
/// Between 0 and 1.
pub struct Happiness(f32);

impl Happiness {
    /// Returns a happiness of 100%.
    pub const fn happy() -> Self {
        Self(1.0)
    }

    /// Adds the given value and clamps the result between 0 and 1
    pub fn add(&mut self, value: f32) {
        self.0 += value;

        // TODO: Use `clamp` when stable
        if self.0 > 1.0 {
            self.0 = 1.0;
        }
        if self.0 < 0.0 {
            self.0 = 0.0;
        }
    }

    /// Subtracts the given value and clamps the result between 0 and 1
    fn sub(&mut self, value: f32) {
        self.add(-value)
    }
}

/// Spawn boarders of the room, avoiding the user to go out of the screen.
fn spawn_happiness_smiley(commands: &mut Commands, materials: Res<GameplayMaterials>) {
    commands
        .spawn((UIObject, Position(Vec3::new(1125.0, 300.0, 0.0))))
        .with_bundle(SpriteSheetBundle {
            texture_atlas: materials.emotion_atlas.clone(),
            transform: Transform::from_scale(Vec3::splat(0.3)),
            sprite: TextureAtlasSprite {
                index: 4,
                ..TextureAtlasSprite::default()
            },
            ..SpriteSheetBundle::default()
        });
}

/// Timer of the decrease of the happiness over time.
struct HappinessTimer(Timer);

impl Default for HappinessTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(1.0, true))
    }
}

/// Update the Happiness smiley image depending on the new happiness value.
fn update_happiness_sprite_system(
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut sprites: Query<(&mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
    happiness_values: Query<&Happiness, Changed<Happiness>>,
) {
    for happiness_value in happiness_values.iter() {
        for (mut sprite, texture_atlas_handle) in sprites.iter_mut() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            let nb_sprites = texture_atlas.textures.len() as f32;

            // Happiness is between 0 and 1 and the result index is a small number
            #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
            let sprite_index = (happiness_value.0 * nb_sprites) as u32;

            sprite.index = sprite_index;
        }
    }
}

/// Update the value of the happiness text.
fn decrease_happiness_system(
    time: Res<Time>,
    mut timer: ResMut<HappinessTimer>,
    mut happiness_values: Query<&mut Happiness>,
) {
    if !timer.0.tick(time.delta_seconds()).just_finished() {
        return;
    }
    for mut happiness in happiness_values.iter_mut() {
        happiness.sub(HAPPINESS_DECREASE);
    }
}

/// Tag the text displaying the happiness of Baobei.
struct HappinessText;

/// Spawn debug text showing the happiness value.
pub fn spawn_debug_text(commands: &mut Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..Style::default()
            },
            text: Text {
                value: "Happiness:".to_string(),
                font: asset_server.load("FiraSans-Bold.ttf"),
                style: TextStyle {
                    font_size: 30.0,
                    color: Color::WHITE,
                    ..TextStyle::default()
                },
            },
            ..TextBundle::default()
        })
        .with(HappinessText);
}

/// Update the value of the happiness text.
fn text_update_system(
    mut happiness_text: Query<&mut Text, With<HappinessText>>,
    happiness_value: Query<&Happiness, Changed<Happiness>>,
) {
    for mut text in happiness_text.iter_mut() {
        if let Some(value) = happiness_value.iter().next() {
            text.value = format!("Happiness: {:.2}", value.0);
        }
    }
}