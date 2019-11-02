#[macro_use]
extern crate log;
#[macro_use]
extern crate num_derive;
extern crate num_traits;

use log::{LevelFilter, SetLoggerError};
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Config, Logger, Root};
use log4rs::encode::pattern::PatternEncoder;

use crate::connector::Connector;
use std::time::Duration;

#[macro_use]
pub mod macros;

pub mod codec;
pub mod packet;
pub mod entity;
pub mod state;
pub mod crypt;
pub mod players;
pub mod com;
pub mod command;
pub mod requests;
pub mod io;
pub mod connector;


#[tokio::main]
async fn main() {
    init_logger(Some(LevelFilter::Debug)).unwrap();
    debug!("Logger init");

    info!("Reaching out to the flattiverse...");
    let mut connector = Connector::login("Player1", "Password").await.unwrap();
    info!("Successfully logged in!");

    info!("Available universes:");
    for universe in connector.universes() {
        info!("  - {}", universe.name());

        info!("      Teams: ");
        for team in universe.teams() {
            info!("        » {}", team.name());
        }

        info!("      Galaxies: ");
        for galaxy in universe.galaxies() {
            info!("        » {}", galaxy.name());
        }

        info!("      Components: ");
        for system in universe.systems() {
            info!("        » {:?} [{}, {}]", system.kind(), system.level_start(), system.level_end());
        }
    }

    let request = connector.universe(1).map(|u| u.join_with_team(0));
    if let Some(request) = request {
        match connector.send_request(request).await.await.expect("Connector disconnected") {
            Ok(_) => info!("Joined successfully"),
            Err(e) => error!("{}", e),
        }
    }

    while let Some(event) = connector.update(Duration::from_millis(1000)).await {
        info!("Processed event: {:?}", event);
    }

    let request = connector.universe(1).map(|u| u.part());
    if let Some(request) = request {
        match connector.send_request(request).await.await.expect("Connector disconnected") {
            Ok(_) => info!("Parted successfully"),
            Err(e) => error!("{}", e)
        }
    }

    while let Some(event) = connector.update(Duration::from_millis(1000)).await {
        info!("Processed event: {:?}", event);
    }
    info!("End of main");
}

pub fn init_logger(level: Option<LevelFilter>) -> Result<::log4rs::Handle, SetLoggerError> {
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{h({d(%Y-%m-%d %H:%M:%S%.3f)}  {M:>30.30}:{L:>03}  {T:>25.25}  {l:>5}  {m})}{n}",
        )))
        .build();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .logger(Logger::builder().build( env!("CARGO_PKG_NAME"), level.unwrap_or(LevelFilter::Info)))
        .build(Root::builder().appender("stdout").build(LevelFilter::Info))
        .expect("Failed to create logger config");

    ::log4rs::init_config(config)
}
