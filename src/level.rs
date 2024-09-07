use bevy::{math::vec2, prelude::*};
use bevy_ecs_ldtk::prelude::*;
// use bevy_ecs_tilemap::helpers::transform;
use runner::{Object, ObjectProperties, StaticMap, LevelProperties};
use crate::player::{Player, PlayerSpawnPoint};

pub fn setup_level_system(
    mut commands: Commands,
    mut level_properties: ResMut<LevelProperties>,
    asset_server: Res<AssetServer>
) {
    // Spawn camera
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scale: 0.5,
            near: -1000.0,
            ..default()
        },
        ..default()
    });

    // Setup level properties
    level_properties.set_gravity_strength(45.0);
    level_properties.set_tile_size(16.0);

    // Spawn level 0
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("level/level.ldtk"),
        ..default()
    });


    // Player.
    let size = vec2(16.0, 24.0);
    let texture = asset_server.load("player.png");
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(size),
                ..default()
            },
            texture,
            transform: Transform {
                translation: vec2(0.0, 148.0).extend(0.0),
                ..default()
            },
            ..default()
        },
        Player,
        Object::basic(),
        ObjectProperties::new(size),
        Name::new("Da Player")
    ));

    

    commands.spawn(StaticMap::_debug_test());
}


pub struct RegisterLdtkEntites;

impl Plugin for RegisterLdtkEntites {
    fn build(&self, app: &mut App) {
        app
            .register_ldtk_entity::<PlayerSpawnPoint>("PlayerSpawnPoint");
    }
}