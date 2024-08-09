mod tilemap;
mod input;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

pub struct EditorPlugin;
impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(TilemapPlugin)
            .add_systems(Startup, (basic_setup, tilemap::_setup_debug_tilemaps, input::setup_input))
            .add_systems(Update, (input::gamepad_connections, input::input_system, tilemap::modify_tiles));
    }
}

fn basic_setup(
    mut commands: Commands
) {
    // Spawn camera
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scale: 1.0,
            near: -1000.0,
            ..default()
        },
        ..default()
    });
}