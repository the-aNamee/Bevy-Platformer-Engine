use crate::globals::{Directional, DIRECTIONAL_DIRS};
use crate::world_collisions::{StaticMap, DOWN_WALL_PUSH_DIRECTION, LEFT_WALL_PUSH_DIRECTION, RIGHT_WALL_PUSH_DIRECTION, UP_WALL_PUSH_DIRECTION};
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

        let cpos = transform.translation.truncate();

        for static_map in &wall_query {
            // for dir in DIRECTIONAL_DIRS.iter() {
            //     let dir_float = DIRECTIONAL_VEC2S.from_dir(dir);
            //     let dir_float = dir_float.x + dir_float.y;
            //     let p = if dir.is_horizontal() { ppos.x } else { ppos.y };
            //     let c = if dir.is_horizontal() { cpos.x } else { cpos.y };
            //     let alt_c = if dir.is_horizontal() { cpos.y } else { cpos.x };

            //     if input.just_pressed(KeyCode::KeyK) {
            //         println!("{}", dir.is_vertical());
            //     }
                

            //     let (new, done) = static_map.walls.up.collide(p, c, alt_c, object_properties.size, dir_float, dir.is_vertical(), &mut gizmos);
            //     if dir.is_horizontal() {
            //         transform.translation.x = new;
            //         object.velocity.x = if done {0.0} else {object.velocity.x};
            //     } else {
            //         transform.translation.y = new;
            //         object.velocity.y = if done {0.0} else {object.velocity.y};
            //     }
            //     *object.is_on_wall.mut_from_dir(dir) = done;
            // }

            if object.velocity.y.is_sign_negative() {
                let (new, done) = static_map.walls.up.collide(ppos.y, cpos.y, cpos.x, object_properties.size, UP_WALL_PUSH_DIRECTION.y, true, &mut gizmos);
                transform.translation.y = new;
                object.velocity.y = if done {0.0} else {object.velocity.y};
                object.is_on_wall.down = done;
                object.is_on_wall.up = false;
            } else {
                let (new, done) = static_map.walls.down.collide(ppos.y, cpos.y, cpos.x, object_properties.size, DOWN_WALL_PUSH_DIRECTION.y, true, &mut gizmos);
                transform.translation.y = new;
                object.velocity.y = if done {0.0} else {object.velocity.y};
                object.is_on_wall.up = done;
                object.is_on_wall.down = false;
            }

            if object.velocity.x.is_sign_positive() {
                let (new, done) = static_map.walls.left.collide(ppos.x, cpos.x, ppos.y, object_properties.size.yx(), RIGHT_WALL_PUSH_DIRECTION.x, false, &mut gizmos);
                transform.translation.x = new;
                object.velocity.x = if done {0.0} else {object.velocity.x};
                object.is_on_wall.right = done;
                object.is_on_wall.left = false;
            } else {
                let (new, done) = static_map.walls.right.collide(ppos.x, cpos.x, ppos.y, object_properties.size.yx(), LEFT_WALL_PUSH_DIRECTION.x, false, &mut gizmos);
                transform.translation.x = new;
                object.velocity.x = if done {0.0} else {object.velocity.x};
                object.is_on_wall.left = done;
                object.is_on_wall.right = false;
            }
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