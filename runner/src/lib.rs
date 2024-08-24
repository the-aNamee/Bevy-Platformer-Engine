mod world_collisions;
mod object;

use bevy::prelude::*;
pub use object::{Object, ObjectProperties};


pub struct EnginePlugin;
impl Plugin for EnginePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, world_collisions::setup_level)
        .add_systems(Update, (world_collisions::show_debug, object::basic_object_system));
    }
}