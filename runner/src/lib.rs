mod world_collisions;
mod object;

use bevy::prelude::*;



pub struct DebugEnginePlugin;
impl Plugin for DebugEnginePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_general, world_collisions::setup_level, object::_setup_debug_objects))
        .add_systems(Update, (world_collisions::show_debug, object::basic_object_system, object::_debug_player_system));
    }
}

fn setup_general(
    mut commands: Commands
) {
    // Spawn camera
    commands.spawn(Camera2dBundle {
        ..default()
    });
}