use bevy::prelude::*;



pub const DIRECTIONAL_VEC2S: Directional<Vec2> = Directional::new(Vec2::X, Vec2::NEG_X, Vec2::Y, Vec2::NEG_Y);
pub const DIRECTIONAL_DIRS: Directional<Direction> = Directional::new(Direction::Right, Direction::Left, Direction::Up, Direction::Down);
pub const ALL_DIRS: [Direction; 4] = [Direction::Right, Direction::Left, Direction::Up, Direction::Down];


#[derive(Debug, Reflect, Clone, Copy)]
pub struct Directional<T> {
    pub right: T,
    pub left: T,
    pub up: T,
    pub down: T
}

impl<T> Directional<T> {
    pub const fn new(right: T, left: T, up: T, down: T) -> Directional<T>{
        Directional {
            right,
            left,
            up,
            down
        }
    }

    pub fn from_dir(&self, dir: Direction) -> &T {
        if dir == Direction::Right { &self.right }
        else if dir == Direction::Left { &self.left }
        else if dir == Direction::Up { &self.up }
        else { &self.down }
    }

    pub fn mut_from_dir(&mut self, dir: Direction) -> &mut T {
        if dir == Direction::Right { &mut self.right }
        else if dir == Direction::Left { &mut self.left }
        else if dir == Direction::Up { &mut self.up }
        else { &mut self.down }
    }

    pub fn for_each<F>(&self, mut f: F)
    where 
        F: FnMut(&T, Vec2, Direction),
    {
        f(&self.right, Vec2::X, Direction::Right);
        f(&self.left, Vec2::NEG_X, Direction::Left);
        f(&self.up, Vec2::Y, Direction::Up);
        f(&self.down, Vec2::NEG_Y, Direction::Down);
    }

    pub fn for_each_mut<F>(&mut self, mut f: F) 
    where 
        F: FnMut(&mut T, Vec2, Direction), // Use FnMut to allow multiple calls
    {
        f(&mut self.right, Vec2::X, Direction::Right);
        f(&mut self.left, Vec2::NEG_X, Direction::Left);
        f(&mut self.up, Vec2::Y, Direction::Up);
        f(&mut self.down, Vec2::NEG_Y, Direction::Down);
    }
}



impl<T: Clone> Directional<T> {
    pub fn new_all(all: T) -> Directional<T>{
        Directional {
            right: all.clone(),
            left: all.clone(),
            up: all.clone(),
            down: all
        }
    }

    pub fn iter(&self) -> [T; 4] {
        let selfin = self.clone();
        [selfin.right, selfin.left, selfin.up, selfin.down]
    }
}





#[macro_export]
macro_rules! dir_for_each_many {
    // Nothing exists. I should prolly just panic.
    ($f:expr, ) => {};

    // More than nothing exists.
    ($f:expr, ($($dirs:expr),+)) => {
        $f(($(&mut $dirs.right),+));
        $f(($(&mut $dirs.left),+));
        $f(($(&mut $dirs.up),+));
        $f(($(&mut $dirs.down),+));
    }
}





#[derive(Debug, Reflect, PartialEq, Clone, Copy)]
pub enum Direction {
    Right,
    Left,
    Up,
    Down
}


impl Direction {
    pub fn is_vertical(&self) -> bool {
        *self == Direction::Up || *self == Direction::Down
    }

    pub fn is_horizontal(&self) -> bool {
        *self == Direction::Right || *self == Direction::Left
    }

    pub fn opposite(&self) -> Direction {
        let mut response = Direction::Up;
        let oppo_dirs = Directional::new(Direction::Left, Direction::Right, Direction::Down, Direction::Up);
        
        for dir in DIRECTIONAL_DIRS.iter() {
            if *self == dir {
                response = *oppo_dirs.from_dir(dir);
                break;
            }
        }

        response
    }
}

