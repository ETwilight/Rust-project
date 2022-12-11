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

use tokio::time::Duration;
use rocket::{State, Shutdown};
use rocket::fs::{relative, FileServer};
use crate::client::client_send_message;
use crate::data::{Message, Room, UserInfo};
use crate::game_info::ClientInfo;
use crate::utils::struct_to_string;
use post_event::{EndSpeakEvent, MessageEvent, UserConnectEvent, VoteEvent, VoteEventType};
use rocket::form::Form;
use rocket::fs::{relative, FileServer};
use rocket::log::LogLevel;
use rocket::response::stream::{Event, EventStream};
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::tokio::select;
use rocket::tokio::sync::broadcast::{channel, error::RecvError, Sender};
use rocket::{Shutdown, State};
use tokio::time::sleep;
use tokio::time::Duration;

#[post("/room/host", data = "<form>")]
async fn post_host_room(form: Form<UserConnectEvent>, qm: &State<Sender<Message>>, qr: &State<Sender<Room>>, qc: &State<Sender<ClientInfo>>) {
    let serverip = form.serverip.clone();
    let username = form.username.clone();
    print!("Try Host {:?}", serverip.clone());
    let _ = server::host::start().await.unwrap();
    let _ = client::connect(serverip.as_str(), &username, qm.inner().clone(), qr.inner().clone(), qc.inner().clone()).await.unwrap();
}

#[post("/room/join", data = "<form>")]
async fn post_join_room(form: Form<UserConnectEvent>, qm: &State<Sender<Message>>, qr: &State<Sender<Room>>, qc: &State<Sender<ClientInfo>>){
    let serverip = form.serverip.clone();
    let username = form.username.clone();
    print!("Try Join {:?}", form.serverip.clone());
    let _ = client::connect(serverip.as_str(), &username, qm.inner().clone(), qr.inner().clone(), qc.inner().clone()).await.unwrap();
}

#[post("/game/event", data = "<form>")]
fn post_game_event(form: Form<VoteEvent>) {}

#[post("/game/endSpeak", data = "<form>")]
fn post_end_speak(form: Form<EndSpeakEvent>) {}

/// Receive a message from a form submission and broadcast it to any receivers.
#[post("/room/message", data = "<form>")]
async fn post_message(form: Form<MessageEvent>, queue: &State<Sender<Message>>) {
    //A send "fails" if there are no active subscribers
    let message_event = form.into_inner();
    let s = struct_to_string(&message_event).0;
    client_send_message(&server_addr(), s).await.unwrap();
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

#[post("/playerInfo", data = "<form>")]
async fn post_player_info(form: Form<UserInfo>, queue: &State<Sender<UserInfo>>) {
    sleep(Duration::from_millis(1000)).await;
    let _res = queue.send(form.into_inner());
}

#[get("/clientInfo")]
async fn event_client_info(queue: &State<Sender<ClientInfo>>, mut end: Shutdown) -> EventStream![] {
    print!("Get ClientInfo");
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
            let event = Event::json(&msg);
            yield event;
        }
    }
}

#[deprecated]
fn server_addr() -> String {"10.213.0.64".to_string()}

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
    let _rocket = rocket::custom(figment)
        .manage(message_channel) //Store the sender 
        .mount("/", routes![post_message, events])
        .manage(channel::<UserInfo>(1024).0)
        .mount("/", routes![post_player_info, event_player_info])
        .manage(room_channel)
        .mount("/", routes![event_room])
        .manage(cinfo_channel)
        .mount("/", routes![event_client_info])
        .mount("/", routes![post_host_room, post_join_room])
        .mount("/", FileServer::from(relative!("/static"))).launch().await.unwrap();
    // a custom rocket build
    //let event_channel = channel::<GameEvent>(1024).0;
    //let _ = client::connect(server_addr().as_str(), "ThgilTac1", message_channel.clone(), room_channel.clone()).await.unwrap();

    Ok(())
}
