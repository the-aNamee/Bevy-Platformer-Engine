use crate::world_collisions::{StaticMap, DOWN_WALL_PUSH_DIRECTION, LEFT_WALL_PUSH_DIRECTION, RIGHT_WALL_PUSH_DIRECTION, UP_WALL_PUSH_DIRECTION};
use crate::properties::LevelProperties;
use bevy::prelude::*;




pub fn basic_object_system(
    mut object_query: Query<(&mut Object, &mut Transform, &ObjectProperties)>,
    level_properties: Res<LevelProperties>,
    wall_query: Query<&StaticMap>,
    time: Res<Time>,
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

        let cpos = transform.translation.truncate();

        for static_map in &wall_query {
            if object.velocity.y.is_sign_negative() {
                let (new, done) = static_map.up_walls.collide(ppos.y, cpos.y, cpos.x, object_properties.size, UP_WALL_PUSH_DIRECTION.y, true, &mut gizmos);
                transform.translation.y = new;
                object.velocity.y = if done {0.0} else {object.velocity.y};
                object.is_on_floor = done;
                object.is_on_ceiling = false;
            } else {
                let (new, done) = static_map.down_walls.collide(ppos.y, cpos.y, cpos.x, object_properties.size, DOWN_WALL_PUSH_DIRECTION.y, true, &mut gizmos);
                transform.translation.y = new;
                object.velocity.y = if done {0.0} else {object.velocity.y};
                object.is_on_ceiling = done;
                object.is_on_floor = false;
            }

            if object.velocity.x.is_sign_positive() {
                let (new, done) = static_map.left_walls.collide(ppos.x, cpos.x, ppos.y, object_properties.size.yx(), RIGHT_WALL_PUSH_DIRECTION.x, false, &mut gizmos);
                transform.translation.x = new;
                object.velocity.x = if done {0.0} else {object.velocity.x};
                object.is_on_right_wall = done;
                object.is_on_left_wall = false;
            } else {
                let (new, done) = static_map.right_walls.collide(ppos.x, cpos.x, ppos.y, object_properties.size.yx(), LEFT_WALL_PUSH_DIRECTION.x, false, &mut gizmos);
                transform.translation.x = new;
                object.velocity.x = if done {0.0} else {object.velocity.x};
                object.is_on_left_wall = done;
                object.is_on_right_wall = false;
            }
        }
    }
}



#[derive(Component)]
pub struct _DebugPLayer;

#[derive(Component, Reflect)]
pub struct Object {
    pub velocity: Vec2,
    pub is_on_floor: bool,
    pub is_on_ceiling: bool,
    pub is_on_right_wall: bool,
    pub is_on_left_wall: bool
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
            is_on_floor: false,
            is_on_ceiling: false,
            is_on_right_wall: false,
            is_on_left_wall: false
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