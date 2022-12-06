use std::vec;

use rocket::serde::{Serialize, Deserialize};


use super::game_info::{Room, RoleType, Player};

use rand::prelude::*;


pub fn update(room:&mut Room){
    match room.game_state.turn.turn_state{
        super::game_info::TurnType::StartTurn => {
            assign_role(&mut room.players);
        },
        super::game_info::TurnType::WolfTurn => {
            
        },
        super::game_info::TurnType::WitchTurn => todo!(),
        super::game_info::TurnType::ProphetTurn => todo!(),
        super::game_info::TurnType::SpeakTurn => todo!(),
        super::game_info::TurnType::VoteTurn => todo!(),
        super::game_info::TurnType::LastWordTurn => todo!(),
        super::game_info::TurnType::EndTurn => todo!(),
    }
    room.game_state.turn.turn_state = room.game_state.turn.turn_state.next();
}

pub fn assign_role(players :&mut Vec<Player>){
    let mut vec = vec![RoleType::Wolf, RoleType::Wolf, RoleType::Witch, RoleType::Prophet, RoleType::Civilian, RoleType::Civilian];
    // Create a random number generator
    let mut rng = rand::thread_rng();
    // Shuffle the vector
    vec.shuffle(&mut rng);
    for i in 1..6{
        players[i].role = vec[i].clone();
    }
}




