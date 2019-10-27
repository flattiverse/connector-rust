use log::{LevelFilter, SetLoggerError};
use log4rs::append::console::ConsoleAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Config, Appender, Logger, Root};
use crate::state::{State, Event};
use crate::com::Connection;
use crate::entity::Universe;
use crate::players::Team;
use crate::requests::Requests;
use tokio::sync::oneshot;
use futures_util::FutureExt;

#[macro_use]
extern crate log;

#[macro_use]
extern crate num_derive;
extern crate num_traits;

#[macro_use]
pub mod macros;

pub mod codec;
pub mod packet;
pub mod entity;
pub mod state;
pub mod crypt;
pub mod players;
pub mod com;
pub mod requests;
pub mod io;


#[tokio::main]
async fn main() {
    init_logger(
        env!("CARGO_PKG_NAME"),
        Some(LevelFilter::Debug)
    ).unwrap();
    info!("Logger init");
    let mut connection = Connection::connect("Player2", "Password").await.unwrap();
    let mut state = State::new();

    let mut requests = Requests::new();


    while let Some(Ok(packet)) = connection.receive().await {
        debug!("Received packet with command 0x{:02x}", packet.command);
        if let Some(packet) = requests.maybe_respond(packet) {
            if let Some(event) = state.update(&packet).expect("Update failed") {
                match event {
                    Event::PlayerRemoved(_, _) => {},
                    Event::NewPlayer(_, _) => {},
                    Event::PlayerDefragmented(_, _, _) => {},
                    Event::PingUpdated(_, _) => {},
                    Event::LoginCompleted => {
                        info!("Login completed");
                        if let Some(universe) = state.universe(0) {
                            let mut join_request = universe.join_with_team(1);
                            let (sender, receiver) = oneshot::channel();
                            if let Some(id) = requests.enqueue(sender) {
                                join_request.session = id as u8;
                                connection.send(join_request).await.expect("Failed to send join request");
                                connection.flush().await.expect("Failed to flush");
                                tokio::spawn(
                                    receiver.map(|packet| {
                                        println!("Received join request response: {:?}", packet);
                                    })
                                );
                            } else {
                                warn!("Enqueue for join request failed");
                            }
                            {
                                let (part_sender, part_receiver) = oneshot::channel();
                                if let Some(id) = requests.enqueue(part_sender) {
                                    let mut part_request = universe.part();
                                    part_request.session = id as u8;
                                    connection.send(part_request).await.expect("Failed to send part request");
                                    connection.flush().await.expect("Failed to flush");
                                    tokio::spawn(
                                        part_receiver.map(|packet| {
                                            println!("Received part request response: {:?}", packet);
                                        })
                                    );
                                } else {
                                    warn!("Enqueue for part request failed")
                                }
                            }
                        } else {
                            eprintln!("No universe at given index");
                        }
                    },
                    Event::UniverseMetaInfoUpdated(index, universe) => info!("Updated universe at index {}: {:?}", index, universe.map(Universe::name)),
                    Event::UniverseTeamMetaInfoUpdated(index, universe, index_team, team) => info!("Updated team at index {} in universe {} which itself is at index {}: {:?}", index_team, universe.name, index, team.map(Team::name)),
                    Event::UniverseGalaxyMetaInfoUpdated() => {},
                }
            }
        }
    }

    // for _ in 0..100 {
        // sleep(Duration::from_millis(100));
        //connection.send(Packet::default()).await.unwrap();
        //connection.send(Packet::new_oob()).await.unwrap();
        //connection.flush().await.unwrap();
    //}

    /*
                self.universes
                    .iter()
                    .flat_map(|u| u.as_ref())
                    .for_each(|u| info!("Universe: {:#?}", u));
                    */


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
