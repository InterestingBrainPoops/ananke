use rand::prelude::*;

use crate::{
    board::{Board, Coordinate, Snake},
    ruleset::{Move, Ruleset},
};

use super::shared;
#[derive(Clone)]
pub struct Standard {
    num_snakes: u32,
    rng: ThreadRng,
}

impl Standard {
    pub fn new(num_snakes: u32) -> Standard {
        if ![2, 4].contains(&num_snakes) {
            panic!(
                "I can't handle snake numbers that arent 2 or 4, given {}",
                num_snakes
            );
        }
        Standard {
            num_snakes,
            rng: thread_rng(),
        }
    }
}

impl Ruleset for Standard {
    fn initialize_board(&mut self) -> Board {
        let mut out = Board {
            width: 11,
            height: 11,
            snakes: vec![],
            food: vec![Coordinate::new(6, 6)],
            hazards: vec![],
        };
        // starting length of 3 for each snake

        if self.num_snakes == 2 {
            let start_x = vec![1, 6, 10];
            let start_y = vec![1, 6, 10];

            // create all possible starting positions
            let mut starting_positions = start_x
                .iter()
                .flat_map(|x| start_y.iter().map(|y| Coordinate::new(*x, *y)))
                .collect::<Vec<Coordinate>>();

            // remove the center
            starting_positions.remove(
                starting_positions
                    .iter()
                    .position(|x| x.x == 6 && x.y == 6)
                    .expect("Couldn't find the center in the removal"),
            );
            // pick two positions
            let places = starting_positions
                .partial_shuffle(&mut self.rng, 2)
                .0
                .iter()
                .cloned()
                .collect::<Vec<Coordinate>>();
            // put a snake on each position

            let offset_x = vec![-1, 1];
            let offset_y = vec![-1, 1];

            let mut offset_positions = offset_x
                .iter()
                .flat_map(|x| offset_y.iter().map(|y| Coordinate::new(*x, *y)))
                .collect::<Vec<Coordinate>>();
            offset_positions.shuffle(&mut self.rng);
            for (id, place) in places.iter().cloned().enumerate() {
                out.snakes.push(Snake {
                    alive: true,
                    id,
                    health: 100,
                    body: (0..4).map(|_| place.clone()).collect::<Vec<Coordinate>>(),
                });
                // put a food within 2 moves of each snake
                let offset = offset_positions.pop().unwrap();
                out.food.push(Coordinate {
                    x: place.x + offset.x,
                    y: place.y + offset.y,
                })
            }
        } else {
            todo!("have yet to do 4 snakes");
        }
        out
    }

    fn step_board(&mut self, moves: Vec<Move>, board: Board) -> Result<Board, &str> {
        /*
        Stages:
            Is Game over?
            Move the snakes
            Apply starvation to each snake
            Feed snakes as needed
            Apply hazard damage as needed
            Process eliminations
        */

        if board.game_over() {
            return Err("Cannot apply moves to a state thats already over");
        }

        // Apply the moves to each snake
        let board = shared::apply_moves(board, &moves);

        // apply starvation to each snake
        let board = shared::apply_starvation(board, 1);

        // feed each snake
        let board = shared::feed_snakes(board);

        // remove eliminations
        let board = shared::eliminate_snakes(board);
        Ok(board)
    }
}
