#[derive(Debug)]
pub struct Directional<T> {
    right: T,
    left: T,
    up: T,
    down: T
}

impl<T> Directional<T> {
    pub fn new(right: T, left: T, up: T, down: T) -> Directional<T>{
        Directional {
            right,
            left,
            up,
            down
        }
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
}