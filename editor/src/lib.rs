mod ui;
mod tilemap;
mod input;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use ui::MenuRes;

pub struct EditorPlugin;
impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(MenuRes::none())
            .add_plugins((TilemapPlugin, sickle_ui::SickleUiPlugin))
            .add_systems(Startup, (basic_setup, tilemap::_setup_debug_tilemaps, input::setup_input, ui::setup_ui_system))
            .add_systems(Update, (input::gamepad_connections, input::cursor_input_system, tilemap::modify_tiles, ui::ui_selection_system));
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