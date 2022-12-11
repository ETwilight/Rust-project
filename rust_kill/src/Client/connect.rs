use tokio::{net::TcpStream, task::JoinHandle, sync::mpsc};
#[path="../utils.rs"]
mod utils;

pub async fn connect(server_addr: &str, client_addr: &str, client_name: &str) -> Result<JoinHandle<()>, ()>{
    let clt = TcpStream::connect((server_addr.to_string() + ":8080").as_str()).await.unwrap();
    let (mut reader, mut writer) = clt.into_split();
    let player = game::game_info::Player {
        name: client_name.to_string(),
        ip: client_addr.to_string(),
        role: game::game_info::RoleType::Undecided,
        state: None,
    };
    let playerInfo = serde_json::to_string(&player);
    if playerInfo.is_err() {
        panic!("cannot serialize into playerInfo")
    }
    utils::clientWrite(&mut writer, utils::encode("REG", playerInfo.unwrap().as_str()).as_str()).await.unwrap();
    let client = tokio::spawn(async move{
        utils::client_response(&mut reader, "AUTH", "client get").await;
    });
    Ok(client)
}