#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
use rocket::form::Form;
use rocket::response::status::BadRequest;

use gomoku_lib::{decide, utils::is_game_end, Board, Move, Player};

#[get("/")]
fn index() -> &'static str {
  "Gomoku server"
}

#[derive(FromForm)]
struct EvalData<'r> {
  player: &'r str,
  fen: &'r str,
}

#[post("/eval", data = "<input>")]
fn eval(input: Form<EvalData>) -> Result<String, BadRequest<String>> {
  let EvalData { player, fen } = input.into_inner();

  let player = match Player::from_string(player) {
    Ok(player) => player,
    Err(_) => return Err(BadRequest(Some("Invalid player".into()))),
  };

  let fen = fen.replace("/", "\n");

  let mut board = match Board::from_string(&fen) {
    Ok(board) => board,
    Err(err) => return Err(BadRequest(Some(format!("{}", err)))),
  };

  let (Move { tile, .. }, ..) = match decide(&mut board, player, 2000, 8) {
    Ok(fen) => fen,
    Err(err) => return Err(BadRequest(Some(format!("{}", err)))),
  };

  println!("{}", board);

  Ok(format!("{:?}", tile))
}

#[derive(FromForm)]
struct CheckData<'r> {
  fen: &'r str,
}

#[post("/check", data = "<input>")]
fn check(input: Form<CheckData>) -> Result<String, BadRequest<String>> {
  let CheckData { fen } = input.into_inner();

  let fen = fen.replace("/", "\n");
  let board = match Board::from_string(&fen) {
    Ok(board) => board,
    Err(err) => return Err(BadRequest(Some(format!("{}", err)))),
  };

  let end = is_game_end(&board, Player::X) || is_game_end(&board, Player::O);

  Ok(format!("{:?}", end))
}

#[launch]
fn rocket() -> _ {
  rocket::build().mount("/", routes![check, eval, index])
}
