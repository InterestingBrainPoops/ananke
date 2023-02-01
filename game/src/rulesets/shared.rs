use crate::{
    board::{Board, Coordinate},
    ruleset::{Direction, Move},
};

pub fn apply_moves(mut board: Board, moves: &Vec<Move>) -> Board {
    for mv in moves {
        let diff = match mv.direction {
            Direction::Up => Coordinate { x: 0, y: 1 },
            Direction::Down => Coordinate { x: 0, y: -1 },
            Direction::Left => Coordinate { x: -1, y: 0 },
            Direction::Right => Coordinate { x: 1, y: 0 },
        };
        let current_snake = &mut board.snakes[mv.id];
        current_snake.body.insert(0, current_snake.body[0]);
        current_snake.body[0] += diff;
        current_snake.body.pop();
    }
    board
}
pub fn apply_starvation(mut board: Board, starve_amount: i32) -> Board {
    for snake in &mut board.snakes {
        snake.health -= starve_amount;
    }
    board
}
pub fn feed_snakes(mut board: Board) -> Board {
    let mut eaten_foods = vec![];
    for snake in &mut board.snakes {
        if snake.alive && board.food.contains(&snake.body[0]) {
            eaten_foods.push(snake.body[0]);
            snake.health = 100;
            snake.body.push(*snake.body.last().unwrap());
        }
    }

    eaten_foods.sort();
    eaten_foods.dedup();
    for food in eaten_foods {
        board
            .food
            .remove(board.food.iter().position(|x| *x == food).unwrap());
    }
    board
}
pub fn eliminate_snakes(mut board: Board) -> Board {
    let mut eliminations = vec![];

    for snake in &board.snakes {
        if !snake.alive {
            continue;
        }

        let length = snake.body.len();
        let head = snake.body[0];

        if snake.health <= 0 {
            eliminations.push(snake.id);
            continue;
        }

        if head.x < 0 || head.y < 0 || head.y >= board.height || head.x >= board.width {
            eliminations.push(snake.id);
            continue;
        }

        if board
            .snakes
            .iter()
            .any(|x| x.alive && x.body[1..].contains(&head))
        {
            eliminations.push(snake.id);
            continue;
        }

        if board
            .snakes
            .iter()
            .any(|x| x.alive && x.body[0] == head && x.body.len() >= length)
        {
            eliminations.push(snake.id);
            continue;
        }
    }
    for x in eliminations {
        board.snakes[x].alive = false;
    }
    board
}
