
use bevy::{color::palettes::{basic::*, css::{ORANGE, PINK}}, math::vec2, prelude::*};
use crate::globals::Direction;

// use crate::object;
use crate::globals::Directional;




#[derive(Component, Debug)]
pub struct StaticMap {
    pub perp_walls: Directional<PerpWalls>,
    pub dag_walls: DagWalls,
    pub is_setup: bool
}

impl StaticMap {
    pub fn empty() -> StaticMap {
        StaticMap {
            perp_walls: Directional::new_all(PerpWalls::empty()),
            dag_walls: DagWalls::default(),
            is_setup: false
        }
    }
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
        do_stuff(&mut gizmos, &static_map.perp_walls.up, vec2(1.0, 0.0), BLUE);
        do_stuff(&mut gizmos, &static_map.perp_walls.down, vec2(1.0, 0.0), GREEN);
        do_stuff(&mut gizmos, &static_map.perp_walls.right, vec2(0.0, 1.0), YELLOW);
        do_stuff(&mut gizmos, &static_map.perp_walls.left, vec2(0.0, 1.0), RED);

        // Dag walls
        for wall in static_map.dag_walls.0.iter() {
            gizmos.line_2d(wall.pos_a, wall.pos_b, PURPLE);
        }
    }
}



// Perpindicuar walls

#[derive(Debug, Clone)]
pub struct PerpWalls(pub Vec<PerpWall>);

#[derive(Debug, Clone)]
pub struct PerpWall {
    position: Vec2,
    length: f32
}


impl PerpWalls {
    pub fn collide(&self, previous_object_pos: f32, current_object_pos: f32, object_slide_pos: f32, object_size: Vec2, wall_push_direction: f32, vertical_wall: bool, _gizmos: &mut Gizmos) -> (f32, bool) {
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

pub fn append(&mut self, other: PerpWalls) {
    self.0.append(&mut other.0.clone());
}

    pub fn empty() -> PerpWalls {
        PerpWalls(Vec::new())
    }
}

impl PerpWall {
    pub fn new(position: Vec2, length: f32) -> Self {
        PerpWall {
            position: position,
            length: length
        }
    }
}



// DagWall

#[derive(Default, Debug, Clone)]
pub struct DagWalls(pub Vec<DagWall>);


#[derive(Debug, Clone)]
pub struct DagWall {
    pub pos_a: Vec2,
    pub pos_b: Vec2
}


impl DagWalls {
    pub fn empty() -> DagWalls {
        DagWalls(Vec::default())
    }

    pub fn add(&mut self, pos_a: Vec2, pos_b: Vec2) {
        self.0.push(DagWall {
            pos_a,
            pos_b
        });
    }

    pub fn collide(&self, ppos: Vec2, cpos: Vec2, size: Vec2, _gizmos: &mut Gizmos) -> Vec2 {
        for wall in self.0.iter() {
            let dir = normalize_weird(wall.pos_b - wall.pos_a);
            let point_dir = vec2(dir.y, -dir.x);
            let ppoint = ppos + size * point_dir / 2.0; 
            let cpoint = cpos + size * point_dir / 2.0; // This is the point of the only part of the object that will actully collide.
      

            
            // Get the slope.
            let m = (wall.pos_a.y - wall.pos_b.y) /* rise over */ / (wall.pos_a.x - wall.pos_b.x) /* run */;
            // Get the y-intercept.
            let b = wall.pos_a.y - m * wall.pos_a.x;

            let pin = ppoint.y < m * ppoint.x + b;
            let cin = cpoint.y < m * cpoint.x + b;
            
            if !pin && cin {
                let npoint /* New point */ = vec2(cpoint.x, m * cpoint.x + b);
                let npos = npoint + size * point_dir * Vec2::NEG_ONE / 2.0;
                return npos;
            }
        }
        return cpos;
    }
}

fn normalize_weird(a: Vec2) -> Vec2 {
    vec2((a.x > 0.0) as i32 as f32 * 2.0 - 1.0, (a.y > 0.0) as i32 as f32 * 2.0 - 1.0)
}

pub enum DagCollisionType {
    Vertical,
    Horizontal,
    Close
}

impl DagCollisionType {
    pub fn from_dir(dir: Direction) -> DagCollisionType {
        if dir.is_horizontal() { DagCollisionType::Horizontal } else { DagCollisionType::Vertical }
    }
}