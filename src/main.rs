use bevy::prelude::*;
use bevy_editor_pls::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, editor::EditorPlugin, bevy_framepace::FramepacePlugin))
        // .add_plugins(EditorPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, setup_framerate)
        .run();
}


fn setup_framerate(
    mut frame_settings: ResMut<bevy_framepace::FramepaceSettings>,
) {
    frame_settings.limiter = bevy_framepace::Limiter::from_framerate(60.0);
}