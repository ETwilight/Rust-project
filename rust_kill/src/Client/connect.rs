use tokio::{net::TcpStream, task::JoinHandle, sync::mpsc};
#[path="../utils.rs"]
mod utils;
#[path="../game/mod.rs"]
mod game;

pub async fn connect(server_addr: &str, client_addr: &str, client_name: &str) -> Result<JoinHandle<()>, ()>{
    let clt = TcpStream::connect((server_addr.to_string() + ":8080").as_str()).await.unwrap();
    let (mut reader, mut writer) = clt.into_split();
    let player = game::game_loop::Player {
        name: client_name.to_string(),
        ip: client_addr.to_string(),
        role: game::game_loop::RoleType::Undecided,
    };
    let playerInfo = serde_json::to_string(&player);
    if playerInfo.is_err() {
        panic!("cannot serialize into playerInfo")
    }
    utils::clientWrite(&mut writer, utils::encode("REG", playerInfo.unwrap().as_str()).as_str()).await.unwrap();
    // Wrap the stream in a BufReader, so we can use the BufRead methods
    //let mut reader = BufReader::new(&mut client.0);
    // Read current current data in the TcpStream
    //let (tx, mut rx) = mpsc::channel::<String>(1);
    
    let client = tokio::spawn(async move{
        utils::clientResponse(&mut reader, "AUTH", "client get").await;
        
    });

    /*
    while let rec = rx.recv().await {
        if rec.is_none() {
            break;
        }
        let recv = rec.unwrap();
        let playerInfo: game::game_loop::Player = serde_json::from_str(&recv).expect("json deserialize failed");
        println!("Player Info: {}", playerInfo.name);
    }
    */
    Ok(client)
}