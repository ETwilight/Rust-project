use std::sync::Arc;

use rocket::serde;
use tokio::{net::tcp::{OwnedWriteHalf, OwnedReadHalf}, io::{BufWriter, AsyncWriteExt, BufReader, AsyncBufReadExt}, sync::mpsc::Sender};

pub fn spliter() -> char {
    '\x03'
}

pub fn spliterKV() -> char {
    '\x04'
}

pub fn encode(key: &str, val: &str) -> String {
    (key.to_string()+spliterKV().to_string().as_str())+val
}

pub fn decode(encoded: &str) -> (&str, &str) {
    let dec = encoded.split_once(spliterKV());
    if dec.is_none() {
        panic!("cannot decode a wrong string")
    }
    dec.unwrap()
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

pub async fn serverResponse(reader: &mut OwnedReadHalf, writer: &mut OwnedWriteHalf, cmd_from: &str, cmd_back: &str, message: &str, tx: Sender<String>) -> Result<String, String>{
    loop {
        let mut red = BufReader::new(&mut *reader);
        let raw= red.fill_buf().await;
        let mut flag = false;
        let received = match raw{
            Ok(rec) => rec.to_vec(),
            Err(e) => {
                flag = true;
                vec!()
            }
        };
        if flag {
            return Err("error receiving".to_string());
        }
        let len = received.len();
        if len == 0 {
            continue;
        }
        // Mark the bytes read as consumed so the buffer will not return them in a subsequent read
        red.consume(len);
        let msg: String = String::from_utf8(received).expect("unwrap read err");
        let mut msgs : Vec<&str> = msg.split(spliter()).collect();
        let mut flag = false;
        let mut tret = "".to_string();
        for m in msgs {
            if m.len() == 0 {
                continue;
            }
            if m.contains(spliter()) {
                continue;
            }
            let (kd,vd) = decode(m);
            if kd == cmd_from {
                flag = true;
                if cmd_from == "REG" {
                    print!("{} : {}\n", message, vd);
                    serverWriteToClient(writer, encode(cmd_back, vd).as_str()).await.unwrap();
                    tx.send(vd.to_string()).await.unwrap();
                    return Ok(vd.to_string());
                }
            }
        }
        if flag{
            return Ok(tret);
        }
    }
}

pub async fn clientResponse(reader: &mut OwnedReadHalf, cmd_from: &str, message: &str) -> Result<String, String>{
    let mut red = BufReader::new(&mut *reader);
    loop {
        let raw= red.fill_buf().await;
        let mut flag = false;
        let received = match raw{
            Ok(rec) => rec.to_vec(),
            Err(e) => {
                return Err("receive error of clientResponse\n".to_string());
            }
        };
        let len = received.len();
        if len == 0 {
            continue;
        }
        // Mark the bytes read as consumed so the buffer will not return them in a subsequent read
        red.consume(len);
        let msg: String = String::from_utf8(received).expect("unwrap read err");
        let mut msgs : Vec<&str> = msg.split(spliter()).collect();
        let mut flag_auth = false;
        for m in msgs {
            if m.len() == 0 {
                continue;
            }
            if m.contains(spliter()) {
                continue;
            }
            let (kd,vd) = decode(m);
            if kd == cmd_from {
                if cmd_from == "AUTH" || cmd_from == "ROOM"{
                    print!("{} : {}\n", message, vd);
                    return Ok(vd.to_string())
                }
            }
        }
    }
}
