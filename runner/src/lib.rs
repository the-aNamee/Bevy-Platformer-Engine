mod world_collisions;
mod object;
mod properties;
mod globals;

use bevy::prelude::*;
pub use object::{Object, ObjectProperties};
pub use world_collisions::{StaticMap, PerpWall, PerpWalls};
pub use properties::LevelProperties;
pub use globals::{Directional, Direction, DIRECTIONAL_DIRS, DIRECTIONAL_VEC2S, ALL_DIRS};


pub struct EnginePlugin;
impl Plugin for EnginePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, object::basic_object_system);
    }
}