use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::input::Cursor;

pub const TILE_SIZE: TilemapTileSize = TilemapTileSize::new(32.0, 32.0);

pub fn _setup_debug_tilemaps(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    array_texture_loader: Res<ArrayTextureLoader>
) {
    let map_size = TilemapSize::new(32, 32);

    let ground_texture_entity = asset_server.load("ground.png");

    let tilemap_entity = commands.spawn_empty().id();

    let mut tile_storage = TileStorage::empty(map_size);
    
    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let position = TilePos::new(x, y);
            let tile_entity = commands.spawn(
                TileBundle {
                    position,
                    tilemap_id: TilemapId(tilemap_entity),
                    ..default()
                }
            ).id();
            tile_storage.set(&position, tile_entity);
        }
    }


    let grid_size = TILE_SIZE.into();
    let map_type = TilemapType::default();

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(ground_texture_entity.clone()),
        tile_size: TILE_SIZE,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        ..default()
    });


    array_texture_loader.add(TilemapArrayTexture {
        texture: TilemapTexture::Single(ground_texture_entity),
        tile_size: TILE_SIZE,
        ..default()
    })
}

pub fn modify_tiles(
    asset_server: Res<AssetServer>,
    mut tile_query: Query<(&TilePos, &mut TileColor)>,
    tilemap_query: Query<&Transform, With<TilemapType>>,
    cursor_query: Query<&Transform, With<Cursor>>
) {
    let tilemap_transform = tilemap_query.single();

    let cursor_transform = cursor_query.single();
    let cursor_tile_pos = get_tile_pos(cursor_transform.translation.xy(), tilemap_transform.translation.xy(), TILE_SIZE);

    for (tile_pos, mut tile_color) in tile_query.iter_mut() {
        if cursor_tile_pos == *tile_pos {
            tile_color.0.set_alpha(0.5);
        } else {
            tile_color.0.set_alpha(1.0);
        }
    }
}

fn get_tile_pos(pos: Vec2, offset: Vec2, tile_size: TilemapTileSize) -> TilePos {
    let tile_size_into: Vec2 = tile_size.into();
    let adjusted_pos = (pos - offset) / tile_size_into;
    let rounded_x = adjusted_pos.x.round().max(0.0) as u32;
    let rounded_y = adjusted_pos.y.round().max(0.0) as u32;
    let real_output = TilePos::new(rounded_x, rounded_y);
    
    // println!(
    //     "real_output: {:?}",
    //     real_output
    // );
    
    real_output
}
