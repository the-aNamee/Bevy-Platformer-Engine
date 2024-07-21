use crate::world_collisions::{StaticMap, DOWN_WALL_PUSH_DIRECTION, LEFT_WALL_PUSH_DIRECTION, RIGHT_WALL_PUSH_DIRECTION, UP_WALL_PUSH_DIRECTION};

use bevy::{color::palettes::basic::RED, math::vec2, prelude::*};


pub fn _setup_debug_objects(
    mut commands: Commands
) {
    let size = vec2(26.0, 48.0);
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::Srgba(RED),
                custom_size: Some(size),
                ..default()
            },
            transform: Transform {
                translation: vec2(64.0, 148.0).extend(0.0),
                ..default()
            },
            ..default()
        },
        Object {
            velocity: vec2(0.0, 0.0),
            ..default()
        },
        ObjectProperties {
            size,
            gravity: vec2(0.0, -50.0)
        },
        _DebugPLayer
    ));
}

pub fn basic_object_system(
    mut object_query: Query<(&mut Object, &mut Transform, &ObjectProperties)>,
    wall_query: Query<&StaticMap>,
    time: Res<Time>,
    mut gizmos: Gizmos
) {  
    for (mut object, mut transform, object_properties) in &mut object_query {
        let ppos = transform.translation.truncate();
        
        object.velocity += object_properties.gravity * time.delta_seconds() * 0.5;
        transform.translation += object.velocity.extend(0.0) * time.delta_seconds();
        object.velocity += object_properties.gravity * time.delta_seconds() * 0.5;

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

pub fn _debug_player_system(
    mut query: Query<&mut Object, With<_DebugPLayer>>,
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>
) {
    let mut player = query.single_mut();
    
    let movement_input = (input.pressed(KeyCode::KeyD) as i8 - input.pressed(KeyCode::KeyA) as i8) as f32;
    let jump_input = input.just_pressed(KeyCode::Space);
    player.velocity.x += movement_input * 750.0 * time.delta_seconds();

    //// println!("On floor = {}", player.is_on_floor);
    
    if jump_input && player.is_on_floor {
        player.velocity.y = 100.0;
    }

    player.velocity.x *= (0.1 as f32).powf(time.delta_seconds());
}

#[derive(Component)]
pub struct _DebugPLayer;

#[derive(Component, Default)]
pub struct Object {
    velocity: Vec2,
    is_on_floor: bool,
    is_on_ceiling: bool,
    is_on_right_wall: bool,
    is_on_left_wall: bool
} 

#[derive(Component)]
pub struct ObjectProperties {
    size: Vec2,
    gravity: Vec2
}