#[macro_use]
extern crate rocket;
mod client;
#[path = "data.rs"]
mod data;
#[path = "game/game_info.rs"]
mod game_info;
mod server;
#[cfg(test)]
mod tests;
#[path = "utils.rs"]
mod utils;

#[path = "game/game_main.rs"]
mod game_main;

#[path = "post_event.rs"]
mod post_event;

use std::io;

use mini_redis::client::Client;
use tokio::join;
use client::client_send_gme;
use tokio::time::Duration;
use rocket::{State, Shutdown};
use rocket::fs::{relative, FileServer};
use crate::client::client_send_message;
use crate::data::{Message, Room, UserInfo};
use crate::game_info::ClientInfo;
use crate::utils::struct_to_string;
use post_event::{EndSpeakEvent, MessageEvent, UserConnectEvent, VoteEvent, VoteEventType};
use rocket::form::Form;
use rocket::log::LogLevel;
use rocket::response::stream::{Event, EventStream};
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::tokio::select;
use rocket::tokio::sync::broadcast::{channel, error::RecvError, Sender};
use tokio::time::sleep;


#[post("/room/host", data = "<form>")]
async fn post_host_room(form: Form<UserConnectEvent>, rc: &State<Sender<Room>>, cc: &State<Sender<ClientInfo>>) {
    let host_event = form.into_inner();

    let _ = server::host::start().await.unwrap();
    let _ = client::connect(&host_event.serverip, &host_event.username, rc.inner().clone(), cc.inner().clone()).await.unwrap();
   
}

#[post("/room/join", data = "<form>")]
async fn post_join_room(form: Form<UserConnectEvent>, rc: &State<Sender<Room>>, cc: &State<Sender<ClientInfo>>){
    let join_event = form.into_inner();
    let _ = client::connect(&join_event.serverip, &join_event.username, rc.inner().clone(), cc.inner().clone()).await.unwrap();
}

#[post("/game/event", data = "<form>")]
async fn post_game_event(form: Form<VoteEvent>) {
    let vote_event = form.into_inner();
    let s = struct_to_string(&vote_event).0;
    client_send_gme(&server_addr(), s).await.unwrap();
}

#[post("/game/endSpeak", data = "<form>")]
async fn post_end_speak(form: Form<EndSpeakEvent>) {
    let end_speak_event = form.into_inner();
    let s = struct_to_string(&end_speak_event).0;
    client_send_gme(&server_addr(), s).await.unwrap();
}

/// Receive a message from a form submission and broadcast it to any receivers.
#[post("/room/message", data = "<form>")]
async fn post_message(form: Form<MessageEvent>) {
    //A send "fails" if there are no active subscribers
    let message_event = form.into_inner();
    let s = struct_to_string(&message_event).0;
    client_send_gme(&server_addr(), s).await.unwrap();
}

#[get("/event/room")]
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

#[get("/clientInfo")]
async fn event_client_info(queue: &State<Sender<ClientInfo>>, mut end: Shutdown) -> EventStream![] {
    let mut rx = queue.subscribe();
    EventStream! {
        loop {
            let msg = select! {
                msg = rx.recv() => match msg {
                    Ok(msg) => msg,
                    Err(RecvError::Closed) => 
                    {
                        print!("lagged");
                        break;
                    },
                    Err(RecvError::Lagged(_)) => {
                        print!("lagged");
                        continue;
                    },
                },
               _ = &mut end => break,
            };
            yield Event::json(&msg);
        }
    }
}




#[deprecated]
fn server_addr() -> String {"10.214.0.22".to_string()}

use std::env;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    print!["{}", struct_to_string(&VoteEvent{
        event_type: VoteEventType::Kill,
        voter_id: 4,
        target_id: 3,
    }).1];
    //server_addr tbd1
    let client_addr = "127.0.0.1";
    let arg = env::args().nth(1);
    let port = arg.unwrap().trim().parse::<i32>().unwrap();
    let message_channel = channel::<Message>(1024).0;
    let room_channel = channel::<Room>(1024).0;
    let cinfo_channel = channel::<ClientInfo>(1024).0;
    let figment = rocket::Config::figment()
        .merge(("address", client_addr))
        .merge(("port", port))
        .merge(("log_level", LogLevel::Debug));
    let rocket = rocket::custom(figment)
        .manage(room_channel)
        .mount("/", routes![event_room])
        .mount("/", routes![post_message, post_end_speak, post_game_event])
        .manage(message_channel) 
        .manage(cinfo_channel)
        .mount("/", routes![post_host_room, post_join_room])
        .mount("/", routes![event_client_info])
        .mount("/", FileServer::from(relative!("/static"))).launch().await.unwrap();
    // a custom rocket build
    //let event_channel = channel::<GameEvent>(1024).0;
    //let _ = client::connect(server_addr().as_str(), "ThgilTac1", message_channel.clone(), room_channel.clone()).await.unwrap();

    Ok(())
}
