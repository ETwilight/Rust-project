use rocket::{serde::{Deserialize, Serialize}, form::FromForm};
use std::collections::HashMap;
pub use crate::data::Room;
use crate::data::UserInfo;

#[derive(Debug, Clone, FromFormField, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub enum RoleType {
    Undecided,
    Civilian,
    Werewolf,
    Witch,
    Prophet,
}
impl Default for RoleType{
    fn default() -> Self{
        RoleType::Undecided
    }
}

#[derive(Debug, Clone, FromFormField, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub enum WinType {
    Undecided,
    WerewolfWin,
    CivilianWin,
    Draw, //平局
}

impl Default for WinType{
    fn default() -> Self{
        WinType::Undecided
    }
}

#[derive(Debug, Clone, FromForm, Serialize, Deserialize, Default)]
#[serde(crate = "rocket::serde")]
pub struct VoteResult {
    pub is_vote: bool,
    pub voter_id: usize, 
    pub target_id: usize, 
}

#[derive(Debug, Clone, FromForm, Serialize, Deserialize, Default)]
#[serde(crate = "rocket::serde")]
pub struct RevealResult {
    pub id: usize,
    pub is_good: bool,
}

#[derive(Debug, Clone, FromForm, Serialize, Deserialize, Default)]
#[serde(crate = "rocket::serde")]
pub struct Player {
    pub user_info: UserInfo,
    pub ip: String,
    pub role: RoleType,
    pub state: PlayerState,
    pub id: usize,
}

#[derive(Debug, Clone, FromFormField, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub enum TurnType {
    StartTurn, //Default turn before GameStart
    WerewolfTurn,
    WitchTurn,
    ProphetTurn,
    SpeakTurn,
    VoteTurn,
    LastWordTurn,
    EndTurn, //The turn after game ends
}

impl Default for TurnType{
    fn default() -> Self{
        TurnType::StartTurn
    }
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
            TurnType::EndTurn => "EndTurn",
        }
    }

    pub fn next(&self) -> Self {
        match self {
            TurnType::StartTurn => TurnType::WerewolfTurn,
            TurnType::WerewolfTurn => TurnType::WitchTurn,
            TurnType::WitchTurn => TurnType::ProphetTurn,
            TurnType::ProphetTurn => TurnType::SpeakTurn,
            TurnType::SpeakTurn => TurnType::VoteTurn,
            TurnType::VoteTurn => TurnType::LastWordTurn,
            TurnType::LastWordTurn => TurnType::WerewolfTurn,
            TurnType::EndTurn => TurnType::StartTurn,
        }
    }
}


#[derive(Debug, Clone, FromForm, Serialize, Deserialize, Default)]
#[serde(crate = "rocket::serde")]
pub struct ClientInfo {
    pub room_name: String,
    pub username: String,
    pub client_addr: String, 
    pub idx: usize,
}

#[derive(Debug, Clone, FromForm, Serialize, Deserialize, Default)]
#[serde(crate = "rocket::serde")]
pub struct GameState {
    pub turn: TurnType,
    pub win_type: WinType,
    pub votes: Vec<VoteResult>,
    pub vote_result: (usize, usize), //前者是id，后者是票数
    pub kill_votes: Vec<VoteResult>,
    pub kill_vote_result: (usize, usize),
    pub reveal_result: RevealResult,
}

#[derive(Debug, Clone, FromForm, Serialize, Deserialize, Default)]
#[serde(crate = "rocket::serde")]
pub struct PlayerState {
    pub is_turn: bool,
    pub is_muted: bool,
    pub is_speaking: bool,
    pub is_alive: bool,
}