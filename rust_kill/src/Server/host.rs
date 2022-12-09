use std::collections::HashMap;
use std::time::Duration;

use queues::queue;
use queues::Queue;
use queues::IsQueue;
use tokio::net::TcpStream;
use tokio::time::sleep;
use tokio::{net::TcpListener, task::JoinHandle, sync::mpsc, io::BufReader};

use crate::data::Room;
use crate::server::client_manager::receive;
use crate::game_info::{Player, GameState, TurnType, ClientInfo, RevealResult};
use crate::server::host::utils::encode;
use crate::server::host::utils::string_to_struct;
use crate::server::host::utils::struct_to_string;

#[path="../utils.rs"]
mod utils;

#[path="../game/game_info.rs"]
mod game_info;

pub fn port() -> u16 {8080}
pub fn client_addr(ip:String, id:usize)->String {ip + ":" + (port() + id as u16 + 1).to_string().as_str()}
pub fn server_addr() -> String {"0.0.0.0".to_string() + ":" + port().to_string().as_str()}

pub async fn start() -> Result<JoinHandle<()>, ()>{
    // the main thread to return    
    let task = tokio::spawn(async move{
        let listener = TcpListener::bind(server_addr()).await.unwrap();
        print!("SERVER STARTS\n");
        // do not consider player log out
        let mut num : usize = 0;
        let (tx, mut rx) = mpsc::channel::<String>(6);
        let wait = tokio::spawn(async move {
            loop {
                if num >= 6 {break;}
                let (socket, _) = listener.accept().await.unwrap();
                let ip = socket.peer_addr().unwrap().ip().to_string();
                let (mut reader, mut writer) = socket.into_split();
                let txc = tx.clone();
                tokio::spawn(async move{
                    let rawc = utils::server_response(BufReader::new(&mut reader), &mut writer, queue!["REG".to_string()], "AUTH", "Get Registration from Client").await;
                    if rawc.is_err() {panic!("err");}
                    let mut player: Player = string_to_struct(&rawc.unwrap());
                    player.ip = ip.clone();
                    player.id = num;
                    let pjson = struct_to_string(&player);
                    txc.send(pjson.0).await.unwrap();
                    let cinfo = ClientInfo {
                        room: Room {
                            room_name: "rust_kill".to_string(), 
                            players:vec![player],
                            game_state: Default::default(),
                        },
                        ts: TurnType::StartTurn,
                        idx: num,
                    };
                    let cjson = serde_json::to_string(&cinfo).unwrap();
                    utils::server_write(&mut writer, &utils::encode("ROOM",cjson.as_str())).await.unwrap();
                }).await.unwrap();
                num += 1;
            }
        });
        let rec_task = tokio::spawn(async move{receive(&mut rx).await});
        let (_, r2) = (wait.await.unwrap(), rec_task.await.unwrap());
        print!("ROOM IS FULFILLED!");
        sleep(Duration::from_millis(1000)).await;
        let main_tcp = TcpListener::bind(server_addr()).await.unwrap();
        let clients = r2.1;
        loop {
            // Process Client Events
            let (socket, _) = main_tcp.accept().await.unwrap();
            let (mut reader, _) = socket.into_split();
            let (k, v) = utils::read_all(BufReader::new(&mut reader)).await.unwrap();
            print!("Get results:{:?}", (k.clone(),v.clone()));
            if k == "MSG".to_string() {
                print!("{}\n", v.clone());
                //TODO: Check who sends the message
                for caddr in clients.iter() {
                    let cstream = TcpStream::connect(caddr).await.unwrap();
                    print!("Connect client true\n");
                    let writer = &mut cstream.into_split().1;
                    utils::server_write(writer, encode(k.as_str(), v.as_str()).as_str()).await.unwrap();
                }
            }
        }
    });
    Ok(task)
}