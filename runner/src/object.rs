use crate::globals::{Directional};
use crate::world_collisions::{DagCollisionType, StaticMap};
use crate::properties::LevelProperties;
use crate::DIRECTIONAL_VEC2S;
use crate::Direction;
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
            // Perp walls
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



            // Diagonal walls
            let new = static_map.dag_walls.collide(ppos, cpos, object_properties.size, &mut gizmos);
            transform.translation = new.extend(transform.translation.z);
            cpos = new;
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



// Rotation settings
pub struct AngleSettings(pub Vec<AngleSetting>);

pub struct AngleSetting {
    pub angle: f32,
    pub collides_with: Direction,
    pub collision_type: DagCollisionType
}


impl AngleSettings {
    pub fn empty() -> AngleSettings {
        AngleSettings(Vec::new())
    }
    
    pub fn add_angle(&mut self, angle: f32, dir: Direction) {
        self.add_angle_full(angle, dir, DagCollisionType::from_dir(dir));
    }

    pub fn add_slide_angle(&mut self, angle: f32, dir: Direction) {
        self.add_angle_full(angle, dir, DagCollisionType::Close);
    }

    pub fn add_angle_full(&mut self, angle: f32, collides_with: Direction, collision_type: DagCollisionType) {
        let angle = angle.to_radians();
        self.0.push(AngleSetting {
            angle,
            collides_with,
            collision_type
        });
    }
}
