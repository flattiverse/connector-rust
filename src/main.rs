#![deny(intra_doc_link_resolution_failure)]

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
use crate::entity::{Privilege, Privileges, Universe};
use crate::players::Account;
use std::time::Duration;

#[macro_use]
pub mod macros;

pub mod com;
pub mod command;
pub mod connector;
pub mod crypt;
pub mod entity;
pub mod io;
pub mod packet;
pub mod players;
pub mod requesting;
pub mod requests;
pub mod state;

#[tokio::main]
async fn main() {
    init_logger(Some(LevelFilter::Info)).unwrap();
    debug!("Logger init");

    let env = std::env::args().collect::<Vec<String>>();

    info!("Reaching out to the flattiverse...");
    let mut connector = Connector::login(&env[1], &env[2]).await.unwrap();
    info!("Successfully logged in!");

    info!("Available universes:");
    for universe in connector.universes() {
        info!("  - {} ({})", universe.name(), universe.id());

        info!("      Teams: ");
        for team in universe.teams() {
            info!("        » {}", team.name());
        }

        info!("      Galaxies: ");
        for galaxy in universe.galaxies() {
            info!("        » {} ({})", galaxy.name(), galaxy.id());
        }

        info!("      Components: ");
        for system in universe.systems() {
            info!(
                "        » {:?} [{}, {}]",
                system.kind(),
                system.level_start(),
                system.level_end()
            );
        }
    }

    {
        let connector = connector.clone();
        tokio::spawn(async move {
            let mut connector = connector.await;
            let request = connector.universe(1).map(|u| u.join_with_team(0));
            if let Some(request) = request {
                match connector
                    .send_request(request)
                    .await
                    .await
                    .expect("Connector disconnected")
                {
                    Ok(_) => info!("Joined successfully"),
                    Err(e) => error!("{}", e),
                }
            }

            /*
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
            }*/
        });
    }

    tokio::spawn(connector.with_clone(query_all_accounts));
    tokio::spawn(connector.with_clone(|mut connector| async move {
        tokio::time::delay_for(Duration::from_secs(2)).await;
        info!(
            "Your({}) account info: {:?}",
            &env[1],
            connector
                .query_account_by_name(&env[1])
                .await
                .expect("Failed to query")
        );
        info!(
            "Random(asdf) account info: {:?}",
            connector
                .query_account_by_name("asdf")
                .await
                .expect("Failed to query")
        );
        query_print_universe_privileges(&mut connector, 0).await;
        query_print_universe_privileges(&mut connector, 15).await;
        query_xml_stuff(&mut connector).await;
        alter_privileges(&mut connector, &env[1]).await;
        connector.disconnect().await;
    }));

    while let Some(event) = connector.update().await {
        info!("Processed event: {:?}", event);
    }
}

async fn alter_privileges(connector: &mut Connector, acc: &str) {
    let acc = connector
        .query_account_by_name(acc)
        .await
        .expect("Failed to query account")
        .expect("Account does not exist");
    let pvs = Privileges::from(&[Privilege::Join, Privilege::ManageUniverse][..]);
    connector
        .alter_privileges_of_universe(0, &acc, pvs)
        .await
        .expect("Failed to alter privileges");
    let mut stream = connector
        .query_privileges_of_universe(0)
        .await
        .expect("Failed to query privileges");
    while let Some(Ok((acc, p))) = stream.next().await {
        info!(
            "{}: {:?}",
            acc.as_ref().map(|a| a.name()).unwrap_or_default(),
            p
        );
    }
    connector
        .reset_privileges_of_universe(0, &acc)
        .await
        .expect("Failed to reset privileges");
}

async fn query_xml_stuff(connector: &mut Connector) {
    connector.update_unit_xml(
        15, 0, "<Sun Name=\"RustUnit\" Radius=\"300\" PositionX=\"0\" PositionY=\"0\" Gravity=\"0.7\" Radiation=\"2\" PowerOutput=\"150\" />"
    ).await.expect("Failed to update");
    info!(
        "RustUnit: {}",
        connector
            .query_unit_xml_by_name(15, 0, "RustUnit")
            .await
            .expect("Failed to query RustUnit details")
    );
    connector
        .delete_unit_by_name(15, 0, "RustUnit")
        .await
        .expect("Failed to delete RustUnit");
}

async fn query_all_accounts(mut connector: Connector) {
    info!("Sending account query");
    let mut stream = connector
        .query_accounts_by_name_pattern(None, false)
        .await
        .expect("Account query failed");
    info!("Accounts:");
    while let Some(Ok(account)) = stream.next().await {
        info!("  - {:?}", account);
    }
    info!("Accounts done");
}

async fn query_print_universe_privileges(connector: &mut Connector, universe: u16) {
    info!(
        "Querying {:?} privileges",
        connector
            .universe(usize::from(universe))
            .map(Universe::name)
            .expect("Invalid universe")
    );
    let mut stream = connector
        .query_privileges_of_universe(universe)
        .await
        .expect("Failed to query universe for privileges");
    info!("Privileges:");
    while let Some(result) = stream.next().await {
        match result {
            Ok((account, privileges)) => info!(
                "  - {:?}: {:?}",
                account.as_ref().map(Account::name),
                privileges
            ),
            Err(e) => error!("{:?}", e),
        }
    }
    info!("Privileges done");
}

pub fn init_logger(level: Option<LevelFilter>) -> Result<::log4rs::Handle, SetLoggerError> {
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{h({d(%Y-%m-%d %H:%M:%S%.3f)}  {M:>30.30}:{L:>03}  {T:>25.25}  {l:>5}  {m})}{n}",
        )))
        .build();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .logger(Logger::builder().build(env!("CARGO_PKG_NAME"), level.unwrap_or(LevelFilter::Info)))
        .build(Root::builder().appender("stdout").build(LevelFilter::Info))
        .expect("Failed to create logger config");

    ::log4rs::init_config(config)
}
