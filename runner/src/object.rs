use crate::globals::{Directional};
use crate::world_collisions::StaticMap;
use crate::properties::LevelProperties;
use crate::DIRECTIONAL_VEC2S;
use bevy::prelude::*;




pub fn basic_object_system(
    mut object_query: Query<(&mut Object, &mut Transform, &ObjectProperties)>,
    level_properties: Res<LevelProperties>,
    wall_query: Query<&StaticMap>,
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut gizmos: Gizmos
) {
    for (mut object, mut transform, object_properties) in &mut object_query {
        let ppos = transform.translation.truncate();
        
        let a = object_properties.gravity_multiplier;
        let b = level_properties.gravity;
        let real_gravity = a * b;
        
        object.velocity += real_gravity * time.delta_seconds() * 0.5;
        transform.translation += object.velocity.extend(0.0) * time.delta_seconds();
        object.velocity += real_gravity * time.delta_seconds() * 0.5;

        let mut cpos = transform.translation.truncate();

        for static_map in &wall_query {
            let (new, done) = static_map.perp_walls.up.collide(ppos.y, cpos.y, cpos.x, object_properties.size, DIRECTIONAL_VEC2S.up.y, true, &mut gizmos);
            transform.translation.y = new;
            cpos.y = new;
            object.velocity.y = if done {0.0} else {object.velocity.y};
            object.is_on_wall.down = done;


            let (new, done) = static_map.perp_walls.down.collide(ppos.y, cpos.y, cpos.x, object_properties.size, DIRECTIONAL_VEC2S.down.y, true, &mut gizmos);
            transform.translation.y = new;
            cpos.y = new;
            object.velocity.y = if done {0.0} else {object.velocity.y};
            object.is_on_wall.up = done;


            let (new, done) = static_map.perp_walls.left.collide(ppos.x, cpos.x, ppos.y, object_properties.size.yx(), -1.0, false, &mut gizmos);
            transform.translation.x = new;
            cpos.x = new;
            object.velocity.x = if done {0.0} else {object.velocity.x};
            object.is_on_wall.right = done;


            let (new, done) = static_map.perp_walls.right.collide(ppos.x, cpos.x, ppos.y, object_properties.size.yx(), 1.0, false, &mut gizmos);
            transform.translation.x = new;
            cpos.x = new;
            object.velocity.x = if done {0.0} else {object.velocity.x};
            object.is_on_wall.left = done;
        }
    }
}



#[derive(Component)]
pub struct _DebugPLayer;

#[derive(Component, Reflect)]
pub struct Object {
    pub velocity: Vec2,
    pub is_on_wall: Directional<bool>
} 

#[derive(Component, Reflect)]
pub struct ObjectProperties {
    pub size: Vec2,
    pub gravity_multiplier: Vec2
}


impl Object {
    pub fn basic() -> Object {
        Object {
            velocity: Vec2::ZERO,
            is_on_wall: Directional::new_all(false)
        }
    }
}

impl ObjectProperties {
    pub fn new(size: Vec2) -> ObjectProperties {
        ObjectProperties {
            size,
            gravity_multiplier: Vec2::ONE
        }
    }
}