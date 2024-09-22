use bevy::prelude::*;
use bevy_framepace::FramepacePlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_ecs_ldtk::prelude::*;
use runner::{EnginePlugin, LevelProperties};
use level::{setup_collision_map_system, setup_level_system, RegisterLdtkEntites};
use player::{player_system, spawn_player_system};
mod level;
mod player;


fn main() {
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
        .run();
}


fn setup_framerate(
    mut frame_settings: ResMut<bevy_framepace::FramepaceSettings>,
) {
    frame_settings.limiter = bevy_framepace::Limiter::from_framerate(60.0);
}
