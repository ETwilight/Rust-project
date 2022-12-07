#[macro_use] extern crate rocket;
#[cfg(test)] mod tests;
mod server;
mod client;
#[path="utils.rs"]
mod utils;
#[path = "game/game_info.rs"]
mod game_info;
use crate::game_info::{Room};

use tokio::time::Duration;
use rocket::log::LogLevel;
use rocket::{State, Shutdown};
use rocket::fs::{relative, FileServer};
use rocket::form::Form;
use rocket::response::stream::{EventStream, Event};
use rocket::serde::{Serialize, Deserialize};
use rocket::tokio::sync::broadcast::{channel, Sender, error::RecvError};
use rocket::tokio::select;
use tokio::time::sleep;
use rocket::serde::json::Json;
use crate::client::send_msg;

#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Message{
    #[field(validate = len(..30))]
    pub room:String, //Maximum Length is 30 for a roomName
    #[field(validate = len(..20))]
    pub username:String, //Maximum Length is 20 for a username
    pub message:String,
}

#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserInfo {
    pub username: String,
    pub serverip: String,
}



#[post("/room", data = "<form>")]
fn post_room(form: Form<Room>, queue: &State<Sender<Room>>){
    //A send "fails" if there are no active subscribers
    let _res = queue.send(form.into_inner());
} 


#[get("/room/event")]
async fn event_room(queue: &State<Sender<Room>>, mut end: Shutdown) -> EventStream![] {
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


/// Receive a message from a form submission and broadcast it to any receivers.
#[post("/message", data = "<form>")]
fn post_message(form: Form<Message>, queue: &State<Sender<Json<Message>>>){
    //A send "fails" if there are no active subscribers
    let msg = form.into_inner();
    let _res = queue.send(Json(msg));
} 

 #[post("/playerInfo", data = "<form>")]
 async fn post_player_info(form: Form<UserInfo>, queue: &State<Sender<UserInfo>>){
    sleep(Duration::from_millis(1000)).await;
    let _res = queue.send(form.into_inner());
 } 



  #[get("/playerInfo/event")]
 async fn event_player_info(queue: &State<Sender<UserInfo>>, mut end: Shutdown) -> EventStream![] {
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
async fn events(queue: &State<Sender<Json<Message>>>, mut end: Shutdown) -> EventStream![] {
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
            let value = msg.into_inner();
            send_msg(server_addr().as_str(), "TO SENT".to_string()).await.unwrap();
            let event = Event::json(&value);
            yield event;
        }
    }
}

fn server_addr() -> String {"10.195.247.228".to_string()}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    //server_addr tbd1
    let client_addr = "127.0.0.1";
    // server connection in parallel, currently in main, will be transferred
    let _ = server::host::start().await.unwrap();
    // a custom rocket build
    //let room_channel = channel::<Room>(1024).0;
    let message_channel = channel::<Json<Message>>(1024).0;
    // a custom rocket build


    let _ = client::connect(server_addr().as_str(), "ThgilTac1", message_channel.clone()).await.unwrap();
    let _ = client::connect(server_addr().as_str(), "ThgilTac2", message_channel.clone()).await.unwrap();
    let _ = client::connect(server_addr().as_str(), "ThgilTac3", message_channel.clone()).await.unwrap();
    let figment = rocket::Config::figment()
        .merge(("address", client_addr))
        .merge(("port", 8000))
        .merge(("log_level", LogLevel::Debug));
    let _rocket = rocket::custom(figment)
        .manage(message_channel) //Store the sender 
        .mount("/", routes![post_message, events])
        .manage(channel::<UserInfo>(1024).0)
        .mount("/", routes![post_player_info, event_player_info])
        .mount("/", FileServer::from(relative!("/static"))).launch().await.unwrap();

    Ok(())
}

