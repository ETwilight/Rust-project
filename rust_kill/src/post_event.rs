use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, FromFormField, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub enum GameEventType {
    Kill,
    Poison,
    Antidote,
    Reveal,
    Vote,
}

#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GameEvent {
    pub event_type: GameEventType,
    pub voter_id: usize, //The one who actively do something to others, like wolf, witch, prophet
    pub target_id: usize,   //The one who passively be done something
}

#[derive(Debug, Clone, FromForm, Serialize, Deserialize, Default)]
#[serde(crate = "rocket::serde")]
pub struct UserConnectEvent {
    pub username: String,
    pub serverip: String,
    pub is_host: bool,
}


#[derive(Debug, Clone, FromForm, Serialize, Deserialize, Default)]
#[serde(crate = "rocket::serde")]
pub struct MessageEvent {
    pub room_name: String,
    pub username: String,
    pub message:String,
}