use log::{LevelFilter, SetLoggerError};
use log4rs::append::console::ConsoleAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Config, Appender, Logger, Root};
use crate::server::Server;
use std::thread::sleep;
use std::time::Duration;
use crate::packet::Packet;
use crate::com::Connection;

#[macro_use]
extern crate log;

#[macro_use]
extern crate num_derive;
extern crate num_traits;

pub mod codec;
pub mod packet;
pub mod entity;
pub mod server;
pub mod crypt;
pub mod players;
pub mod com;
pub mod io;

#[tokio::main]
async fn main() {
    init_logger(
        env!("CARGO_PKG_NAME"),
        None
    ).unwrap();
    info!("Logger init");
    let mut connection = Connection::connect("Anonymous", "Password").await.unwrap();
    let mut server = Server::new();


    for _ in 0..100 {
        sleep(Duration::from_millis(100));
        connection.send(Packet::default()).await.unwrap();
        connection.send(Packet::new_oob()).await.unwrap();
        connection.flush().await.unwrap();

        let packet = connection.receive().await;
        println!("{:?}", packet);
        if let Some(Ok(packet)) = packet {
            server.update(&packet).unwrap();
        }
    }


    println!("async");
}

pub fn init_logger(package: &str, level: Option<LevelFilter>) -> Result<::log4rs::Handle, SetLoggerError> {
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{h({d(%Y-%m-%d %H:%M:%S%.3f)}  {M:>30.30}:{L:>03}  {T:>25.25}  {l:>5}  {m})}{n}",
        )))
        .build();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .logger(Logger::builder().build(package, level.unwrap_or(LevelFilter::Info)))
        .build(Root::builder().appender("stdout").build(LevelFilter::Info))
        .expect("Failed to create logger config");

    ::log4rs::init_config(config)
}
