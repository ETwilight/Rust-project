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
pub struct Room{
    #[field(validate = len(..30))]
    pub room_name:String,
    pub players: HashMap<i32, Player>
}


#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Player{
    pub name:String,
    pub ip:String,
    pub role:RoleType,
}