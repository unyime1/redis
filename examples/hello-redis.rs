use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = client::connect("127.0.0.1:6379").await?;

    println!("0");
    client.set("hello", "world".into()).await?;
    println!("1");

    let result = client.get("hello").await?;

    println!("got value from the server; result={:?}", result);

    Ok(())
}