use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use runner::Object;


pub fn spawn_player_system( 
    commands: Commands
) {}

pub fn player_system(
    mut query: Query<&mut Object, With<Player>>,
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>
) {
    let mut player = query.single_mut();
    
    let movement_input = (input.pressed(KeyCode::KeyD) as i8 - input.pressed(KeyCode::KeyA) as i8) as f32;
    let jump_input = input.just_pressed(KeyCode::Space);
    player.velocity.x += movement_input * 750.0 * time.delta_seconds();
    
    if jump_input && player.is_on_floor {
        player.velocity.y = 100.0;
    }

    player.velocity.x *= (0.1 as f32).powf(time.delta_seconds());
}

#[derive(Component, Default)]
pub struct Player;

#[derive(Default, Bundle, LdtkEntity)]
pub struct PlayerSpawnPoint {
    player: Player,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: LdtkSpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords
}