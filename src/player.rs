use bevy::{prelude::*, render::camera};
use bevy_ecs_ldtk::prelude::*;
use bevy_ecs_tilemap::helpers::transform;
use runner::Object;


pub fn spawn_player_system(
    spawn_points: Query<&Transform, With<PlayerSpawnPoint>>,
    mut players: Query<(&mut Transform, &mut Player),  Without<PlayerSpawnPoint>>
  ) {
    // println!("There are {} spawn points.", spawn_points.iter().count());
    // println!("There are {} players.", players.iter().count());
    let Ok(spawn_point) = spawn_points.get_single() else {
        println!("Not only one PlayerSpawnPoint");
        return;
    };
    
    let (mut player_transform, mut player) = players.single_mut();

    if player.spawned {
        return;
    }    

    player_transform.translation = spawn_point.translation;

    player.spawned = true;
  }

pub fn player_system(
    mut query: Query<(&mut Object, &Transform), With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>
) {
    let (mut player_object, player_transform) = query.single_mut();
    let mut camera_transform = camera_query.single_mut();
    
    let movement_input = (input.pressed(KeyCode::KeyD) as i8 - input.pressed(KeyCode::KeyA) as i8) as f32;
    let jump_input = input.just_pressed(KeyCode::Space);
    player_object.velocity.x += movement_input * 750.0 * time.delta_seconds();
    
    if jump_input && player_object.is_on_floor {
        player_object.velocity.y = 100.0;
    }

    player_object.velocity.x *= (0.1 as f32).powf(time.delta_seconds());

    camera_transform.translation = player_transform.translation;
}

#[derive(Component, Default)]
pub struct Player {
  spawned: bool
}

#[derive(Default, Component, LdtkEntity)]
pub struct PlayerSpawnPoint {
    player: Player,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: LdtkSpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords
}