mod types;

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use eval::{self, evaluation::AreaControlEval, Eval};
use game::rulesets::standard::Standard;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::net::SocketAddr;
use types::Request;
use vecmath::Vector;
#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/start", post(start))
        .route("/move", post(get_move))
        .route("/end", post(end));
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    r##"{"apiversion": "1","author": "BrokenKeyboard","color": "#F8DE7F","head": "shades","tail": "block-bum","version": "0.1.0"}"##
}

async fn start() {}

#[derive(Deserialize)]
struct Resp {}
async fn get_move(Json(payload): Json<Request>) -> Json<Value> {
    let mut ruleset = Standard::new(2);
    let (you, board) = payload.into_game_board();
    let weights = Vector::<4>::from_array([
        1.0193606502595651,
        0.9306437173176719,
        -21.026319381742347,
        1.200013697796948,
    ]);
    let to_move = AreaControlEval::get_move(&board, you, weights);
    let out_move = match to_move.direction {
        game::ruleset::Direction::Up => "up",
        game::ruleset::Direction::Down => "down",
        game::ruleset::Direction::Left => "left",
        game::ruleset::Direction::Right => "right",
    };

    Json(json!({
        "move": out_move,
        "shout": ""
    }))
}

async fn end() {}
