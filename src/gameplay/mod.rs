//! Systems of the game phase

use bevy::prelude::*;

use crate::{collisions::CollisionSystems, constants::GameState, controllers::ControllerSystems};

use self::{
    entities::SpawnEntitiesPlugin, happiness::HappinessPlugin, items::ItemsPlugin,
    materials::GameplayMaterials, movement::movement_system,
};

mod entities;
mod happiness;
mod items;
mod materials;
mod movement;

/// Plugin the gameplay of the game
pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<GameplayMaterials>()
            .register_type::<Didi>()
            .register_type::<Furniture>()
            .register_type::<Baobei>()
            .add_plugin(SpawnEntitiesPlugin)
            .add_system_set(
                SystemSet::on_update(GameState::InGame)
                    .with_system(back_to_menu_system.system())
                    .with_system(
                        movement_system
                            .system()
                            .after(ControllerSystems)
                            .before(CollisionSystems),
                    ),
            )
            .add_plugin(ItemsPlugin)
            .add_plugin(HappinessPlugin);
    }
}

/// Goes back to the menu state when the player press `Escape`.
fn back_to_menu_system(keyboard_input: Res<Input<KeyCode>>, mut state: ResMut<State<GameState>>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        state.set(GameState::Menu).unwrap();
    }
}

/// The player
#[derive(Reflect, Default)]
#[reflect(Component)]
pub struct Didi;
/// The baobei to take care of
#[derive(Reflect, Default)]
#[reflect(Component)]
pub struct Baobei;
/// Furniture containing items
#[derive(Reflect, Default)]
#[reflect(Component)]
pub struct Furniture;
