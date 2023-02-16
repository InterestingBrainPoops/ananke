use game::board::{Board, Coordinate, Snake, SnakeID};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Request {
    pub turn: u64,
    pub board: RBoard,
    pub you: RSnake,
}

#[derive(Deserialize, Debug)]
pub struct RBoard {
    pub height: u32,
    pub width: u32,
    pub food: Vec<Coordinate>,
    pub hazards: Vec<Coordinate>,
    pub snakes: Vec<RSnake>,
}

#[derive(Deserialize, Debug)]
pub struct RSnake {
    pub id: String,
    pub health: i8,
    pub body: Vec<Coordinate>,
}

impl Request {
    pub fn into_game_board(&self) -> (SnakeID, Board) {
        let you_id = self
            .board
            .snakes
            .iter()
            .enumerate()
            .find(|(idx, x)| x.id == self.you.id)
            .unwrap()
            .0;
        let mut out = Board {
            width: self.board.width as i32,
            height: self.board.height as i32,
            snakes: vec![],
            food: self.board.food.clone(),
            hazards: self.board.hazards.clone(),
        };

        for (idx, x) in self.board.snakes.iter().enumerate() {
            out.snakes.push(Snake {
                alive: true,
                id: idx,
                health: x.health as i32,
                body: x.body.clone(),
            });
        }

        (you_id, out)
    }
}
