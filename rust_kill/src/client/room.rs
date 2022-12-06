use crate::game::{Room, game_loop::Player};
#[path="../utils.rs"]
mod utils;
#[path="../game/mod.rs"]
mod game;

// tell the javascript that it should connect to an empty room with name given in the room info
pub async fn connectRoom(name: String) {
    print!("Successfully connect to room: {} \n", name);
}

pub async fn addPlayer(player: Player, idx: i32) {

}