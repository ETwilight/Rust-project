use tokio::net::{TcpListener, TcpStream};
use bytes::Bytes;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

type Db = Arc<Mutex<HashMap<String, Bytes>>>;

#[tokio::main]
async fn main() {
    let server_addr = "127.0.0.1".to_string();
    let listener = TcpListener::bind(server_addr + ":6379").await.unwrap(); //redis://127.0.0.1/ (maybe?)
    println!("Listening");
    // let db = Arc::new(Mutex::new(HashMap::new()));
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        // let db = db.clone();
        println!("Accepted");
        //tokio::spawn(async move {
        //process(socket, db).await;
        //});
    }
}

async fn process(socket: TcpStream, db: Db) {
    /*
    use redis::Command::{self, Get, Set};
    let mut connection = Connection::new(socket);
    while let Some(frame) = connection.read_frame().await.unwrap() {
      let response = match redis::Command::from_frame(frame).unwrap() {
        Set(cmd) => {
          let mut db = db.lock().unwrap();
          db.insert(cmd.key().to_string(),cmd.value().clone());
          Frame::Simple("OK".to_string())
        }
        Get(cmd) => {
          let mut db = db.lock().unwrap();
          if let Some(value) = db.get(cmd.key()) {
            Frame::Bulk(value.clone().into())
          } else {
            Frame::Null
          }
        }
        cmd => panic!("unimplemented {:?}", cmd),
      };
      connection.write_frame(&response).await.unwrap();
    }
    */
  }