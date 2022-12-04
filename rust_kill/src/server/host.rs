use rocket::futures::future::ok;
use tokio::{net::TcpListener, task::JoinHandle};
pub async fn host(server_addr:String) -> Result<JoinHandle<()>, ()> {
    let task = tokio::spawn(async move{
        let listener = TcpListener::bind("0.0.0.0".to_string() + ":6379").await.unwrap();
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
    });
    Ok(task)
}