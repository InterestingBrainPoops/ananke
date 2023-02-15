use game::{
    board::Coordinate,
    ruleset::{Direction, Move},
};
use vecmath::Vector;

use crate::Eval;

pub struct AreaControlEval;

impl Eval<4> for AreaControlEval {
    fn eval(board: &game::board::Board, you: game::board::SnakeID) -> Vector<4> {
        if board.game_over() {
            return Vector::<4>::new(0.0);
        }
        /*    [
            // done
            length_difference as f64,
            // done
            distance_to_center as f64,
            // done
            health_diff as f64,
            food_ownership_difference as f64,
            square_ownership_difference as f64,
            target_distance as f64,
            // done
            distance_to_other as f64,
        ] */
        let you_snake = board.snakes.iter().find(|x| x.id == you).unwrap();
        let length_difference = you_snake.body.len() as f64
            - board
                .snakes
                .iter()
                .map(|x| {
                    if x.alive && x.id != you {
                        x.body.len()
                    } else {
                        0
                    }
                })
                .sum::<usize>() as f64;
        let center = Coordinate::new(board.width / 2, board.height / 2);
        let distance_to_center = manhattan_dist(you_snake.body[0], center)
            - board
                .snakes
                .iter()
                .map(|x| {
                    if x.alive && x.id != you {
                        manhattan_dist(x.body[0], center)
                    } else {
                        0
                    }
                })
                .sum::<i32>();
        let health_diff =
            you_snake.health as f64 - board.snakes.iter().map(|x| x.health as f64).sum::<f64>();
        let mut distance_to_other = board
            .snakes
            .iter()
            .map(|x| {
                if x.alive && x.id != you {
                    manhattan_dist(x.body[0], you_snake.body[0]) as f64
                } else {
                    0f64
                }
            })
            .sum::<f64>()
            / (board.num_alive()) as f64;
        if board
            .snakes
            .iter()
            .any(|x| x.alive && x.id != you && x.body.len() > you_snake.body.len())
        {
            distance_to_other *= -1.0;
        }
        Vector::<4>::from_array([
            length_difference,
            distance_to_center as f64,
            health_diff,
            distance_to_other,
        ])
    }

    fn get_move(
        board: &game::board::Board,
        you: game::board::SnakeID,
        weights: vecmath::Vector<4>,
    ) -> game::ruleset::Move {
        let mut possible_moves = vec![
            (Direction::Up, Coordinate::new(0, 1), 0.0),
            (Direction::Down, Coordinate::new(0, -1), 0.0),
            (Direction::Left, Coordinate::new(-1, 0), 0.0),
            (Direction::Right, Coordinate::new(1, 0), 0.0),
        ];
        let you_snake = board.snakes.iter().find(|x| x.id == you).unwrap();
        for x in &mut possible_moves {
            x.1 = x.1 + you_snake.body[0];
        }
        let mut new_possible = vec![];
        for x in possible_moves {
            if !board
                .snakes
                .iter()
                .any(|y| y.alive && y.body[0..y.body.len() - 1].contains(&x.1))
                && !(x.1.x < 0 || x.1.x >= board.width || x.1.y < 0 || x.1.y >= board.height)
            {
                new_possible.push(x);
            }
        }

        for x in &mut new_possible {
            let mut new_board = board.clone();
            let you_snake = new_board.snakes.iter_mut().find(|x| x.id == you).unwrap();
            you_snake.body.insert(0, you_snake.body[0] + x.1);
            for x in &mut new_board.snakes {
                if x.alive {
                    x.body.pop();
                    x.health -= 1;
                }
            }
            x.2 = Self::eval(&new_board, you).dot(weights);
        }
        if new_possible.len() == 0 {
            return Move {
                direction: Direction::Up,
                id: you,
            };
        }
        return Move {
            direction: new_possible
                .iter()
                .max_by(|x, y| x.2.total_cmp(&y.2))
                .unwrap()
                .0,
            id: you,
        };
    }
}

fn manhattan_dist(c1: Coordinate, c2: Coordinate) -> i32 {
    (c1.x - c2.x).abs() + (c1.y - c2.y).abs()
}
