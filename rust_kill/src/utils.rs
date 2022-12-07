
use std::sync::Arc;

use rocket::serde;
use ::serde::{Serialize, Deserialize};

use queues::{Queue, IsQueue};
use tokio::{net::tcp::{OwnedWriteHalf, OwnedReadHalf}, io::{BufWriter, AsyncWriteExt, BufReader, AsyncBufReadExt}, sync::mpsc::Sender};

pub fn spliter() -> char {'\x03'}

pub fn spliterKV() -> char {'\x04'}

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

use async_recursion::async_recursion;

#[async_recursion]
pub async fn server_response(mut red: BufReader<&'async_recursion mut OwnedReadHalf>, writer: &mut OwnedWriteHalf, mut cmd_from: Queue<String>, cmd_back: &str, message: &str) -> Result<String, String>{
    loop {
        let raw= red.fill_buf().await;
        let received = match raw{
            Ok(rec) => rec.to_vec(),
            Err(_) => {
                return Err("receive error of client_response\n".to_string());
            }
        };
        if received.len() == 0 {continue;}
        red.consume(received.len());
        let msg: String = String::from_utf8(received).expect("unwrap read err");
        let msgs : Vec<&str> = msg.split(spliter()).collect();
        let mut res = "";
        for m in msgs {
            if m.len() == 0 {continue;}
            if m.contains(spliter()) {continue;}
            let (kd,vd) = decode(m);
            let next = match cmd_from.remove() {
                Ok(n) => n,
                Err(_) => return Err("Too many commands!".to_string())
            };
            if kd == next {
                print!("{} : {}\n", message, vd);
                res = vd;
                continue;
            }
            return Err("Unexpected Command ".to_string() + kd);
        }
        if cmd_from.size() != 0 {
            return server_response(red, writer, cmd_from, cmd_back, message).await;
        }
        serverWriteToClient(writer, encode(cmd_back, res).as_str()).await.unwrap();
        return Ok(res.to_string());
    }
}

pub async fn read_all(mut red: BufReader<&mut OwnedReadHalf>) -> Result<(String, String), String>{
    let mut res : Vec<(String, String)> = Vec::new();
    loop {
        let raw= red.fill_buf().await;
        let received = match raw{
            Ok(rec) => rec.to_vec(),
            Err(_) => {
                return Err("receive error of client_response\n".to_string());
            }
        };
        if received.len() == 0 {continue;}
        red.consume(received.len());
        let msg: String = String::from_utf8(received).expect("unwrap read err");
        let msgs : Vec<&str> = msg.split(spliter()).collect();
        for m in msgs {
            if m.len() == 0 {continue;}
            if m.contains(spliter()) {continue;}
            let (kd,vd) = decode(m);
            return Ok((kd.to_string(), vd.to_string()));
        }
    }
}

#[async_recursion]
pub async fn client_response(mut red: BufReader<&'async_recursion mut OwnedReadHalf>, mut cmd_from: Queue<String>, message: &str) -> Result<String, String>{
    loop {
        let raw= red.fill_buf().await;
        let received = match raw{
            Ok(rec) => rec.to_vec(),
            Err(_) => {
                return Err("receive error of client_response\n".to_string());
            }
        };
        if received.len() == 0 {continue;}
        red.consume(received.len());
        let msg: String = String::from_utf8(received).expect("unwrap read err");
        let msgs : Vec<&str> = msg.split(spliter()).collect();
        let mut res = "";
        for m in msgs {
            if m.len() == 0 {continue;}
            if m.contains(spliter()) {continue;}
            let (kd,vd) = decode(m);
            let next = match cmd_from.remove() {
                Ok(n) => n,
                Err(_) => return Err("Too many commands!".to_string())
            };
            if kd == next {
                print!("{} : {}\n", message, vd);
                res = vd;
                continue;
            }
            return Err("Unexpected Command ".to_string() + kd);
        }
        return if cmd_from.size() != 0 {client_response(red, cmd_from, message).await} else {Ok(res.to_string())}
    }
}

fn print_type_of<T>(_: &T) {

}

pub fn struct_to_string<T>(obj: &T) -> (String, String)
where T: Serialize,
{
  let a = serde_json::to_string(obj).unwrap();
  let type_name = std::any::type_name::<T>().to_string();
  return (a, type_name);
}

pub fn string_to_struct<'de, T>(s: &'de String) -> T
where
  T: Deserialize<'de>,
{
  serde_json::from_str(s).unwrap()
}
