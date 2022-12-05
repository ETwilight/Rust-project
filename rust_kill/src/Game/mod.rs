use rocket::serde::{Serialize, Deserialize};

pub mod turn;
pub mod game_loop;

#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GameInfo {
    pub room: game_loop::Room,
    pub ts: turn::TurnState,
}

#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ClientInfo {
    pub room: game_loop::Room,
    pub ts: turn::TurnState,
    pub idx: usize,
}
