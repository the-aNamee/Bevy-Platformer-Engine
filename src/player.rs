use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use runner::Object;

const FRICTION: f32 = 0.002;
const JUMP_VELOCITY: f32 = 100.0;
const SPEED: f32 = 1000.0;

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
    mut query: Query<(&mut Object, &mut Sprite), With<Player>>,
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>
) {
    let (mut player_object, mut player_sprite) = query.single_mut();
    
    let movement_input = (input.pressed(KeyCode::KeyD) as i8 - input.pressed(KeyCode::KeyA) as i8) as f32;
    let jump_input = input.just_pressed(KeyCode::Space);
    player_object.velocity.x += movement_input * SPEED * time.delta_seconds();


    // Flip sprite
    if movement_input != 0.0 {
        player_sprite.flip_x = if movement_input == -1.0 { true } else { false };
    }
    
    if player_object.is_on_wall.down && jump_input {
        player_object.velocity.y = JUMP_VELOCITY;
    }

    player_object.velocity.x *= FRICTION.powf(time.delta_seconds());
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