#[macro_use]
extern crate tracing;

use flattiverse_connector::galaxy_hierarchy::Galaxy;
use flattiverse_connector::{FlattiverseEventKind, Vector};
use tracing_subscriber::fmt::format::FmtSpan;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        // all spans/events with a level higher than TRACE (e.g, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(tracing::Level::INFO)
        .with_line_number(true)
        .with_env_filter("flattiverse_connector=trace")
        .with_span_events(FmtSpan::CLOSE)
        // sets this to be the default, global collector for this application.
        .init();

    // spectator
    let galaxy = Galaxy::connect(
        0,
        "6df3b734005dcd57efef3deaf87d4675de608afc0555c2e1ed65aba1e04c6600",
        "Pink",
    )
    .await
    .unwrap();

    info!("Connected to galaxy: {:?}", &*galaxy.name());
    info!("    {:?}", &*galaxy.description());
    info!("Myself: {:?}", galaxy.player().name());

    galaxy.chat("Was geht in der Galaxie!?").await.unwrap();
    galaxy
        .player()
        .team()
        .chat("Was geht im Team!?")
        .await
        .unwrap();
    galaxy.player().chat("Was geht bei mir!?").await.unwrap();

    let ship = galaxy.create_classic_ship("yummy").await.unwrap();

    let mut log_tick_tack = false;
    while let Ok(event) = galaxy.next_event().await {
        if matches!(event.kind(), FlattiverseEventKind::GalaxyTick { .. }) {
            if !ship.alive() {
                if let Err(e) = ship.r#continue().await {
                    error!("Failed to continue ma ship: {e:?}");
                }
                if let Some(controls) = ship.classic_controls() {
                    let movement = Vector::from_xy(0.1, 0.0);
                    if let Err(e) = controls.r#move(movement.normalized() * 0.1).await {
                        error!("Failed to move ma ship: {e:?}");
                    }
                }
            }
        }

        match event.kind() {
            FlattiverseEventKind::GalaxyTick { .. } if !log_tick_tack => {}
            FlattiverseEventKind::GalaxyChat {
                player, message, ..
            } => {
                if let Err(e) = player
                    .chat(format!("Du hast geschrieben: {message:?}"))
                    .await
                {
                    error!("{e:?}");
                }
                info!("{event}");
            }
            _ => {
                log_tick_tack = matches!(event.kind(), FlattiverseEventKind::PingMeasured(..));
                info!("{event}");
            }
        }
    }

    // just to let everyone know
    warn!("SHUTTING DOWN");
    Ok(())
}
