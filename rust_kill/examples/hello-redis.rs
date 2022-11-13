use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
  let mut client = client::connect("10.215.0.189:6379").await?;
  client.set("hello", "world".into()).await?;
  client.set("howdy", "vagrant".into()).await?;
  let result = client.get("hello").await.unwrap();
  let result2 = client.get("howdy").await.unwrap();
  println!("got value from the server; result={:?}", result);
  println!("got value from the server; result={:?}", result2);
  Ok(())
}