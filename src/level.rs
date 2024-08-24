use bevy::{math::vec2, prelude::*};
use runner::{Object, ObjectProperties};
use crate::player::Player;

pub fn setup_level_system(
    mut commands: Commands
) {
    // Player.
    let size = vec2(26.0, 48.0);
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(size),
                ..default()
            },
            transform: Transform {
                translation: vec2(64.0, 148.0).extend(0.0),
                ..default()
            },
            ..default()
        },
        Player,
        Object::basic(),
        ObjectProperties::new(size, Vec2::NEG_Y * -5.0),
        Name::new("Da Player")
    ));


    // Spawn camera
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scale: 2.0,
            near: 1000.0,
            ..default()
        },
        ..default()
    });
}