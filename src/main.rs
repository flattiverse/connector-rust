use flattiverse_connector::com::Connection;

pub mod codec;
pub mod packet;
pub mod crypt;
pub mod com;

#[tokio::main]
async fn main() {
    Connection::connect("abc", "def").await.unwrap();
    println!("async");
}