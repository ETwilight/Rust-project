use crate::server::host::client_addr;
use crate::utils;
use crate::server::command_processor::server_send_room;

use tokio::sync::mpsc::Receiver;
use crate::data::{Room};
use crate::game_info::Player;

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
        room.players[id] = player_info.clone();
        clients[id] = player_addr;
        print!("{}\n", player_info.clone().user_info.clone().username.clone());
        for i in 0..6{
            if room.players[i].id == 7 || clients[i] == "Howdy".to_string() {continue;}
            let res = server_send_room(&clients[i], utils::struct_to_string(&room).0, i).await;
            if res.is_err() {continue;}
        }
    }
    (room, clients)
}