use rocket::serde::{Serialize, Deserialize};
use std::collections::HashMap;



#[derive(Debug, Clone, FromFormField, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub enum RoleType{
    Civilian,
    Wolf,
    Witch,
    Prophet,
    Undecided,
}



#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Player{
    pub name:String,
    pub ip:String,
    pub role:RoleType,
}





#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Room{
    #[field(validate = len(..30))]
    pub room_name:String,
    pub players: HashMap<i32, Player>, 
    pub turn: Turn,
    //pub Listmessage
}

#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ClientInfo {
    pub room: Room,
    pub ts: TurnType,
    pub idx: usize,
}


#[derive(Debug, Clone, FromFormField, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub enum TurnType{
    StartTurn, //Default turn before GameStart
    WolfTurn,
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
            TurnType::WolfTurn => "WolfTurn",
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
            TurnType::StartTurn => TurnType::WolfTurn,
            TurnType::WolfTurn => TurnType::WitchTurn,
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
pub struct Turn{
    pub turn_state: TurnType,
}


