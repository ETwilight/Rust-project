use redis::{Connection, Commands, RedisResult};
use tokio::{net::{TcpListener, TcpStream}, task::JoinHandle};
pub async fn host(server_addr: String) -> Result<JoinHandle<()>, ()> {
    let task = tokio::spawn(async move{
        let listener = TcpListener::bind("0.0.0.0".to_string() + ":6379").await.unwrap();
        let svc = redis::Client::open("redis://".to_string()+server_addr.as_str()).unwrap();
        println!("Listening1");
        let mut server_msg = svc.get_connection().unwrap();
        println!("Listening2");
        //redis::cmd("SET").arg("num").arg(0).execute(&mut server_msg);
        server_msg.set::<&str, usize, usize>("num", 0);
        println!("Listening3");
        println!("{:?}", server_msg.get("num").unwrap_or(1005));
        println!("Listening3.5");
        server_msg.set::<String, Vec<String>, Vec<String>>("clients".to_string(), vec!());
        println!("Listening4");
        loop {
            let (socket, _) = listener.accept().await.unwrap();
            let aut = auth(server_addr.clone()).await;
            match aut{
                Ok(()) => {println!("{:?}", "accept")},
                Err(()) => break,
            }
        }
    });
    Ok(task)
}

pub async fn auth(server_addr: String) -> Result<(), ()> {
    let mut svc = redis::Client::open("redis://".to_string()+server_addr.as_str()).unwrap().get_connection().unwrap();
    let mut num: usize = svc.get::<String, usize>("num".to_string()).unwrap();
    loop {
        let mut nv : Vec<String> = svc.get::<String, Vec<String>>("clients".to_string()).unwrap_or(vec!());
        if nv.len() <= num {
            continue
        }
        if num >= 6 {
            svc.set::<String, i32, i32>("auth".to_string() + nv[num].clone().as_str(), -1);
            return Err(())
        }
        svc.set::<String, i32, i32>("auth".to_string() + nv[num].clone().as_str(), num as i32);
        svc.set::<String, usize, usize>("num".to_string(), num+1);
        return Ok(())
    }
}