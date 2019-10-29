#[macro_use]
extern crate log;
#[macro_use]
extern crate num_derive;
extern crate num_traits;

use futures_util::FutureExt;
use log::{LevelFilter, SetLoggerError};
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Config, Logger, Root};
use log4rs::encode::pattern::PatternEncoder;

use crate::com::Connection;
use crate::entity::Universe;
use crate::players::Team;
use crate::requests::Requests;
use crate::state::{Event, State};

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

    info!("Connection, connection is using protocol version {}", connection.version());

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
                    Event::LoginCompleted => {
                        info!("Login completed");
                        if let Some(universe) = state.universe(1) {
                            let mut team_id = 0;
                            for team in universe.teams.iter().filter_map(Option::<Team>::as_ref) {
                                println!(" - Team {:?}", team);
                                team_id = team.id();
                            }
                            let mut join_request = universe.join_with_team(team_id);
                            if let Some(receiver) = requests.enqueue(&mut join_request) {
                                connection.send(join_request).await.expect("Failed to send join request");
                                connection.flush().await.expect("Failed to flush");
                                tokio::spawn(
                                    receiver.map(|packet| {
                                        match packet {
                                            Err(_) => error!("Receiver disconnected"),
                                            Ok(Err(err)) => {
                                                error!("   » {}", err.general());
                                                error!("     {}", err.message());
                                            },
                                            Ok(Ok(p)) => {
                                                println!("Received join request response: {:?}", p);
                                            }
                                        }
                                    })
                                );
                            } else {
                                warn!("Enqueue for join request failed");
                            }
                            {
                                let mut part_request = universe.part();
                                if let Some(part_receiver) = requests.enqueue(&mut part_request) {
                                    connection.send(part_request).await.expect("Failed to send part request");
                                    connection.flush().await.expect("Failed to flush");
                                    tokio::spawn(
                                        part_receiver.map(|packet| {
                                            match packet {
                                                Err(_) => error!("Receiver disconnected"),
                                                Ok(Err(err)) => {
                                                    error!("   » {}", err.general());
                                                    error!("     {}", err.message());
                                                },
                                                Ok(Ok(p)) => {
                                                    println!("Received join request response: {:?}", p);
                                                }
                                            }
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
                    Event::PingUpdated(_, _) => {},
                    Event::UniverseMetaInfoUpdated(index, universe) => info!("Updated universe at index {}: {:?}", index, universe.map(Universe::name)),
                    Event::UniverseTeamMetaInfoUpdated(index, universe, index_team, team) => info!("Updated team at index {} in universe {} which itself is at index {}: {:?}", index_team, universe.name, index, team.map(Team::name)),
                    Event::UniverseGalaxyMetaInfoUpdated() => {},
                }
            }
        }
    }
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
