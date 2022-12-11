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
pub enum AliveType {
    Alive,
    Dead,
    Wound,
}

impl Default for AliveType{
    fn default() -> Self{
        AliveType::Alive
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
pub struct WitchState {
    pub is_poison_used: bool,
    pub is_antidote_used: bool,
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
    EndTurn, //The turn after game ends
}

impl Default for TurnType{
    fn default() -> Self{
        TurnType::StartTurn
    }
}
impl TurnType {
    pub fn next(&self) -> Self {
        match self {
            TurnType::StartTurn => TurnType::WerewolfTurn,
            TurnType::WerewolfTurn => TurnType::WitchTurn,
            TurnType::WitchTurn => TurnType::ProphetTurn,
            TurnType::ProphetTurn => TurnType::SpeakTurn,
            TurnType::SpeakTurn => TurnType::VoteTurn,
            TurnType::VoteTurn => TurnType::WerewolfTurn,
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
pub struct KillVoteState {
    pub votes: Vec<VoteResult>,
    pub count: usize,
    pub vote_result: (usize, usize),
}

#[derive(Debug, Clone, FromForm, Serialize, Deserialize, Default)]
#[serde(crate = "rocket::serde")]
pub struct VoteState {
    pub votes: Vec<VoteResult>,
    pub count: usize,
    pub vote_result: (usize, usize), //前者是id，后者是票数
}

#[derive(Debug, Clone, FromForm, Serialize, Deserialize, Default)]
#[serde(crate = "rocket::serde")]
pub struct GameState {
    pub turn: TurnType,
    pub win_type: WinType,
    pub speak_id: usize, //The one speaking now
    pub vote_state: VoteState,
    pub kill_vote_state: KillVoteState,
    pub witch_state: WitchState,
    pub reveal_result: RevealResult,
}

#[derive(Debug, Clone, FromForm, Serialize, Deserialize, Default)]
#[serde(crate = "rocket::serde")]
pub struct PlayerState {
    pub is_alive: AliveType,
    pub is_turn: bool,
    pub is_muted: bool,
    pub is_speaking: bool,
}