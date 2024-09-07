use bevy::{color::palettes::basic::*, math::vec2, prelude::*};
use crate::properties::LevelProperties;

// use crate::object;

pub const UP_WALL_PUSH_DIRECTION: Vec2 = vec2(0.0, 1.0);
pub const DOWN_WALL_PUSH_DIRECTION: Vec2 = vec2(0.0, -1.0);
pub const RIGHT_WALL_PUSH_DIRECTION: Vec2 = vec2(-1.0, 0.0);
pub const LEFT_WALL_PUSH_DIRECTION: Vec2 = vec2(1.0, 0.0);


#[derive(Default, Component, Debug)]
pub struct StaticMap {
    pub right_walls: PerpWalls,
    pub left_walls: PerpWalls,
    pub up_walls: PerpWalls,
    pub down_walls: PerpWalls,
}

impl StaticMap {
    pub fn _debug_test() -> Self {
        let mut up_walls = PerpWalls(Vec::new());
        up_walls.0.push(PerpWall::new_tiled(vec2(1.0, 0.0), 4.0));
        // up_walls.0.push(PerpWall::new_tiled(vec2(3.0, 2.0), 0.0));``
        up_walls.0.push(PerpWall::new_tiled(vec2(5.0, -3.0), 1.0));

        let mut down_walls = PerpWalls(Vec::new());
        down_walls.0.push(PerpWall::new_tiled(vec2(-1.0, 3.0), 2.0));

        let mut right_walls = PerpWalls(Vec::new());
        right_walls.0.push(PerpWall::new_tiled(vec2(5.0, 3.0), 2.0));

        let mut left_walls = PerpWalls(Vec::new());
        left_walls.0.push(PerpWall::new_tiled(vec2(1.0, 3.0), 2.0));

        StaticMap {
            up_walls,
            down_walls,
            right_walls,
            left_walls
        }
    }
}

pub fn setup_level(
    mut commands: Commands
) {
    commands.spawn(StaticMap::_debug_test());
}

pub fn show_debug(
    mut gizmos: Gizmos,
    static_maps: Query<&StaticMap>
) {
    fn do_stuff(gizmos: &mut Gizmos, walls: &PerpWalls, direction: Vec2, color: Srgba) {
        for wall in &walls.0 {
            let pos1 = wall.position;
            let pos2 = pos1 + direction * wall.length;
            gizmos.line_2d(pos1, pos2, color);
        }

    }

    for static_map in &static_maps {
        do_stuff(&mut gizmos, &static_map.up_walls, vec2(1.0, 0.0), BLUE);
        do_stuff(&mut gizmos, &static_map.down_walls, vec2(1.0, 0.0), GREEN);
        do_stuff(&mut gizmos, &static_map.right_walls, vec2(0.0, 1.0), YELLOW);
        do_stuff(&mut gizmos, &static_map.left_walls, vec2(0.0, 1.0), RED);
    }
}

#[derive(Default, Debug, Clone)]
pub struct PerpWalls(Vec<PerpWall>);

impl PerpWalls {
    pub fn collide(&self, previous_object_pos: f32, current_object_pos: f32, object_slide_pos: f32, object_size: Vec2, wall_push_direction: f32, vertical_wall: bool, gizmos: &mut Gizmos) -> (f32, bool) {
        // 'Real' means the position that we care about.
        let pre_real_pos = previous_object_pos - (object_size.y / 2.0) * wall_push_direction;
        let cur_real_pos = current_object_pos - (object_size.y / 2.0) * wall_push_direction;

        // Gets both sides of the platform.
        let object_min_slide_pos = object_slide_pos - object_size.x / 2.0;
        let object_max_slide_pos = object_slide_pos + object_size.x / 2.0;

        // gizmos.line_2d(vec2(0.0, pre_real_pos), vec2(32.0, cur_real_pos), GREEN);
        // gizmos.line_2d(vec2(object_min_slide_pos, 0.0), vec2(object_max_slide_pos, 32.0), PURPLE);

        // Possibility is the best wall to position to put the object.
        let mut possibility = None;
        for wall in &self.0 {
            let wall_position = if vertical_wall {wall.position.y} else {wall.position.x};
            let was_in_wall = pre_real_pos * wall_push_direction < wall_position * wall_push_direction;
            let is_in_wall = cur_real_pos * wall_push_direction < wall_position * wall_push_direction;

            let min_wall_slide_pos = if vertical_wall {wall.position.x} else {wall.position.y};
            let max_wall_slide_pos = min_wall_slide_pos + wall.length;
            let is_off_min = object_max_slide_pos <= min_wall_slide_pos;
            let is_off_max = object_min_slide_pos >= max_wall_slide_pos;

            if !was_in_wall && is_in_wall && !is_off_min && !is_off_max && if possibility.is_none() {true} else {wall_position * wall_push_direction > possibility.unwrap() * wall_push_direction} {
                //// println!("Collide");
                possibility = Some(wall_position);
            }
        }

        if possibility.is_some() {
            return (possibility.unwrap() + object_size.y * 0.5 * wall_push_direction, true);
        } else {
            return (current_object_pos, false);
        }
    }
}

#[derive(Debug, Clone)]
pub struct PerpWall {
    position: Vec2,
    length: f32
}

impl PerpWall {
    fn new_tiled(position: Vec2, length: f32) -> Self {
        PerpWall {
            position: position,
            length: length
        }
    }
}