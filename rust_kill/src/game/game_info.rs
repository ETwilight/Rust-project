use rocket::serde::{Serialize, Deserialize};
use std::collections::HashMap;


#[derive(Debug, Clone, FromFormField, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub enum RoleType{
    Undecided,
    Civilian,
    Werewolf,
    Witch,
    Prophet,
}




#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Player{
    pub name:String,
    pub ip:String,
    pub role:RoleType,
    pub state: Option<PlayerState>,
    pub id: usize,
}

#[derive(Debug, Clone, FromFormField, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub enum TurnType{
    StartTurn, //Default turn before GameStart
    WerewolfTurn,
    WitchTurn,
    ProphetTurn,
    SpeakTurn,
    VoteTurn,
    LastWordTurn,
    EndTurn, //The turn after game ends
}

impl TurnType {
    pub fn as_str(&self) -> &'static str {
        match self {
            TurnType::StartTurn => "StartTurn",
            TurnType::WerewolfTurn => "WerewolfTurn",
            TurnType::WitchTurn => "WitchTurn",
            TurnType::ProphetTurn => "ProphetTurn",
            TurnType::SpeakTurn => "SpeakTurn",
            TurnType::VoteTurn => "VoteTurn",
            TurnType::LastWordTurn => "LastWordTurn",
            TurnType::EndTurn=> "EndTurn",
        }
    }

    pub fn next(&self) -> Self{
        match self{
            TurnType::StartTurn => TurnType::WerewolfTurn,
            TurnType::WerewolfTurn => TurnType::WitchTurn,
            TurnType::WitchTurn => TurnType::ProphetTurn,
            TurnType::ProphetTurn => TurnType::SpeakTurn,
            TurnType::SpeakTurn => TurnType::VoteTurn,
            TurnType::VoteTurn => TurnType::LastWordTurn,
            TurnType::LastWordTurn => TurnType::EndTurn,
            TurnType::EndTurn => TurnType::StartTurn,
        }
    }
}



#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Room{
    #[field(validate = len(..30))]
    pub room_name:String,
    pub players: Vec<Player>, 
    pub game_state: GameState,
    //pub Listmessage
}

#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ClientInfo {
    pub room: Room,
    pub ts: TurnType,
    pub idx: usize,
}



#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GameState {
    pub turn: TurnType,
    pub vote_map: HashMap<usize, i32>,
}


#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct PlayerState {
    pub is_turn: bool,
    pub is_muted: bool,
    pub is_speaking: bool,
    pub is_alive: bool,
}
