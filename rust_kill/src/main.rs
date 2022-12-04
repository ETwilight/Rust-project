#[macro_use] extern crate rocket;
#[cfg(test)] mod tests;

use rocket::serde::json::serde_json;
use rocket::{State, Shutdown};
use rocket::fs::{relative, FileServer};
use rocket::form::Form;
use rocket::response::stream::{EventStream, Event};
use rocket::serde::{Serialize, Deserialize};
use rocket::tokio::sync::broadcast::{channel, Sender, error::RecvError};
use rocket::tokio::select;


#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Message{
    #[field(validate = len(..30))]
    pub room:String, //Maximum Length is 30 for a roomName
    #[field(validate = len(..20))]
    pub username:String, //Maximum Length is 20 for a username
    pub message:String,
}

#[derive(Clone, FromForm, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct InitializeHost{
    pub hostname: String,
    pub hostip: String,
}

#[post("/InitializeHost", data = "<host>")]
fn init(host: Form<InitializeHost>, queue: &State<Sender<InitializeHost>>) {
    let _info = queue.send(host.into_inner());
}

#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct PlayerInfo {
    pub username: String,
    pub clientip: String,
    pub serverip: String,
}

#[post("/PlayerInfo", data = "<info>")]
fn setplayer(info: Form<PlayerInfo>, queue: &State<Sender<PlayerInfo>>) {
    let _info = queue.send(info.into_inner());
    //println!("{:?} {:?} {:?}",info.username, info.clientip, info.serverip);
}


/// Receive a message from a form submission and broadcast it to any receivers.
#[post("/message", data = "<form>")]
fn post(form: Form<Message>, quene: &State<Sender<Message>>){
    //A send "fails" if there are no active subscribers
    let _res = quene.send(form.into_inner());
} 




/// Returns an infinite stream of server-sent events. Each event is a message
/// pulled from a broadcast queue sent by the `post` handler.

#[get("/events")]
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


 
#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(channel::<Message>(1024).0) //Store the sender 
        .mount("/", routes![post, events])
        .mount("/", FileServer::from(relative!("/static"))) //It will be saved in a folder called "static"
}

