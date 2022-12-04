use rocket::serde::{Serialize, Deserialize};
use rocket::form::FromForm;

#[derive(Debug, Clone, FromFormField, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
enum TurnState{
    StartTurn, //Default turn before GameStart
    WolfTurn,
    WitchTurn,
    ProphetTurn,
    SpeakTurn,
    VoteTurn,
    LastWordTurn,
    EndTurn, //The tur after game ends
}

impl TurnState {
    fn as_str(&self) -> &'static str {
        match self {
            TurnState::StartTurn => "StartTurn",
            TurnState::WolfTurn => "WolfTurn",
            TurnState::WitchTurn => "WitchTurn",
            TurnState::ProphetTurn => "ProphetTurn",
            TurnState::SpeakTurn => "SpeakTurn",
            TurnState::VoteTurn => "VoteTurn",
            TurnState::LastWordTurn => "LastWordTurn",
            TurnState::EndTurn=> "EndTurn",
        }
    }
}

#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Turn{
    pub turn_state: TurnState,
}
