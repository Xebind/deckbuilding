use axum::routing::{get, post};
use serde::{Serialize};
use axum::response::{Json, Redirect};
use axum::extract::{State, Path};
use std::sync::{Arc, Mutex};


mod player;
mod card;
mod market;
mod game;

use crate::game::Game;

#[tokio::main]
async fn main() {
    let mut game = Arc::new(Mutex::new(Game::new()));

    let app = axum::Router::new()
        .route("/", get(index))
        .route("/gameLog", get(game_log))
        .route("/buy/:index", get(buy_card))
        .route("/reset", get(reset))
        .with_state(game);


    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
async fn index() ->String {
    "Hello, here we will show our Deckbuilding Game instructions".to_string()
}
async fn random_game_log(State(game): State<Arc<Mutex<Game>>>) -> String {
    game.lock().unwrap().run_random();
    game.lock().unwrap().logs.clone()
}

async fn game_log(State(game): State<Arc<Mutex<Game>>>) -> String {
    game.lock().unwrap().run_game();
    game.lock().unwrap().logs.clone()
}

async fn buy_card(Path(index): Path<usize>, State(game): State<Arc<Mutex<Game>>>) -> Redirect {
    game.lock().unwrap().real_player_api_turn(index);
    return Redirect::to("/gameLog");

}

async fn reset(State(game): State<Arc<Mutex<Game>>>)-> Redirect{
    game.lock().unwrap().reset();
    return Redirect::to("/gameLog");
}