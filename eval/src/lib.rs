pub mod evaluation;

use game::{
    board::{Board, SnakeID},
    ruleset::Move,
};
use vecmath::Vector;

pub trait Eval<const N: usize> {
    fn eval(board: &Board, you: SnakeID) -> Vector<N>;
    fn get_move(board: &Board, you: SnakeID, weights: Vector<N>) -> Move;
}
