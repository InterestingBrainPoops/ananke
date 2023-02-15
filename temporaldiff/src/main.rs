use std::f64::consts::E;

use eval::{evaluation::AreaControlEval, Eval};
use game::{
    ruleset::{Move, Ruleset},
    rulesets::standard::Standard,
};
use vecmath::Vector;
struct Stats {
    game_length: Vec<f64>,
    game_winrate: Vec<f64>,
}
fn TDL<const N: usize, E: Eval<N>, R: Ruleset>(
    weights: Vector<N>,
    mut ruleset: R,
    num_episodes: usize,
    offset_players: usize,
) -> (Vector<N>, Stats) {
    let first_weights = weights.clone();
    // step size
    let alpha = 0.001;
    // discount rate
    let gamma = 0.001;
    // trace decay rate
    let lambda = 0.1;
    let mut old_weights = weights.clone();
    let mut weights = weights.clone();
    let you_id = 0;
    let mut stats = Stats {
        game_length: vec![],
        game_winrate: vec![],
    };
    for episode_num in 0..num_episodes {
        let mut z = Vector::<N>::new(0.0);
        let mut v_old = 0.0;
        let mut state = ruleset.initialize_board();
        let mut x = E::eval(&state, you_id);
        while (!state.game_over()) {
            // get moves from each snake, with your snake being the one with the latest weights
            let moves = state
                .snakes
                .iter()
                .map(|x| {
                    if x.id != you_id {
                        E::get_move(&state, x.id, first_weights)
                    } else {
                        E::get_move(&state, x.id, weights)
                    }
                })
                .collect::<Vec<Move>>();

            state = ruleset.step_board(moves, state).unwrap();
            // find the new feature vector
            let x_prime = (E::eval(&state, you_id));
            // find the latest
            let v = sigmoid(weights.dot(x));

            let v_prime = sigmoid(weights.dot(x_prime));
            // println!("v: {}, v' : {}", v, v_prime);
            // println!("board : {:?}", state);
            // 1 is your reward here
            let sigma = 1f64 + alpha * v_prime - v;

            z = z * gamma * lambda + x * (1.0 - (gamma * lambda * alpha) * (z.dot(x)));
            if episode_num % offset_players == 0 {
                old_weights = weights;
            }
            weights = weights + z * alpha * (sigma + v - v_old) - x * alpha * (v - v_old);
            v_old = v_prime;
            x = x_prime;
        }
        {
            if episode_num % 100 == 0 {
                let num_test = 1000;
                let mut num_wins = 0;
                let mut total_length = 0;
                for _ in 0..num_test {
                    let mut length = 1;
                    let mut state = ruleset.initialize_board();
                    while (!state.game_over()) {
                        length += 1;
                        let moves = state
                            .snakes
                            .iter()
                            .map(|x| {
                                if x.id != you_id {
                                    E::get_move(&state, x.id, first_weights)
                                } else {
                                    E::get_move(&state, x.id, weights)
                                }
                            })
                            .collect::<Vec<Move>>();

                        state = ruleset.step_board(moves, state).unwrap();
                    }
                    if let Some(x) = state.snakes.iter().find(|x| x.alive) {
                        if x.id == you_id {
                            num_wins += 1;
                        }
                    }
                    total_length += length;
                }
                stats
                    .game_length
                    .push(total_length as f64 / num_test as f64);
                stats.game_winrate.push(num_wins as f64 / num_test as f64);
            }
        }
    }
    (weights, stats)
}
fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + E.powf(-x))
}
fn main() {
    let inital_weights = Vector::<4>::new(1.0);
    let ruleset = Standard::new(2);
    let out_weights = TDL::<4, AreaControlEval, Standard>(inital_weights, ruleset, 50000, 50);
    println!("{:?}", out_weights.0);
    for x in 0..out_weights.1.game_length.len() {
        println!(
            "{}, {}",
            out_weights.1.game_length[x], out_weights.1.game_winrate[x],
        );
    }
}
