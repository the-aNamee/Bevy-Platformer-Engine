use bevy::prelude::*;

#[derive(Resource)]
pub struct LevelProperties {
    pub gravity: Vec2,
    pub tile_size: Vec2
}

impl LevelProperties {
    // Basic values that aren't meant to be used.
    pub fn empty() -> LevelProperties {
        LevelProperties {
            gravity: Vec2::NEG_ONE * 10.0,
            tile_size: Vec2::ONE * 64.0
        }
    }

    pub fn set_gravity_strength(&mut self, strength: f32) {
        self.gravity = Vec2::NEG_Y * strength;
    }

    pub fn set_tile_size(&mut self, size: f32) {
        self.tile_size = Vec2::ONE * size;
    }
}