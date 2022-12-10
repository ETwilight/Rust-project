use std::collections::HashMap;

use crate::server::host::client_addr;
use crate::utils;
use crate::server::command_processor::server_send_room;

use tokio::net::TcpStream;
use tokio::sync::mpsc::Receiver;
#[path="../game.rs"]
mod game;
#[path="../game/game_info.rs"]
mod game_info;
use crate::server::client_manager::game_info::{GameState, TurnType, RevealResult};
use crate::data::{Room};
use crate::game_info::{Player, RoleType};

pub async fn receive(rx: &mut Receiver<String>) -> (Room, Vec<String>){
    let mut room:Room = Default::default();
    let mut clients = Vec::<String>::new();
    for _ in 0..6{
        room.players.push(Player{
            user_info: Default::default(),
            ip: "127.0.0.1".to_string(),
            role: Default::default(),
            state: Default::default(),
            id: 7,
        });
        clients.push("Howdy".to_string());
    }
    loop{
        let rec = rx.recv().await;
        if rec.is_none() {break;}
        let recv = rec.unwrap();
        let player_info: Player = serde_json::from_str(&recv).expect("json deserialize failed");
        let id = player_info.id;
        let player_addr = client_addr(player_info.ip.clone(), id);
        room.players[id] = player_info;
        clients[id] = player_addr;
        for i in 0..6{
            if room.players[i].id == 7 || clients[i] == "Howdy".to_string() {continue;}
            let res = server_send_room(&clients[i], utils::struct_to_string(&room).1, i).await;
            if res.is_err() {continue;}
        }
    }
    (room, clients)
}