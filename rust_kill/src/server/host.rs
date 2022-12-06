use tokio::{net::{TcpListener, tcp::{OwnedReadHalf, OwnedWriteHalf}}, task::JoinHandle, io::{BufReader, AsyncBufReadExt}, sync::mpsc::{self, Sender, Receiver}, join};
use serde::{Serialize, Deserialize};
#[path="../utils.rs"]
mod utils;
#[path="../Game/mod.rs"]
mod game;

pub async fn start(server_addr: &str) -> Result<JoinHandle<()>, ()>{
    // the main thread to return    
    let task = tokio::spawn(async move{
        let clientList: Vec<&str> = Vec::new();
        let listener = TcpListener::bind("0.0.0.0".to_string() + ":8080").await.unwrap();
        print!("SERVER STARTS\n");
        // do not consider player log out
        let mut num : usize = 0;
        let (tx, mut rx) = mpsc::channel::<String>(6);
        let wait = tokio::spawn(async move {
            loop {
                if num >= 6 {break;}
                let (socket, _) = listener.accept().await.unwrap();
                println!("{}", socket.local_addr().unwrap().ip());
                let (mut reader, mut writer) = socket.into_split();
                let txc = tx.clone();
                let task = tokio::spawn(async move{
                    utils::serverResponse(&mut reader, &mut writer, "REG", "AUTH", "Get Registration from Client", txc).await;
                });
                join!(task);
                num += 1;

            }
        });
        join!(wait);
        while let rec = rx.recv().await {
            if rec.is_none() {
                break;
            }
            let recv = rec.unwrap();
            let playerInfo: game::game_loop::Player = serde_json::from_str(&recv).expect("json deserialize failed");
            print!("Player Info: {}\n", playerInfo.name);
        }
        print!("ROOM IS FULFILLED!");
    });
    Ok(task)
}