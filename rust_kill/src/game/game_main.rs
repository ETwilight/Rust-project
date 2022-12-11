use std::{collections::HashMap, vec};

use rocket::serde::{Deserialize, Serialize};

use crate::data::{Message, Room, VisibleType};
use crate::game_info::{AliveType, Player, RevealResult, RoleType, TurnType, VoteResult, WinType};
use crate::post_event::{MessageEvent, PostEventType, UserConnectEvent, VoteEvent, VoteEventType};
use crate::utils::{string_to_struct, struct_to_string};
use rand::prelude::*;

pub fn update(room: &mut Room, json_string: &String) {
    print!("\n Update Howdy");
    let value: serde_json::Value = serde_json::from_str(json_string).unwrap();
    match serde_json::from_value(value) {
        Ok(PostEventType::VoteEvent(event)) => {
            update_vote(&event, room);
        }
        Ok(PostEventType::UserConnectEvent(event)) => {}
        Ok(PostEventType::MessageEvent(event)) => {
            print!("-------Add Message------\n");
            add_message(&event, room);
        }
        Ok(PostEventType::EndSpeakEvent(event)) => {
            if(room.game_state.speak_id == get_last_id(room)){
                room.game_state.turn = room.game_state.turn.next();
                return;
            }
            match room.game_state.turn {
                TurnType::SpeakTurn => {
                    room.players[event.id].state.is_speaking = false;
                    room.game_state.speak_id += 1;
                }
                _ => {
                    print!["\n Warning!: Not My Turn \n"];
                }
            };
        }
        _ => print!("my_struct is of unknown type"),
        Err(_) => print!("my_struct is of unknown type"),
    }
}

pub fn game_start(room: &mut Room){
    assign_role(room);
    room.game_state.turn = room.game_state.turn.next();
    return;
}
pub fn update_vote(event: &VoteEvent, room: &mut Room) {
    match room.game_state.turn {
        TurnType::WerewolfTurn => {
            match event.event_type {
                VoteEventType::Kill => {
                    kill_vote(event, room);
                }
                VoteEventType::WerewolfGiveUp => {
                    {
                        room.game_state.kill_vote_state.count += 1;
                    };
                }
                _ => {
                    print!("Warning: VoteEvent in WerewolfTurn should only be VoteEventType::Kill");
                    return;
                }
            };
            if room.game_state.kill_vote_state.count >= count_werewolf(room) {
                calculate_kill_vote_result(room);
                room.game_state.turn = room.game_state.turn.next();
                return;
            };
        }
        TurnType::WitchTurn => {
            match event.event_type {
                VoteEventType::Poison => {
                    poison(event, room);
                }
                VoteEventType::Antidote => {
                    antidote(event, room);
                }
                VoteEventType::WitchGiveUp => {}
                _ => {
                    print!("Warning: VoteEvent in WerewolfTurn should only be VoteEventType::Kill");
                    return;
                }
            };
            room.game_state.turn = room.game_state.turn.next();
            return;
        }
        TurnType::ProphetTurn => {
            if matches!(event.event_type, VoteEventType::Reveal) {
                reveal(event, room);
            }
            wound_to_dead(room);
            if detect_win(room) {
                room.game_state.turn = TurnType::EndTurn;
                print!["\nGame Ends!!!!!!!!!!!!!!! \n"];
                return;
            }
            room.game_state.turn = room.game_state.turn.next();
            return;
        }
        TurnType::VoteTurn => {
            match event.event_type {
                VoteEventType::Vote => {
                    vote(event, room);
                    room.game_state.vote_state.count += 1;
                }
                VoteEventType::VoteGiveUp => {
                    room.game_state.vote_state.count += 1;
                }
                _ => {}
            };
            if room.game_state.vote_state.count >= count_all(room) {
                room.game_state.turn = room.game_state.turn.next();
                return;
            }
        }
        TurnType::EndTurn => {
            print!["\nHowdy Ends!!!!!!!!!!!!!!! \n"];
        }
        _ => {
            print!["\n Warning!: Not My Turn \n"];
        }
    }
}

//Get the alive player with last id
pub fn get_last_id(room: &mut Room) -> usize{
    let mut id:usize = 0;
    for player in room.players.iter() {
        if matches!(player.state.is_alive, AliveType::Alive) {
            id = player.id;
        }
    }
    return id;
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
    for i in 0..5 {
        room.players[i].role = vec[i].clone();
    }
}

pub fn wound_to_dead(room: &mut Room) {
    for player in room.players.iter_mut() {
        if matches!(player.state.is_alive, AliveType::Wound) {
            player.state.is_alive = AliveType::Dead;
        }
    }
}
pub fn count_player(room: &mut Room) -> (usize, usize, usize) {
    let mut count_all: usize = 0;
    let mut count_good: usize = 0;
    let mut count_werewolf: usize = 0;
    for player in room.players.iter() {
        if matches!(player.state.is_alive, AliveType::Alive) {
            count_all += 1;
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
    return (count_all, count_good, count_werewolf);
}

pub fn count_all(room: &mut Room) -> usize {
    return count_player(room).0;
}

pub fn count_good(room: &mut Room) -> usize {
    return count_player(room).1;
}

pub fn count_werewolf(room: &mut Room) -> usize {
    return count_player(room).2;
}

pub fn calculate_vote_resulte(room: &mut Room) {
    let mut vote_map: HashMap<usize, usize> = HashMap::new();
    for vote in room.game_state.vote_state.votes.iter() {
        if vote.is_vote {
            if vote.target_id == 0 {
                continue;
            }
            vote_map
                .entry(vote.voter_id)
                .and_modify(|count| *count += 1)
                .or_insert(0);
        }
    }
    let key_with_max_value = vote_map.iter().max_by_key(|entry| entry.1).unwrap();
    room.game_state.vote_state.vote_result = (*key_with_max_value.0, *key_with_max_value.1);
}

pub fn calculate_kill_vote_result(room: &mut Room) {
    let mut kill_vote_map: HashMap<usize, usize> = HashMap::new();
    for vote in room.game_state.kill_vote_state.votes.iter() {
        if vote.is_vote {
            kill_vote_map
                .entry(vote.voter_id)
                .and_modify(|count| *count += 1)
                .or_insert(0);
        }
    }
    let key_with_max_value = kill_vote_map.iter().max_by_key(|entry| entry.1).unwrap();
    room.game_state.kill_vote_state.vote_result = (*key_with_max_value.0, *key_with_max_value.1);
    room.players[*key_with_max_value.0].state.is_alive = AliveType::Wound;
}

pub fn vote(event: &VoteEvent, room: &mut Room) {
    let vote = VoteResult {
        is_vote: true,
        voter_id: event.voter_id,
        target_id: event.target_id,
    };
    room.game_state.vote_state.votes.push(vote);
    room.game_state.vote_state.count += 1;
}

pub fn kill_vote(event: &VoteEvent, room: &mut Room) {
    let vote = VoteResult {
        is_vote: true,
        voter_id: event.voter_id,
        target_id: event.target_id,
    };
    room.game_state.kill_vote_state.votes.push(vote);
    room.game_state.kill_vote_state.count += 1;
}

//Must Ensure that player.is_alive is false
pub fn antidote(event: &VoteEvent, room: &mut Room) {
    let mut player = &mut room.players[event.target_id.clone()];
    match player.state.is_alive {
        AliveType::Wound => player.state.is_alive = AliveType::Alive,
        _ => {
            print!("Warning: antidote() player's is_alive should be alive or wound");
        }
    }
}

pub fn poison(event: &VoteEvent, room: &mut Room) {
    let mut player = &mut room.players[event.target_id.clone()];
    match player.state.is_alive {
        AliveType::Alive => player.state.is_alive = AliveType::Wound,
        AliveType::Wound => player.state.is_alive = AliveType::Wound,
        _ => {
            print!("Warning: antidote() player's is_alive should not be dead");
        }
    }
}

pub fn reveal(event: &VoteEvent, room: &mut Room) {
    room.game_state.reveal_result.id = event.target_id;
    let player = &room.players[event.target_id.clone()];
    match player.role {
        RoleType::Werewolf => {
            room.game_state.reveal_result.is_good = false;
        }
        _ => {
            room.game_state.reveal_result.is_good = true;
        }
    }
}

pub fn add_message(event: &MessageEvent, room: &mut Room) {
    let player = &(room.players[event.id]);
    let message = Message {
        id: event.id,
        username: player.clone().user_info.username,
        message: event.message.clone(),
        visible_type: Default::default(),
    };
    room.messages.push(message);
}
pub fn detect_win(room: &mut Room) -> bool {
    let mut count_werewolf = count_werewolf(room);
    let mut count_good = count_good(room);

    if count_werewolf == 0 && count_good == 0 {
        room.game_state.win_type = WinType::Draw; //平局
        return true;
    } else if count_good == 0 || (count_good < count_werewolf) {
        room.game_state.win_type = WinType::WerewolfWin;
        return true;
    } else if count_werewolf == 0 {
        room.game_state.win_type = WinType::CivilianWin;
        return true;
    } else {
        room.game_state.win_type = WinType::Undecided;
        return false;
    }
}
