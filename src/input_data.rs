use std::io::Read;

use gomoku_lib::Player;

use rocket::data::{self, FromDataSimple};
use rocket::http::{ContentType, Status};
use rocket::{Data, Outcome, Outcome::*, Request};

// enough space for player, board size and board data up to 20x20 board
const LIMIT: u64 = 3 + 3 + 20 * (20 + 1);

#[derive(Debug)]
pub struct InputData {
  player: Player,
  size: u8,
  fen: String,
}

impl FromDataSimple for InputData {
  type Error = String;

  fn from_data(req: &Request, data: Data) -> data::Outcome<Self, String> {
    // Ensure the content type is correct before opening the data.
    let input_ct = ContentType::new("application", "x-input");
    if req.content_type() != Some(&input_ct) {
      return Outcome::Forward(data);
    }

    // Read the data into a String.
    let mut string = String::new();
    if let Err(e) = data.open().take(LIMIT).read_to_string(&mut string) {
      return Failure((Status::InternalServerError, format!("{:?}", e)));
    }

    // Split the string into two pieces at ':'.
    let mut splitted = string.split('/');

    let player = match splitted.next() {
      Some(player) => player,
      None => return Failure((Status::BadRequest, "Missing player".into())),
    };
    let size = match splitted.next() {
      Some(size) => size,
      None => return Failure((Status::BadRequest, "Missing size".into())),
    };
    let fen = splitted.collect::<String>();

    let player = match Player::from_char(player.chars().next().unwrap()) {
      Ok(player) => player,
      Err(_) => return Failure((Status::UnprocessableEntity, "Player".into())),
    };

    let size: u8 = match size.parse() {
      Ok(size) => size,
      Err(_) => return Failure((Status::UnprocessableEntity, "Player".into())),
    };

    let fen = match gomoku_lib::utils::parse_fen_string(&fen) {
      Ok(fen) => fen,
      Err(_) => return Failure((Status::UnprocessableEntity, "Player".into())),
    };

    // Return successfully.
    Success(InputData { player, size, fen })
  }
}
