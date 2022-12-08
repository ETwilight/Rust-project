use std::{vec, collections::HashMap};

use rocket::serde::{Serialize, Deserialize};


use super::{game_info::{Room, RoleType, Player, TurnType}, utils::send_message};

use rand::prelude::*;


pub fn update(room:&mut Room){
    match room.game_state.turn{
        TurnType::StartTurn => {
            assign_role(room);
        },
        TurnType::WerewolfTurn => {
            
        },
        TurnType::WitchTurn => todo!(),
        TurnType::ProphetTurn => todo!(),
        TurnType::SpeakTurn => todo!(),
        TurnType::VoteTurn => todo!(),
        TurnType::LastWordTurn => todo!(),
        TurnType::EndTurn => todo!(),
    }
    room.game_state.turn = room.game_state.turn.next();
}


pub fn assign_role(room:&mut Room){
    let mut vec = vec![RoleType::Werewolf, RoleType::Werewolf, RoleType::Witch, RoleType::Prophet, RoleType::Civilian, RoleType::Civilian];
    // Create a random number generator
    let mut rng = rand::thread_rng();
    // Shuffle the vector
    vec.shuffle(&mut rng);
    for i in 1..6{
        room.players[i].role = vec[i].clone();
    }
}

pub fn update_turn(room:&mut Room, turn:TurnType){
    
}
pub fn vote_id(id: usize, room:&mut Room){ 
    //Insert a {id, 0} if not exist, add the value by one if exist
    room.game_state.vote_map.entry(id)
    .and_modify(|count| *count += 1)
    .or_insert(0);
}


pub fn vote_player(player:Player, room:&mut Room){ 
    vote_id(player.id, room)
}

pub fn antidote(player: Player, room:&mut Room){
    
}

pub fn position(player: Player, room:&mut Room){

}

pub fn reveal(player: Player, room:&mut Room){

//send_message(queue, name, " is a pointer")
}