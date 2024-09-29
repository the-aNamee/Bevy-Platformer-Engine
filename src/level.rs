use bevy::{math::vec2, prelude::*, render::camera::ScalingMode};
use bevy_ecs_ldtk::prelude::*;
use bevy_ecs_tilemap::{map::TilemapTileSize, tiles::TilePos};
use runner::{LevelProperties, Object, ObjectProperties, PerpWall, PerpWalls, StaticMap};
use crate::player::{Player, PlayerSpawnPoint};

pub fn setup_level_system(
    mut commands: Commands,
    mut level_properties: ResMut<LevelProperties>,
    asset_server: Res<AssetServer>
) {
    // Spawn camera
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            near: -1000.0,
            // scaling_mode: ScalingMode::WindowSize(4.0),
            scaling_mode: ScalingMode::FixedVertical(0xC0 as f32),
            ..default()
        },
        ..default()
    });

    // Setup level properties
    level_properties.set_gravity_strength(100.0);
    level_properties.set_tile_size(16.0);

    // Spawn level 0
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("level/level.ldtk"),
        ..default()
    });


    // Player.
    let size = vec2(8.0, 12.0);
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
        Player::default(),
        Object::basic(),
        ObjectProperties::new(size),
        Name::new("Da Player")
    ));

    
    // Spawn Static Map
    commands.spawn((StaticMap::empty(), MainStaticMap));
}

pub fn setup_collision_map_system(
    tile_query: Query<&TilePos>,
    tilemap_query: Query<&TilemapTileSize, (Without<TilePos>, Without<MainStaticMap>)>,
    mut level_collision: Query<&mut StaticMap, (Without<TilePos>, With<MainStaticMap>)>
) {
    let mut static_map = level_collision.single_mut();
    let tile_amount = tile_query.iter().count();
    if tile_amount <= 0 || static_map.is_setup {
        return;
    }

    let tile_size = tilemap_query.single();
    let tile_size = vec2(tile_size.x, tile_size.y);

    let mut new_right_walls = PerpWalls::empty();
    let mut new_left_walls = PerpWalls::empty();
    let mut new_up_walls = PerpWalls::empty();
    let mut new_down_walls = PerpWalls::empty();
    for tile_pos in tile_query.iter() {
        let tile_pos = vec2(tile_pos.x as f32, tile_pos.y as f32);
        let new_pos = tile_pos * tile_size;

        // Find surrounding tiles
        let mut is_right = false;
        let mut is_left = false;
        let mut is_up = false;
        let mut is_down = false;
        for checking_tile_pos in tile_query.iter() {
            let checking_tile_pos = vec2(checking_tile_pos.x as f32, checking_tile_pos.y as f32);

            if tile_pos + Vec2::X == checking_tile_pos {
                is_right = true;
            }
            if tile_pos + Vec2::NEG_X == checking_tile_pos {
                is_left = true;
            }
            if tile_pos + Vec2::Y == checking_tile_pos {
                is_up = true;
            }
            if tile_pos + Vec2::NEG_Y == checking_tile_pos {
                is_down = true;
            }
        }

        // Right walls
        if !is_right {
            let new_wall = PerpWall::new(new_pos + Vec2::X * tile_size.x, tile_size.x);
            new_right_walls.0.push(new_wall);
        }

        // Left walls
        if !is_left {
            let new_wall = PerpWall::new(new_pos, tile_size.x);
            new_left_walls.0.push(new_wall);
        }

        // Up walls
        if !is_up {
            let new_wall = PerpWall::new(new_pos + Vec2::Y * tile_size.y, tile_size.y);
            new_up_walls.0.push(new_wall);
        }

        // Down walls
        if !is_down {
            let new_wall = PerpWall::new(new_pos, tile_size.y);
            new_down_walls.0.push(new_wall);
        }
    }
    
    static_map.walls.right.append(new_right_walls);
    static_map.walls.left.append(new_left_walls);
    static_map.walls.up.append(new_up_walls);
    static_map.walls.down.append(new_down_walls);
    
    static_map.is_setup = true;
}
pub struct RegisterLdtkEntites;

impl Plugin for RegisterLdtkEntites {
    fn build(&self, app: &mut App) {
        app
            .register_ldtk_entity::<PlayerSpawnPoint>("PlayerSpawnPoint");
    }
}

#[derive(Component)]
pub struct MainStaticMap;
