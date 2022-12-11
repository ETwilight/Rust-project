use crate::game_info::{GameState, Player};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, FromFormField, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub enum VisibleType {
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

impl Default for VisibleType{
    fn default() -> Self {
        VisibleType::All
    }
}
#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Message {
    #[field(validate = len(..20))]
    pub username: String, //Maximum Length is 20 for a username
    pub message: String,
    pub visible_type: VisibleType,
}

#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserInfo {
    pub username: String,
    pub serverip: String,
}

impl Default for UserInfo {
    fn default() -> Self {
        UserInfo {
            username: "Howdy".to_string(),
            serverip: "".to_string(),
        }
    }
}

#[derive(Debug, Clone, FromForm, Serialize, Deserialize, Default)]
#[serde(crate = "rocket::serde")]
pub struct Room {
    #[field(validate = len(..30))]
    pub room_name: String,
    pub players: Vec<Player>,
    pub messages: Vec<Message>,
    pub game_state: GameState,
    //pub Listmessage
}
