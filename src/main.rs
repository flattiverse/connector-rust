use flattiverse_connector::com::Connection;

pub mod codec;
pub mod packet;
pub mod crypt;
pub mod com;

#[tokio::main]
async fn main() {
    Connection::connect("Anonymous", "Password").await.unwrap();
    println!("async");
}