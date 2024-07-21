mod world_collisions;
mod object;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, DebugEnginePlugin, bevy_framepace::FramepacePlugin))
        .run();
}


pub struct DebugEnginePlugin;
impl Plugin for DebugEnginePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_general, world_collisions::setup_level, object::_setup_debug_objects))
        .add_systems(Update, (world_collisions::show_debug, object::basic_object_system, object::_debug_player_system));
    }
}

fn setup_general(
    mut frame_settings: ResMut<bevy_framepace::FramepaceSettings>,
    mut commands: Commands
) {
    // Spawn camera
    commands.spawn(Camera2dBundle {
        ..default()
    });

    frame_settings.limiter = bevy_framepace::Limiter::from_framerate(60.0);

}