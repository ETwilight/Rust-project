pub mod room;
use queues::queue;
use tokio::net::TcpListener;
use tokio::{net::TcpStream, task::JoinHandle, io::BufReader};

#[path="game/game_info.rs"]
mod game_info;

#[path="game.rs"]
mod game;

#[path="game/utils.rs"]
use game::utils::send_message;

use crate::data::{Message, VisibleType};
use crate::server::host::{client_addr, server_addr};
use crate::client::utils::encode;
use crate::client::utils::string_to_struct;
use crate::client::utils::struct_to_string;

use crate::client::game_info::{Player, RoleType, ClientInfo, Room};

use self::room::connectRoom;
use rocket::{tokio::sync::broadcast::Sender, serde::json::Json};
use queues::Queue;
use queues::IsQueue;
#[path="utils.rs"]
mod utils;

pub async fn connect(server_addr: &str, client_name: &str, sender: Sender<Json<Message>>) -> Result<JoinHandle<()>, ()>{
    let clt = TcpStream::connect((server_addr.to_string() + ":8080").as_str()).await.unwrap();
    let (mut reader, mut writer) = clt.into_split();
    let player = Player {
        name: client_name.to_string(),
        ip : "127.0.0.1".to_string(),
        role: RoleType::Undecided,
        state: Default::default(),
        id: 7,
    };
    let player_info = serde_json::to_string(&player);
    if player_info.is_err() {
        panic!("cannot serialize into playerInfo")
    }
    utils::clientWrite(&mut writer, utils::encode("REG", player_info.unwrap().as_str()).as_str()).await.unwrap();
    let inner_sender = sender.clone();
    let client = tokio::spawn(async move{
        let auth = match utils::client_response(BufReader::new(&mut reader), queue!["AUTH".to_string(), "ROOM".to_string()], "client get").await {
            Ok(r) => r,
            Err(e) => panic!("{}", e),
        };
        let cinfo : ClientInfo = serde_json::from_str(&auth).expect("json deserialize failed");
        connectRoom(cinfo.room.room_name.clone(), inner_sender).await;
        client_addr(cinfo.room.players[0].to_owned().ip, cinfo.idx)
    }).await.unwrap();
    Ok(main_task(client, sender.clone()).await)
}

pub async fn main_task(client_addr: String, sender: Sender<Json<Message>>) -> JoinHandle<()>{
    tokio::spawn(async move {
        let listener = TcpListener::bind(client_addr.clone()).await.unwrap();
        print!("here in spawn spawn\n");
        loop {
            let (socket, _) = listener.accept().await.unwrap();
            // Process Server Events
            let (mut reader, mut writer) = socket.into_split();
            let (k, v) = utils::read_all(BufReader::new(&mut reader)).await.unwrap();
            if k == "MSG".to_string() {
                client_receive_msg(&v, sender.clone()).await;
            }
        }
    })
}

pub async fn client_send_message(server_addr: &String, msg: String) -> Result<(), ()>{
    let address = format!("{}{}", server_addr, ":8080");
    let cstream = TcpStream::connect(address).await.unwrap();
    let writer = &mut cstream.into_split().1;
    utils::clientWrite(writer, encode("MSG", msg.as_str()).as_str()).await
}


pub async fn client_receive_msg(msg: &String, sender: Sender<Json<Message>>) {
    let original_msg:Message = string_to_struct(&msg);
    send_message(sender, original_msg.username, original_msg.message, VisibleType::All).unwrap();
}

pub async fn client_send_room(server_addr: &String, msg: String) -> Result<(), ()>{
    let address = format!("{}{}", server_addr, ":8080");
    let cstream = TcpStream::connect(address).await.unwrap();
    let writer = &mut cstream.into_split().1;
    utils::clientWrite(writer, encode("MSG", msg.as_str()).as_str()).await
}

pub async fn client_receive_room(msg: &String, sender: Sender<Room>) {
    let original_msg:Message = string_to_struct(msg);
    
}