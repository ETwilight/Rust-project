use queues::queue;
use queues::Queue;
use queues::IsQueue;
use tokio::{net::TcpListener, task::JoinHandle, join, sync::mpsc, io::BufReader};

use crate::server::client_manager::receive;
use crate::server::{host::game_info::{Room, Player, GameState, TurnType, ClientInfo}, self};

#[path="../utils.rs"]
mod utils;

#[path="../game/game_info.rs"]
mod game_info;

pub async fn start() -> Result<JoinHandle<()>, ()>{
    // the main thread to return    
    let task = tokio::spawn(async move{
        let listener = TcpListener::bind("0.0.0.0".to_string() + ":8080").await.unwrap();
        print!("SERVER STARTS\n");
        // do not consider player log out
        let mut num : usize = 0;
        let _room = Room{
            room_name: "rustkill".to_string(),
            players: Vec::<Player>::new(),
            game_state: GameState{turn: server::host::game_info::Turn { turn_state: TurnType::StartTurn }},
        };
        let (tx, mut rx) = mpsc::channel::<String>(6);
        let wait = tokio::spawn(async move {
            loop {
                if num >= 6 {break;}
                let (socket, _) = listener.accept().await.unwrap();
                print!("Client Addr: {}\n", socket.peer_addr().unwrap().ip());
                let (mut reader, mut writer) = socket.into_split();
                let txc = tx.clone();
                let task = tokio::spawn(async move{
                    let rawc = utils::server_response(BufReader::new(&mut reader), &mut writer, queue!["REG".to_string()], "AUTH", "Get Registration from Client", txc).await;
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
                    utils::serverWriteToClient(&mut writer, &utils::encode("ROOM",cjson.as_str())).await.unwrap();
                });
                join!(task).0.unwrap();
                num += 1;

            }
        });
        let rec_task = tokio::spawn(async move{receive(&mut rx).await;});
        join!(wait, rec_task).0.unwrap();
        print!("ROOM IS FULFILLED!");
    });
    Ok(task)
}