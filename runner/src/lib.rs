mod world_collisions;
mod object;
mod properties;

use bevy::prelude::*;
pub use object::{Object, ObjectProperties};
pub use world_collisions::StaticMap;
pub use properties::LevelProperties;


pub struct EnginePlugin;
impl Plugin for EnginePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (world_collisions::show_debug, object::basic_object_system));
    }
}