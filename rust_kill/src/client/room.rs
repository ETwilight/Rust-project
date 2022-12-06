#[path="../utils.rs"]
mod utils;

#[path="../game/game_info.rs"]
mod game_info;

#[path="../game.rs"]
mod game;

use crate::client::room::game_info::Player;

// tell the javascript that it should connect to an empty room with name given in the room info
pub async fn connectRoom(name: String) {
    print!("Successfully connect to room: {} \n", name);

}

pub async fn addPlayer(player: Player, idx: i32) {
    todo!()
}