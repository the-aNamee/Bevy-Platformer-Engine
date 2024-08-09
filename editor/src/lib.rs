mod tilemap;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

pub struct EditorPlugin;
impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(TilemapPlugin)
            .add_systems(Startup, (basic_setup, tilemap::_setup_debug_tilemaps));
    }
}

fn basic_setup(
    mut commands: Commands
) {
    // Spawn camera
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scale: 2.0,
            ..default()
        },
        ..default()
    });
}