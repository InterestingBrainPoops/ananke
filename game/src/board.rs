use std::{num, ops::AddAssign};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

impl Coordinate {
    pub fn new(x: i32, y: i32) -> Coordinate {
        Coordinate { x, y }
    }
}

impl AddAssign for Coordinate {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

#[derive(Clone)]
pub struct Snake {
    pub alive: bool,
    pub id: usize,
    pub health: i32,
    pub body: Vec<Coordinate>,
}

#[derive(Clone)]
pub struct Board {
    pub width: i32,
    pub height: i32,
    pub snakes: Vec<Snake>,
    pub food: Vec<Coordinate>,
    pub hazards: Vec<Coordinate>,
}

impl Board {
    pub fn num_alive(&self) -> usize {
        return self
            .snakes
            .iter()
            .map(|snake| if snake.alive { 1 } else { 0 })
            .sum::<usize>();
    }
    pub fn game_over(&self) -> bool {
        let num_alive = self.num_alive();

        return num_alive == 1 || num_alive == 0;
    }
}
