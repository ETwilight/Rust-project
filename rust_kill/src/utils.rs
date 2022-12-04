use tokio::{net::tcp::{OwnedWriteHalf, OwnedReadHalf}, io::{BufWriter, AsyncWriteExt, BufReader, AsyncBufReadExt}};

pub fn spliter() -> char {
    '\x03'
}

pub async fn serverWriteToClient(socket: &mut OwnedWriteHalf, msg: &str) -> Result<(), ()>{
    let mut writer = BufWriter::new(socket);
    writer.write((spliter().to_string()+msg).as_bytes()).await.expect("err2");
    writer.flush().await.expect("err3");
    //println!("send from server");
    return Ok(())
}

pub async fn clientWrite(socket: &mut OwnedWriteHalf, msg: &str) -> Result<(), ()>{
    let mut writer = BufWriter::new(socket);
    writer.write((spliter().to_string()+msg).as_bytes()).await.expect("err2");
    writer.flush().await.expect("err3");
    //println!("send from client");
    return Ok(())
}

pub async fn serverRead(reader: &mut OwnedReadHalf, writer: &mut OwnedWriteHalf, pre: &str) {
    loop {
        let mut red = BufReader::new(&mut *reader);
        let raw= red.fill_buf().await;
        let received = match raw{
            Ok(rec) => rec.to_vec(),
            Err(e) => return
        };
        let len = received.len();
        if len == 0 {
            return;
        }
        // Mark the bytes read as consumed so the buffer will not return them in a subsequent read
        red.consume(len);
        let msg = String::from_utf8(received).expect("unwrap read err");
        let mut msgs : Vec<&str> = msg.split(spliter()).collect();
        for m in msgs {
            if m.len() == 0 {
                continue;
            }
            if m.contains(spliter()) {
                continue;
            }
            println!("Get from client: {} with len {}", m, m.len());
            serverWriteToClient(writer, (pre.clone().to_string() + m).as_str()).await.unwrap();
        }
    }
}

pub async fn clientRead(reader: &mut OwnedReadHalf, writer: &mut OwnedWriteHalf, pre: &str) {
    loop {
        let mut red = BufReader::new(&mut *reader);
        let raw= red.fill_buf().await;
        let received = match raw{
            Ok(rec) => rec.to_vec(),
            Err(e) => return
        };
        let len = received.len();
        if len == 0 {
            return;
        }
        // Mark the bytes read as consumed so the buffer will not return them in a subsequent read
        red.consume(len);
        let msg = String::from_utf8(received).expect("unwrap read err");
        let mut msgs : Vec<&str> = msg.split(spliter()).collect();
        for m in msgs {
            if m.len() == 0 {
                continue;
            }
            if m.contains(spliter()) {
                continue;
            }
            println!("Get from server: {} with len {}", m, m.len());
        }
    }
}