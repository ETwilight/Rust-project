use std::{vec, collections::HashMap};

use rocket::serde::{Serialize, Deserialize};


use crate::data::{GameEvent, Room};

use crate::game_info::{RoleType, Player, TurnType, WinType, RevealResult};

use rand::prelude::*;


pub fn update(event:GameEvent, room:&mut Room){
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

pub fn detect_kill_vote(room:&mut Room){

}


pub fn vote_id(id: usize, room:&mut Room){ 
    //Insert a {id, 0} if not exist, add the value by one if exist
    room.game_state.vote_map.entry(id)
    .and_modify(|count| *count += 1)
    .or_insert(0);
}


pub fn vote_player(player:&Player, room:&mut Room){ 
    vote_id(player.id, room)
}

//Must Ensure that player.is_alive is false
pub fn antidote(player: &mut Player, room:&mut Room){
    if player.state.is_alive{
        print!("Warning: antidote() player's is_alive should be false");
        return;
    }
    player.state.is_alive = true;
}

pub fn poison(player: &mut Player, room:&mut Room){
    if !player.state.is_alive{
        print!("Warning: antidote() player's is_alive should be true");
        return;
    }
    player.state.is_alive = false;
}

pub fn reveal(player: &Player, room:&mut Room){
    room.game_state.reveal_result.id = player.id;
    match player.role{
        RoleType::Undecided => (),
        RoleType::Werewolf => {room.game_state.reveal_result.is_good = false;},
        _ => {room.game_state.reveal_result.is_good = true;}
    }
    
}


pub fn detect_win(room:&mut Room){
    let mut count_werewolf = 0;
    let mut count_good = 0;
    for player in room.players.iter(){
        if player.state.is_alive{
            match player.role{
                RoleType::Undecided => (),
                RoleType::Civilian => {count_good +=1;},
                RoleType::Werewolf => {count_werewolf +=1;},
                RoleType::Witch => {count_good +=1;},
                RoleType::Prophet => {count_good +=1;},
            }
        }
    }
    if(count_werewolf ==0 && count_good == 0){
        room.game_state.win_type = WinType::Draw; //平局
    }
    else if(count_good == 0 || (count_good < count_werewolf)){
        room.game_state.win_type = WinType::WerewolfWin;
    }
    else if(count_werewolf ==0){
        room.game_state.win_type = WinType::CivilianWin;
    }
    else{
        room.game_state.win_type = WinType::Undecided;
    }
}