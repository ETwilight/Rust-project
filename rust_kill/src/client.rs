use std::time::Duration;

use queues::queue;
use tokio::net::TcpListener;
use tokio::time::sleep;
use tokio::{net::TcpStream, task::JoinHandle, io::BufReader};

pub mod room;
use room::connect_room;

use crate::data::{Message, VisibleType, Room};
use crate::server::host::client_addr;
use crate::client::utils::encode;
use crate::client::utils::encode_type;
use crate::client::utils::string_to_struct;
use crate::game_info::{Player, RoleType, ClientInfo};

use rocket::tokio::sync::broadcast::Sender;
use queues::Queue;
use queues::IsQueue;

use self::game::utils::{send_message, send_delay_room, send_room};
#[path="utils.rs"]
mod utils;

#[path="game.rs"]
mod game;

pub async fn connect(server_addr: &str, client_name: &str, sender_msg: Sender<Message>, sender_room: Sender<Room>, sender_cinfo: Sender<ClientInfo>) -> Result<JoinHandle<()>, ()>{
    let clt = TcpStream::connect((server_addr.to_string() + ":8080").as_str()).await.unwrap();
    let (mut reader, mut writer) = clt.into_split();
    let player = Player {
        user_info: Default::default(),
        ip : "127.0.0.1".to_string(),
        role: RoleType::Undecided,
        state: Default::default(),
        id: 7,
    };
    let player_info = serde_json::to_string(&player);
    if player_info.is_err() {
        panic!("cannot serialize into playerInfo")
    }
    utils::client_write(&mut writer, utils::encode("REG", player_info.unwrap().as_str()).as_str()).await.unwrap();
    let inner_sender = sender_msg.clone();
    let client = tokio::spawn(async move{
        let auth = match utils::client_response(BufReader::new(&mut reader), queue!["AUTH".to_string(), "CLI".to_string()], "client get").await {
            Ok(r) => r,
            Err(e) => "err".to_string()
        };
        if auth == "err".to_string() {
            print!("ERR! Cannot Authorize!\n");
        }
        let cinfo : ClientInfo = serde_json::from_str(&auth).expect("json deserialize failed");
        connect_room(cinfo.clone(), sender_cinfo).await;
        client_addr(cinfo.client_addr, cinfo.idx)
    }).await.unwrap();
    Ok(main_task(client, sender_msg.clone(), sender_room.clone()).await)
}

pub async fn main_task(client_addr: String, sender_msg: Sender<Message>, sender_room: Sender<Room>) -> JoinHandle<()>{
    tokio::spawn(async move {
        let listener = TcpListener::bind(client_addr.clone()).await.unwrap();
        loop {
            let soc = listener.accept().await;
            if soc.is_err() {
                print!("Unable to connect server!\n");
                continue;
            }
            let (socket, _) = soc.unwrap();
            // Process Server Events
            let (mut reader, _writer) = socket.into_split();
            let (k, v) = utils::read_all(BufReader::new(&mut reader)).await.unwrap();
            if k == "MSG".to_string() {
                client_receive_msg(&v, sender_msg.clone()).await;
            }
            else if k == "ROOM".to_string() {
                client_receive_room(&v, sender_room.clone()).await;
            }
        }
    })
}

#[deprecated]
pub async fn client_send_message(server_addr: &String, msg: String) -> Result<(), ()>{
    let address = format!("{}{}", server_addr, ":8080");
    let cstream = TcpStream::connect(address).await;
    if cstream.is_err() {
        print!("Unable to connect server while sending msg!\n");
        return Err(());
    }
    let writer = &mut cstream.unwrap().into_split().1;
    print!("Try to send message\n");
    utils::client_write(writer, encode("MSG", msg.as_str()).as_str()).await
}

#[deprecated]
pub async fn client_receive_msg(msg: &String, sender: Sender<Message>) {
    let msg:Message = string_to_struct(&msg);
    send_message(sender, msg.id,msg.username, msg.message, VisibleType::All).unwrap();
}

#[deprecated]
pub async fn client_send_room(server_addr: &String, room: String) -> Result<(), ()>{
    let address = format!("{}{}", server_addr, ":8080");
    let cstream = TcpStream::connect(address).await;
    if cstream.is_err() {
        print!("Unable to connect server while sending room!\n");
        return Err(());
    }
    let writer = &mut cstream.unwrap().into_split().1;
    utils::client_write(writer, encode("ROOM", room.as_str()).as_str()).await
}

pub async fn client_send_gme(server_addr: &String, gme: String) -> Result<(), ()>{
    let address = format!("{}{}", server_addr, ":8080");
    let cstream = TcpStream::connect(address).await;
    if cstream.is_err() {
        print!("Unable to connect server while sending game event!\n");
        return Err(());
    }
    let writer = &mut cstream.unwrap().into_split().1;
    utils::client_write(writer, encode("GME", gme.as_str()).as_str()).await
}

pub async fn client_receive_room(room: &String, sender: Sender<Room>) {
    let value: Room = string_to_struct(room);
    send_room(sender, value).unwrap();
}

