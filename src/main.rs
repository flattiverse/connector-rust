use flattiverse_connector::com::Connection;
use log::{LevelFilter, SetLoggerError};
use log4rs::append::console::ConsoleAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Config, Appender, Logger, Root};

#[macro_use]
extern crate log;

pub mod codec;
pub mod packet;
pub mod crypt;
pub mod com;

#[tokio::main]
async fn main() {
    init_logger(
        env!("CARGO_PKG_NAME"),
        None
    ).unwrap();
    info!("Logger init");
    Connection::connect("Anonymous", "Password").await.unwrap();
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
