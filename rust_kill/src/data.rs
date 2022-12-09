use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, FromFormField, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub enum GameEventType{
    Kill,
    Poison,
    Antidote,
    Reveal,
    Vote,
}

#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GameEvent {
    pub event_type : GameEventType,
    pub attacker: String, //The one who actively do something to others, like wolf, witch, prophet
    pub target: String, //The one who passively be done something
}

#[derive(Debug, Clone, FromFormField, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub enum VisibleType{
    #[field(value = "None")]
    None,
    #[field(value = "All")]
    All,
    #[field(value = "Werewolf")]
    Werewolf,
    #[field(value = "Witch")]
    Witch,
    #[field(value = "Prophet")]
    Prophet,
}


#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Message{
    #[field(validate = len(..30))]
    pub room:String, //Maximum Length is 30 for a roomName
    #[field(validate = len(..20))]
    pub username:String, //Maximum Length is 20 for a username
    pub message:String,
    pub visible_type :VisibleType
}

#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserInfo {
    pub username: String,
    pub serverip: String,
}