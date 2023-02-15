use crate::board::{Board, SnakeID};

#[derive(Clone, Copy)]
pub struct Move {
    pub direction: Direction,
    pub id: SnakeID,
}

#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub trait Ruleset {
    // initialize a new board, and reset the ruleset's internal state (if any)
    fn initialize_board(&mut self) -> Board;
    // apply a turn to a board, including food spawns, deaths, etc.
    fn step_board(&mut self, moves: Vec<Move>, board: Board) -> Result<Board, &str>;
}
