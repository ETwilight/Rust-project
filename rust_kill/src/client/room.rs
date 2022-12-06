#[path="../utils.rs"]
mod utils;

#[path="../game/game_info.rs"]
mod game_info;

use rocket::tokio::sync::broadcast::Sender;
use crate::{client::room::game_info::Player, Message};

// tell the javascript that it should connect to an empty room with name given in the room info
pub async fn connectRoom(name: String, sender: Sender<Message>) {
    print!("Successfully connect to room: {} \n", name);
}

pub async fn addPlayer(player: Player, idx: i32) {
    todo!()
}