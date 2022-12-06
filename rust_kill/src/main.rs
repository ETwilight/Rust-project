#[macro_use] extern crate rocket;
#[cfg(test)] mod tests;
mod game;

use tokio::time::Duration;
use redis::Commands;
use rocket::log::LogLevel;
use rocket::response::Debug;
use rocket::{State, Shutdown};
use rocket::fs::{relative, FileServer};
use rocket::form::Form;
use rocket::response::stream::{EventStream, Event};
use rocket::serde::{Serialize, Deserialize};
use rocket::tokio::sync::broadcast::{channel, Sender, error::RecvError};
use rocket::tokio::select;
use tokio::io::{self, AsyncWriteExt};
use tokio::time::sleep;


#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Message{
    #[field(validate = len(..30))]
    pub room:String, //Maximum Length is 30 for a roomName
    #[field(validate = len(..20))]
    pub username:String, //Maximum Length is 20 for a username
    pub message:String,
}

#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct PlayerInfo {
    pub username: String,
    pub clientip: String,
    pub serverip: String,
}



/// Receive a message from a form submission and broadcast it to any receivers.
#[post("/message", data = "<form>")]
fn post(form: Form<Message>, queue: &State<Sender<Message>>){
    //A send "fails" if there are no active subscribers
    let _res = queue.send(form.into_inner());

} 

 #[post("/playerInfo", data = "<form>")]
 async fn post_player_info(form: Form<PlayerInfo>, queue: &State<Sender<PlayerInfo>>){
    sleep(Duration::from_millis(1000)).await;
    print!("Howdy Whore");
    let _res = queue.send(form.into_inner());
 } 



  #[get("/playerInfo/event")]
 async fn event_player_info(queue: &State<Sender<PlayerInfo>>, mut end: Shutdown) -> EventStream![] {
    print!("Get event");
      let mut rx = queue.subscribe();
      EventStream! {
          loop {
              let msg = select! {
                  msg = rx.recv() => match msg {
                      Ok(msg) => msg,
                      Err(RecvError::Closed) => break,
                      Err(RecvError::Lagged(_)) => continue,
                  },
                 _ = &mut end => break,
              };
              yield Event::json(&msg);
          }
      }
  }



/// Returns an infinite stream of server-sent events. Each event is a message
/// pulled from a broadcast queue sent by the `post` handler.
  
#[get("/message/event")]
async fn events(queue: &State<Sender<Message>>, mut end: Shutdown) -> EventStream![] {
    let mut rx = queue.subscribe();
    EventStream! {
        loop {
            let msg = select! {
                msg = rx.recv() => match msg {
                    Ok(msg) => msg,
                    Err(RecvError::Closed) => break,
                    Err(RecvError::Lagged(_)) => continue,
                },
                _ = &mut end => break,
            };

            yield Event::json(&msg);
        }
    }
}

mod server;
mod client;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    //server_addr tbd1
    let server_addr = "192.168.178.127";
    let client_addr = "127.0.0.1";

    // server connection in parallel, currently in main, will be transferred
    let server = server::host::start(server_addr.clone()).await.unwrap();

    // client connection, currently in main, will be transferred
    let client1 = client::connect::connect(server_addr.clone(), "127.0.0.4", "ThgilTac4").await.unwrap();
    let client2 = client::connect::connect(server_addr.clone(), "127.0.0.5", "ThgilTac5").await.unwrap();
    let client3 = client::connect::connect(server_addr.clone(), "127.0.0.6", "ThgilTac6").await.unwrap();
    //let client4 = client::connect::connect(server_addr.clone(), "127.0.0.1", "ThgilTac4").await.unwrap();
    //let client5 = client::connect::connect(server_addr.clone(), "127.0.0.1", "ThgilTac5").await.unwrap();
    //let client6 = client::connect::connect(server_addr.clone(), "127.0.0.1", "ThgilTac6").await.unwrap();

    // a custom rocket build

    let figment = rocket::Config::figment()
        .merge(("address", client_addr))
        .merge(("port", 8000))
        .merge(("log_level", LogLevel::Debug));
    let _rocket = rocket::custom(figment)
        .manage(channel::<Message>(1024).0) //Store the sender 
        .mount("/", routes![post, events])
        .manage(channel::<PlayerInfo>(1025).0)
        .mount("/", routes![post_player_info, event_player_info])
        .mount("/", FileServer::from(relative!("/static"))).launch().await.unwrap();
    print!("Howdy there!");
    Ok(())
}

