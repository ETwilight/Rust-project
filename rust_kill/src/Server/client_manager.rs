use tokio::sync::mpsc::Receiver;
#[path="../game.rs"]
mod game;
use game::game_info::Player;

pub async fn receive(rx: &mut Receiver<String>) {
    loop{
        let rec = rx.recv().await;
        if rec.is_none() {break;}
        let recv = rec.unwrap();
        let player_info: Player = serde_json::from_str(&recv).expect("json deserialize failed");
        print!("Player Info: {}\n", player_info.name);
    }
}