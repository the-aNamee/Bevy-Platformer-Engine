use bevy::prelude::*;
use bevy_framepace::FramepacePlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use runner::EnginePlugin;
use level::setup_level_system;
use player::player_system;
mod level;
mod player;


fn main() {
    App::new()
        .add_plugins((DefaultPlugins, FramepacePlugin, EnginePlugin))
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, setup_framerate)
        .add_systems(Startup, setup_level_system)
        .add_systems(Update, player_system)
        .run();
}


fn setup_framerate(
    mut frame_settings: ResMut<bevy_framepace::FramepaceSettings>,
) {
    frame_settings.limiter = bevy_framepace::Limiter::from_framerate(60.0);
}
