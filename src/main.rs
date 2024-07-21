use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, runner::DebugEnginePlugin, bevy_framepace::FramepacePlugin))
        .add_systems(Startup, setup_framerate)
        .run();
}




fn setup_framerate(
    mut frame_settings: ResMut<bevy_framepace::FramepaceSettings>,
) {
    frame_settings.limiter = bevy_framepace::Limiter::from_framerate(60.0);
}