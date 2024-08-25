use bevy::{math::vec2, prelude::*};
use runner::{Object, ObjectProperties, StaticMap};
use crate::player::Player;

pub fn setup_level_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    // Player.
    let size = vec2(26.0, 48.0);
    let texture = asset_server.load("white knight.png");
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(size),
                ..default()
            },
            texture,
            transform: Transform {
                translation: vec2(64.0, 148.0).extend(0.0),
                ..default()
            },
            ..default()
        },
        Player,
        Object::basic(),
        ObjectProperties::new(size),
        Name::new("Da Player")
    ));


    // Spawn camera
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scale: 2.0,
            near: -1000.0,
            ..default()
        },
        ..default()
    });

    // Spawn level
    commands.spawn(StaticMap::_debug_test());
}