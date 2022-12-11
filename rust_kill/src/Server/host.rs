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
use crate::server::host::utils::decode_type;
use crate::server::host::utils::encode;
use crate::server::host::utils::string_to_struct;
use crate::server::host::utils::struct_to_string;

#[path="../utils.rs"]
mod utils;

#[path="../game/game_loop.rs"]
mod game_loop;

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
                        room_name: "rust_kill".to_string(),
                        username: player.user_info.username.clone(),
                        client_addr: ip.clone(),
                        idx: num,
                    };
                    let cjson = serde_json::to_string(&cinfo).unwrap();
                    utils::server_write(&mut writer, &utils::encode("CLI",cjson.as_str())).await.unwrap();
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
        let mut room = r2.0;
        loop {
            // Process Client Events
            let (socket, _) = main_tcp.accept().await.unwrap();
            let (mut reader, _) = socket.into_split();
            let (k, v) = utils::read_all(BufReader::new(&mut reader)).await.unwrap();
            print!("Get results:{:?}", (k.clone(),v.clone()));
            let mut send_key = "";
            let mut send_val = "";
            if k == "GME".to_string() {
                print!("{}\n", v.clone());
                let (gme, tpe) = decode_type(&v);
                receive_from_server(&mut room, &gme.to_string(), &tpe.to_string());
                let room_json = struct_to_string(&room);
                send_key = "ROOM";
                send_val = room_json.0.as_str();
            }
            if k == "MSG".to_string() {
                print!("{}\n", v.clone());
                send_key = "MSG";
                send_val = v.as_str();
            }
            for caddr in clients.iter() {
                let cstrm = TcpStream::connect(caddr).await;
                if cstrm.is_err() {continue};
                let cstream = cstrm.unwrap();
                let writer = &mut cstream.into_split().1;
                utils::server_write(writer, encode(k.as_str(), v.as_str()).as_str()).await.unwrap();
            }
        }
    });
    Ok(task)
}

pub fn receive_from_server(room:&mut Room, game_event:&String, tpe:&String) {
    todo!()
}