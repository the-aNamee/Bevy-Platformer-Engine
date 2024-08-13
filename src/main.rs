use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;



fn main() {
    // let object = object_manager::load_object_data_from_file("objects/dirt_tile.zip");
    
    // // Print the content of the extracted file
    // println!("{:?}", object.unwrap_or_default());

    // let mut input = String::new();
    // std::io::stdin().read_line(&mut input).expect("Failed to read line");

    App::new()
        .add_plugins((DefaultPlugins, editor::EditorPlugin, bevy_framepace::FramepacePlugin, object_manager::ObjectManagerPlugin))
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, setup_framerate)
        .run();
}


fn setup_framerate(
    mut frame_settings: ResMut<bevy_framepace::FramepaceSettings>,
) {
    frame_settings.limiter = bevy_framepace::Limiter::from_framerate(60.0);
}