use bevy::prelude::*;
use bevy_framepace::FramepacePlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_ecs_ldtk::prelude::*;
use runner::{dir_for_each_many, EnginePlugin, LevelProperties, Direction, DIRECTIONAL_DIRS};
use camera::camera_system;
use level::{setup_collision_map_system, setup_level_system, RegisterLdtkEntites};
use player::{player_system, spawn_player_system};
mod camera;
mod level;
mod player;


use runner::Directional;


fn main() {
    let mut dir1 = Directional::new(30, 2, 54, 1);
    let mut dir2 = Directional::new(true, false, true, true);


    dir_for_each_many!(|(a, b, c): (&mut i32, &bool, &Direction)| {
        if *b {
            *a += 1;
            println!("{:?}", c);
        }
    }, (dir1, dir2, DIRECTIONAL_DIRS));
    
    println!("{:?}", dir1);


    App::new()
        .add_plugins((DefaultPlugins.set(ImagePlugin::default_nearest()), FramepacePlugin, EnginePlugin))
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(LdtkPlugin)
        .add_plugins(RegisterLdtkEntites)
        .insert_resource(LevelProperties::empty())
        .insert_resource(LevelSelection::index(0))
        .add_systems(Startup, setup_framerate)
        .add_systems(Startup, setup_level_system)
        .add_systems(Update, (spawn_player_system, setup_collision_map_system))
        .add_systems(Update, player_system)
        .add_systems(PostUpdate, camera_system)
        .run();
}


fn setup_framerate(
    mut frame_settings: ResMut<bevy_framepace::FramepaceSettings>,
) {
    frame_settings.limiter = bevy_framepace::Limiter::from_framerate(60.0);
}
