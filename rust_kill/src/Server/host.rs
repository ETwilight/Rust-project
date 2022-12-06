use std::{collections::HashMap, sync::Arc};

use tokio::{net::{TcpListener, tcp::{OwnedReadHalf, OwnedWriteHalf}}, task::JoinHandle, io::{BufReader, AsyncBufReadExt}, sync::{mpsc::{self, Sender, Receiver}, Mutex}, join};
use serde::{Serialize, Deserialize};

use crate::server::{host::game_info::{Room, Player, GameState, TurnType, ClientInfo}, self};

#[path="../utils.rs"]
mod utils;

#[path="../game/game_info.rs"]
mod game_info;

pub async fn start(server_addr: &str) -> Result<JoinHandle<()>, ()>{
    // the main thread to return    
    let task = tokio::spawn(async move{
        let clientList: Vec<&str> = Vec::new();
        let listener = TcpListener::bind("0.0.0.0".to_string() + ":8080").await.unwrap();
        print!("SERVER STARTS\n");
        // do not consider player log out
        let mut num : usize = 0;
        let mut room = Room{
            room_name: "rustkill".to_string(),
            players: Vec::<Player>::new(),
            game_state: GameState{turn: server::host::game_info::Turn { turn_state: TurnType::StartTurn }},
        };
        let (tx, mut rx) = mpsc::channel::<String>(6);
        let rom = room.clone();
        
        let wait = tokio::spawn(async move {
            loop {
                if num >= 6 {break;}
                let (socket, _) = listener.accept().await.unwrap();
                print!("Client Addr: {}\n", socket.peer_addr().unwrap().ip());
                let (mut reader, mut writer) = socket.into_split();
                let txc = tx.clone();
                let task = tokio::spawn(async move{
                    let rawc = utils::serverResponse(&mut reader, &mut writer, "REG", "AUTH", "Get Registration from Client", txc).await;
                    if rawc.is_err() {panic!("err");}
                    let cinfo = ClientInfo {
                        room: Room {
                            room_name: "rust_kill".to_string(), 
                            players:Vec::new(), 
                            game_state: GameState{turn: server::host::game_info::Turn { turn_state: TurnType::StartTurn }},
                        },
                        idx: num,
                        ts: TurnType::StartTurn,
                    };
                    let cjson = serde_json::to_string(&cinfo).unwrap();
                    print!("sending {} to client\n", cjson);
                    utils::serverWriteToClient(&mut writer, &utils::encode("ROOM",cjson.as_str())).await.unwrap();
                });
                join!(task);
                num += 1;

            }
        });
        join!(wait);
        while let rec = rx.recv().await {
            if rec.is_none() {
                break;
            }
            let recv = rec.unwrap();
            let player_info: Player = serde_json::from_str(&recv).expect("json deserialize failed");
            print!("Player Info: {}\n", player_info.name);
        }
        print!("ROOM IS FULFILLED!");
    });
    Ok(task)
}