#[macro_use] extern crate rocket;
#[cfg(test)] mod tests;

use std::process::exit;

use redis::Commands;

use rocket::{State, Shutdown};
use rocket::fs::{relative, FileServer};
use rocket::form::Form;
use rocket::response::stream::{EventStream, Event};
use rocket::serde::{Serialize, Deserialize};
use rocket::tokio::sync::broadcast::{channel, Sender, error::RecvError};
use rocket::tokio::select;
use tokio::io::{self, AsyncWriteExt};
use tokio::join;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::sync::mpsc;
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
fn post(form: Form<Message>, quene: &State<Sender<Message>>){
    //A send "fails" if there are no active subscribers
    let _res = quene.send(form.into_inner());

} 

// #[post("/playerInfo", data = "<form>")]
// fn postPlayerInfo(form: Form<PlayerInfo>, quene: &State<Sender<PlayerInfo>>){
//     let _res = quene.send(form.into_inner());
//     //println!("{} days", 31)
// } 

// #[get("/events/playerInfo")]
// async fn eventPlayerInfo(queue: &State<Sender<PlayerInfo>>, mut end: Shutdown) -> EventStream![] {
//     let mut rx = queue.subscribe();
//     EventStream! {
//         loop {
//             let msg = select! {
//                 msg = rx.recv() => match msg {
//                     Ok(msg) => msg,
//                     Err(RecvError::Closed) => break,
//                     Err(RecvError::Lagged(_)) => continue,
//                 },
//                 _ = &mut end => break,
//             };

//             yield Event::json(&msg);
//         }
//     }
// }



/// Returns an infinite stream of server-sent events. Each event is a message
/// pulled from a broadcast queue sent by the `post` handler.

#[get("/events/message")]
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
#[path="utils.rs"]
mod utils;
#[path="Game/mod.rs"]
mod game;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    //server_addr tbd
    let server_addr = "192.168.178.53";
    let client_addr = "127.0.0.1";
    // server connection in parallel, currently in main, will be transferred
    //let (tx_main, mut rx_main) = mpsc::channel::<(OwnedReadHalf, OwnedWriteHalf)>(6);
    //let server = server::host::start(server_addr.clone());//, tx_main);
    // let recv = tokio::spawn(async move{
    //     loop{
    //         let rec = rx_main.recv().await;
    //         if rec.is_none() {
    //             break;
    //         }
    //         //let mut recv = rec.unwrap();
    //         //utils::serverWriteToClient(&mut recv.1, "a").await.unwrap();
    //     }
    // });
    //join!(server);

    // client connection, currently in main, will be transferred
    let client1 = client::connect::connect(server_addr.clone(), "127.0.0.1", "ThgilTac4").await.unwrap();
    let client2 = client::connect::connect(server_addr.clone(), "127.0.0.2", "ThgilTac5").await.unwrap();
    let client3 = client::connect::connect(server_addr.clone(), "127.0.0.3", "ThgilTac6").await.unwrap();
    while(true){}
    // a custom rocket build
/*
    let figment = rocket::Config::figment()
        .merge(("address", client_addr))
        .merge(("port", 8000));
    
    let _rocket = rocket::custom(figment).mount("/", routes![/* .. */])
        .manage(channel::<Message>(1024).0) //Store the sender 
        .mount("/", routes![post, events])
        .mount("/", FileServer::from(relative!("/static"))).launch().await.unwrap();
*/
    Ok(())
}

