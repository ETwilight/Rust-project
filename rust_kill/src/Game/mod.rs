use rocket::serde::{Serialize, Deserialize};
use std::collections::HashMap;

use self::{game_loop::Player, turn::Turn};

pub mod turn;
pub mod game_loop;



#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Room{
    #[field(validate = len(..30))]
    pub room_name:String,
    pub players: Vec<Player>, 
    pub turn: Turn,
    //pub Listmessage
}

#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ClientInfo {
    pub room: Room,
    pub idx: usize,
}
