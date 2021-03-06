//! Systems spawning entities of the game.

use bevy::prelude::*;
use rand::random;

use crate::{
    collisions::{BoxCollider, Movement, Position, TriggerArea},
    constants::{WINDOW_HEIGHT, WINDOW_WIDTH},
};

use super::{
    happiness::Happiness,
    items::{AskedItem, AskingItem, Item, ItemProducer},
    materials::GameplayMaterials,
    Baobei, Didi,
};

/// Plugin that spawns main entities of the game.
pub struct SpawnEntitiesPlugin;

impl Plugin for SpawnEntitiesPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup_camera.system())
            .add_startup_system(spawn_background.system())
            .add_startup_system(spawn_furniture.system())
            .add_startup_system(spawn_didi_and_baobei.system())
            .add_startup_system(spawn_item_producers.system())
            .add_startup_system(spawn_boarders.system());
    }
}

/// Stores entities in the gameplay phase
pub struct GameData {
    /// Entity of didi
    pub didi_entity: Entity,
    /// Entity of baobei
    pub baobei_entity: Entity,
}

/// Spawn the camera.
fn setup_camera(mut commands: Commands) {
    let mut camera_2d = OrthographicCameraBundle::new_2d();
    camera_2d.transform.translation += Vec3::new(WINDOW_WIDTH / 2.0, WINDOW_HEIGHT / 2.0, 0.0);

    commands.spawn().insert_bundle(camera_2d);
}

/// Spawn the background of the screen.
fn spawn_background(mut commands: Commands, materials: Res<GameplayMaterials>) {
    commands.spawn().insert_bundle(SpriteBundle {
        material: materials.background_sprite.clone(),
        transform: Transform::from_xyz(WINDOW_WIDTH / 2.0, WINDOW_HEIGHT / 2.0, 0.0),
        ..SpriteBundle::default()
    });
}

/// Spawn the entity for Didi, the player and Baobei.
fn spawn_didi_and_baobei(mut commands: Commands, materials: Res<GameplayMaterials>) {
    let transform = Transform::from_scale(Vec3::new(0.3, 0.3, 0.0));

    let didi_entity = commands
        .spawn()
        .insert(Didi)
        .insert(Position(Vec3::new(640.0, 260.0, 0.0)))
        .insert(BoxCollider {
            size: Vec2::new(75.0, 50.0),
            offset: Vec3::new(0.0, -10.0, 0.0),
        })
        .insert(Movement::default())
        .insert_bundle(SpriteBundle {
            material: materials.didi_sprite.clone(),
            transform,
            ..SpriteBundle::default()
        })
        .id();

    let asked_item = random::<Item>();

    let baobei_entity = commands
        .spawn()
        .insert(Baobei)
        .insert(Position(Vec3::new(1050.0, 150.0, 85.0)))
        .insert(TriggerArea::new(150.0, 150.0))
        .insert(AskingItem(asked_item))
        .insert(Happiness::happy())
        .insert_bundle(SpriteBundle {
            material: materials.baobei_sprite.clone(),
            transform,
            ..SpriteBundle::default()
        })
        .with_children(|parent| {
            parent
                .spawn()
                .insert(AskedItem)
                .insert_bundle(SpriteBundle {
                    material: materials.item_sprite_for(asked_item),
                    transform: Transform {
                        translation: Vec3::new(0.0, 475.0, 0.0),
                        scale: Vec3::new(1.5, 1.5, 0.0),
                        ..Transform::default()
                    },
                    ..SpriteBundle::default()
                });
        })
        .id();

    commands.insert_resource(GameData {
        didi_entity,
        baobei_entity,
    });
}

/// Spawn furniture in the.
fn spawn_furniture(mut commands: Commands, materials: Res<GameplayMaterials>) {
    // Sink
    commands
        .spawn()
        .insert(Position(Vec3::new(1050.0, 500.0, 0.0)))
        .insert(BoxCollider {
            size: Vec2::new(220.0, 40.0),
            offset: Vec3::new(0.0, 10.0, 0.0),
        })
        .insert_bundle(SpriteBundle {
            material: materials.sink_sprite.clone(),
            transform: Transform::from_scale(Vec3::new(0.3, 0.3, 0.0)),
            ..SpriteBundle::default()
        });
    // Kitchen
    commands
        .spawn()
        .insert(Position(Vec3::new(300.0, 540.0, 0.0)))
        .insert(BoxCollider::new(400.0, 100.0))
        .insert_bundle(SpriteBundle {
            material: materials.kitchen_sprite.clone(),
            transform: Transform::from_scale(Vec3::new(0.5, 0.5, 0.0)),
            ..SpriteBundle::default()
        });
    // Fridge
    commands
        .spawn()
        .insert(Position(Vec3::new(720.0, 540.0, 0.0)))
        .insert(BoxCollider::new(100.0, 100.0))
        .insert_bundle(SpriteBundle {
            material: materials.fridge_sprite.clone(),
            transform: Transform::from_scale(Vec3::new(0.35, 0.35, 0.0)),
            ..SpriteBundle::default()
        });
    // Couch
    commands
        .spawn()
        .insert(Position(Vec3::new(1000.0, 150.0, 0.0)))
        .insert(BoxCollider {
            size: Vec2::new(300.0, 40.0),
            offset: Vec3::new(10.0, 15.0, 0.0),
        })
        .insert_bundle(SpriteBundle {
            material: materials.couch_sprite.clone(),
            transform: Transform::from_scale(Vec3::new(0.4, 0.4, 0.0)),
            ..SpriteBundle::default()
        });
    // Table
    commands
        .spawn()
        .insert(Position(Vec3::new(300.0, 200.0, 0.0)))
        .insert(BoxCollider {
            size: Vec2::new(300.0, 40.0),
            offset: Vec3::new(0.0, 25.0, 0.0),
        })
        .insert_bundle(SpriteBundle {
            material: materials.table_sprite.clone(),
            transform: Transform::from_scale(Vec3::new(0.4, 0.4, 0.0)),
            ..SpriteBundle::default()
        });
}

/// Spawn item producers.
fn spawn_item_producers(mut commands: Commands) {
    commands
        .spawn()
        .insert(ItemProducer(Item::WaterGlass))
        .insert(Position(Vec3::new(1050.0, 500.0, 0.0)))
        .insert(TriggerArea::new(230.0, 50.0));
    commands
        .spawn()
        .insert(ItemProducer(Item::Chips))
        .insert(Position(Vec3::new(210.0, 480.0, 0.0)))
        .insert(TriggerArea::new(75.0, 75.0));
    commands
        .spawn()
        .insert(ItemProducer(Item::IceCream))
        .insert(Position(Vec3::new(720.0, 540.0, 0.0)))
        .insert(TriggerArea::new(175.0, 175.0));
}

/// Spawn boarders of the room, avoiding the user to go out of the screen.
fn spawn_boarders(mut commands: Commands) {
    /// Gap between the screen limit and the available space.
    const GAP: f32 = 50.0;

    // Top
    commands
        .spawn()
        .insert(Position(Vec3::new(WINDOW_WIDTH / 2.0, 510.0 + GAP, 0.0)))
        .insert(BoxCollider::new(WINDOW_WIDTH, GAP));
    // Bottom
    commands
        .spawn()
        .insert(Position(Vec3::new(WINDOW_WIDTH / 2.0, GAP / 2.0, 0.0)))
        .insert(BoxCollider::new(WINDOW_WIDTH, GAP));
    // Left
    commands
        .spawn()
        .insert(Position(Vec3::new(GAP / 2.0, WINDOW_HEIGHT / 2.0, 0.0)))
        .insert(BoxCollider::new(GAP, WINDOW_HEIGHT));
    // Right
    commands
        .spawn()
        .insert(Position(Vec3::new(
            WINDOW_WIDTH - GAP / 2.0,
            WINDOW_HEIGHT / 2.0,
            0.0,
        )))
        .insert(BoxCollider::new(GAP, WINDOW_HEIGHT));
}
