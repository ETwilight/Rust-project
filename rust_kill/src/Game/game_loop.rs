use std::{collections::HashMap, vec};

use rocket::serde::{Deserialize, Serialize};

use crate::data::Room;
use crate::game_info::{Player, RevealResult, RoleType, TurnType, VoteResult, WinType};
use crate::post_event::GameEvent;

use rand::prelude::*;

pub fn update(event: GameEvent, room: &mut Room) {
    match room.game_state.turn {
        TurnType::StartTurn => {
            assign_role(room);
        }
        TurnType::WerewolfTurn => {}
        TurnType::WitchTurn => todo!(),
        TurnType::ProphetTurn => todo!(),
        TurnType::SpeakTurn => todo!(),
        TurnType::VoteTurn => todo!(),
        TurnType::LastWordTurn => todo!(),
        TurnType::EndTurn => todo!(),
    }
    room.game_state.turn = room.game_state.turn.next();
}

pub fn assign_role(room: &mut Room) {
    let mut vec = vec![
        RoleType::Werewolf,
        RoleType::Werewolf,
        RoleType::Witch,
        RoleType::Prophet,
        RoleType::Civilian,
        RoleType::Civilian,
    ];
    // Create a random number generator
    let mut rng = rand::thread_rng();
    // Shuffle the vector
    vec.shuffle(&mut rng);
    for i in 1..6 {
        room.players[i].role = vec[i].clone();
    }
}

pub fn detect_vote(room: &mut Room, number: usize) -> bool {
    let mut count: usize = 0;
    let mut vote_map: HashMap<usize, usize> = HashMap::new();
    for vote in room.game_state.votes.iter() {
        if vote.is_vote {
            count += 1;
            if vote.target_id == 0{
                continue;
            }
            vote_map
                .entry(vote.voter_id)
                .and_modify(|count| *count += 1)
                .or_insert(0);
        }
    }
    if (count >= number) {
        let key_with_max_value = vote_map.iter().max_by_key(|entry | entry.1).unwrap();
        room.game_state.vote_result = (*key_with_max_value.0, *key_with_max_value.1);
        return true;
    }
    return false;
}


pub fn detect_kill_vote(room: &mut Room, number: usize) -> bool {
    let mut count: usize = 0;
    let mut kill_vote_map: HashMap<usize, usize> = HashMap::new();
    for vote in room.game_state.kill_votes.iter() {
        if vote.is_vote {
            count += 1;
            if vote.target_id == 0{
                continue;
            }
            kill_vote_map
                .entry(vote.voter_id)
                .and_modify(|count| *count += 1)
                .or_insert(0);
        }
    }
    if (count >= number) {
        let key_with_max_value = kill_vote_map.iter().max_by_key(|entry | entry.1).unwrap();
        room.game_state.kill_vote_result = (*key_with_max_value.0, *key_with_max_value.1);
        return true;
    }
    return false;
}

pub fn vote_id(event: GameEvent, room: &mut Room) {
    let vote = VoteResult {
        is_vote: true,
        voter_id: event.voter_id,
        target_id: event.target_id,
    };
    room.game_state.votes.push(vote);
}

pub fn kill_vote_id(event: GameEvent, room: &mut Room) {
    let vote = VoteResult {
        is_vote: true,
        voter_id: event.voter_id,
        target_id: event.target_id,
    };
    room.game_state.kill_votes.push(vote);
}

//Must Ensure that player.is_alive is false
pub fn antidote(player: &mut Player, room: &mut Room) {
    if player.state.is_alive {
        print!("Warning: antidote() player's is_alive should be false");
        return;
    }
    player.state.is_alive = true;
}

pub fn poison(player: &mut Player, room: &mut Room) {
    if !player.state.is_alive {
        print!("Warning: antidote() player's is_alive should be true");
        return;
    }
    player.state.is_alive = false;
}

pub fn reveal(player: &Player, room: &mut Room) {
    room.game_state.reveal_result.id = player.id;
    match player.role {
        RoleType::Undecided => (),
        RoleType::Werewolf => {
            room.game_state.reveal_result.is_good = false;
        }
        _ => {
            room.game_state.reveal_result.is_good = true;
        }
    }
}

pub fn detect_win(room: &mut Room) {
    let mut count_werewolf = 0;
    let mut count_good = 0;
    for player in room.players.iter() {
        if player.state.is_alive {
            match player.role {
                RoleType::Undecided => (),
                RoleType::Civilian => {
                    count_good += 1;
                }
                RoleType::Werewolf => {
                    count_werewolf += 1;
                }
                RoleType::Witch => {
                    count_good += 1;
                }
                RoleType::Prophet => {
                    count_good += 1;
                }
            }
        }
    }
    if count_werewolf == 0 && count_good == 0 {
        room.game_state.win_type = WinType::Draw; //平局
    } else if count_good == 0 || (count_good < count_werewolf) {
        room.game_state.win_type = WinType::WerewolfWin;
    } else if count_werewolf == 0 {
        room.game_state.win_type = WinType::CivilianWin;
    } else {
        room.game_state.win_type = WinType::Undecided;
    }
}
