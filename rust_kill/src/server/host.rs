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
        println!("started");
        loop {
            let (mut socket, _) = listener.accept().await.unwrap();
            let (mut reader, mut writer) = socket.into_split();
            println!("accepted");
            utils::serverWrite(&mut writer, "howdy".to_string()).await.unwrap();
            utils::serverWrite(&mut writer, "howdy".to_string()).await.unwrap();
            utils::serverWrite(&mut writer, "howdy".to_string()).await.unwrap();
            utils::serverWrite(&mut writer, "howdy".to_string()).await.unwrap();
            println!("written");
            tokio::spawn(async move{
                loop{
                    let mut red = BufReader::new(&mut reader);
                    let raw= red.fill_buf().await;
                    let received = match raw{
                        Ok(rec) => rec.to_vec(),
                        Err(e) => continue,
                    };
                    let len = received.len();
                    if len == 0 {
                        continue;
                    }
                    // Mark the bytes read as consumed so the buffer will not return them in a subsequent read
                    red.consume(len);
                    let msg = String::from_utf8(received).expect("unwrap read err");
                    println!("server get: {}", msg);
                }
            }).await.unwrap();
        }
    });
    Ok(task)
}