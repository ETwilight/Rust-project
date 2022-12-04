use tokio::{net::TcpListener, task::JoinHandle, io::{BufReader, AsyncBufReadExt}};
use serde::{Serialize, Deserialize};
#[path="../utils.rs"]
mod utils;

 
#[derive(Serialize, Deserialize, Debug)]
struct Person {
    person_id: i32,
    person_name: String
}
 
#[derive(Serialize, Deserialize, Debug)]
struct User {
    user_id: i32,
    user_name: String,
    user_password: String,
    user_person: Person
}

pub async fn start(server_addr: &str) -> Result<JoinHandle<()>, ()>{
    // the main thread to return
    let task = tokio::spawn(async move{
        let listener = TcpListener::bind("0.0.0.0".to_string() + ":8080").await.unwrap();
        println!("SERVER STARTS");
        loop {
            let (mut socket, _) = listener.accept().await.unwrap();
            let (mut reader, mut writer) = socket.into_split();
            println!("STARTS TO HANDLE NEW PLAYER:");
            utils::serverWriteToClient(&mut writer, "howdy").await.unwrap();
            tokio::spawn(async move{
                utils::serverRead(&mut reader, &mut writer, "REPLY-").await;
            }).await.unwrap();
        }
    });
    Ok(task)
}