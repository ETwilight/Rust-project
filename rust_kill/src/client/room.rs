#[path="../utils.rs"]
mod utils;

#[path="../game/game_info.rs"]
mod game_info;

#[path="../game.rs"]
mod game;


use rocket::{tokio::sync::broadcast::Sender, serde::json::Json};
use crate::{client::room::game_info::Player, data::{Message, VisibleType}};


// tell the javascript that it should connect to an empty room with name given in the room info
pub async fn connect_room(name: String, sender: Sender<Message>) {
    print!("Successfully connect to room: {} \n", name);
    //game::utils::send_delay_message(sender, "Howdy".to_string(), name, VisibleType::All, 15000).await.unwrap();
}

pub async fn add_player(player: Player, idx: i32) {
    todo!()
}