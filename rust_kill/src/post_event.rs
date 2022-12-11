use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, FromFormField, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub enum VoteEventType {
    Kill,
    WerewolfGiveUp,
    Poison,
    Antidote,
    WitchGiveUp,
    Reveal,
    Vote,
}

#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct VoteEvent {
    pub event_type: VoteEventType,
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
    pub username: String,
    pub message:String,
}


#[derive(Debug, Clone, FromForm, Serialize, Deserialize, Default)]
#[serde(crate = "rocket::serde")]
pub struct EndSpeakEvent {
    pub id: usize,
}
