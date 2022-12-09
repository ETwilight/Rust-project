use std::collections::HashMap;

use crate::server::host::client_addr;

use tokio::sync::mpsc::Receiver;
#[path="../game.rs"]
mod game;
#[path="../game/game_info.rs"]
mod game_info;
use crate::server::client_manager::game_info::{Room, Player, GameState, TurnType, RevealResult};

pub async fn receive(rx: &mut Receiver<String>) -> (Room, Vec<String>){
    let mut room:Room = Default::default();
    for i in 0..6{
        room.players.push(Player{
            name: "Howdy".to_string(),
            ip: "127.0.0.1".to_string(),
            role: game_info::RoleType::Undecided,
            state: Default::default(),
            id: 7,
        });
    }
    let mut clients = Vec::<String>::new();
    loop{
        let rec = rx.recv().await;
        if rec.is_none() {break;}
        let recv = rec.unwrap();
        let player_info: Player = serde_json::from_str(&recv).expect("json deserialize failed");
        let id = player_info.id;
        //player_info.
        print!("Player Info: {}\n", player_info.name);
        let player_addr = client_addr(player_info.ip.clone(), id);
        room.players[id] = player_info;
        clients.push(player_addr);
    }
    (room, clients)
}