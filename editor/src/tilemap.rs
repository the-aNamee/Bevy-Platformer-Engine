use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

const TILE_SIZE: TilemapTileSize = TilemapTileSize::new(32.0, 32.0);

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

fn do_tiles(
    asset_server: Res<AssetServer>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut TilemapTexture, &mut Visibility)>
) {

}