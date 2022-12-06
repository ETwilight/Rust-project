use rocket::serde::{Serialize, Deserialize};


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