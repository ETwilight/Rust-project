pub mod room;
use queues::queue;
use tokio::{net::TcpStream, task::JoinHandle, io::BufReader};

#[path="game/game_info.rs"]
mod game_info;

use crate::{client::game_info::{Player, RoleType, ClientInfo}, Message};

use self::room::connectRoom;
use rocket::tokio::sync::broadcast::Sender;
use queues::Queue;
use queues::IsQueue;
#[path="utils.rs"]
mod utils;

pub async fn connect(server_addr: &str, client_addr: &str, client_name: &str, sender: Sender<Message>) -> Result<JoinHandle<()>, ()>{
    let clt = TcpStream::connect((server_addr.to_string() + ":8080").as_str()).await.unwrap();
    let (mut reader, mut writer) = clt.into_split();
    let player = Player {
        name: client_name.to_string(),
        ip: client_addr.to_string(),
        role: RoleType::Undecided,
        state: None,
    };
    let player_info = serde_json::to_string(&player);
    if player_info.is_err() {
        panic!("cannot serialize into playerInfo")
    }
    utils::clientWrite(&mut writer, utils::encode("REG", player_info.unwrap().as_str()).as_str()).await.unwrap();
    let client = tokio::spawn(async move{
        let auth = match utils::client_response(BufReader::new(&mut reader), queue!["AUTH".to_string(), "ROOM".to_string()], "client get").await {
            Ok(r) => r,
            Err(e) => panic!("{}", e),
        };
        let cinfo : ClientInfo = serde_json::from_str(&auth).expect("json deserialize failed");
        connectRoom(cinfo.room.room_name.clone(), sender).await;
    });
    Ok(client)
}
