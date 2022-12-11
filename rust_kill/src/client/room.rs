#[path="../utils.rs"]
mod utils;

#[path="../game/game_info.rs"]
mod game_info;

#[path="../game.rs"]
mod game;

use crate::game_info::ClientInfo;


use rocket::{tokio::sync::broadcast::Sender, serde::json::Json};
use crate::{client::room::game_info::Player, data::{Message, VisibleType}};


// tell the javascript that it should connect to an empty room with name given in the room info
pub async fn connect_room(cinfo: ClientInfo, sender: Sender<Message>) {
    print!("Successfully connect to room: {} \n", cinfo.room_name);
    //game::utils::send_delay_message(sender, "Howdy".to_string(), name, VisibleType::All, 15000).await.unwrap();
}