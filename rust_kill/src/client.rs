pub mod room;
use tokio::{net::TcpStream, task::JoinHandle};

#[path="game/game_info.rs"]
mod game_info;

use crate::client::game_info::{Player, RoleType, ClientInfo};

use self::room::connectRoom;
#[path="utils.rs"]
mod utils;

pub async fn connect(server_addr: &str, client_addr: &str, client_name: &str) -> Result<JoinHandle<()>, ()>{
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
    // Wrap the stream in a BufReader, so we can use the BufRead methods
    //let mut reader = BufReader::new(&mut client.0);
    // Read current current data in the TcpStream
    //let (tx, mut rx) = mpsc::channel::<String>(1);
    
    let client = tokio::spawn(async move{
        let auth = match utils::clientResponse(&mut reader, "AUTH", "client get").await {
            Ok(r) => r,
            Err(e) => panic!("{}", e),
        };
        let cinfo : ClientInfo;
        if auth.0 == "ROOM" {
            cinfo = serde_json::from_str(&auth.1).expect("json deserialize failed");
        }
        else {
            let rm = match utils::clientResponse(&mut reader, "ROOM", "room info get").await {
                Ok(r) => r,
                Err(e) => panic!("{}", e),
            };
            cinfo = serde_json::from_str(&rm.1).expect("json deserialize failed");
        }
        connectRoom(cinfo.room.room_name.clone()).await;
    });
    Ok(client)
}
