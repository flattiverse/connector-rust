#[macro_use]
extern crate tracing;

use flattiverse_connector::galaxy_hierarchy::Galaxy;
use flattiverse_connector::FlattiverseEventKind;
use tracing_subscriber::fmt::format::FmtSpan;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        // all spans/events with a level higher than TRACE (e.g, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(tracing::Level::INFO)
        .with_line_number(true)
        //.with_env_filter("flattiverse_connector=trace")
        .with_span_events(FmtSpan::CLOSE)
        // sets this to be the default, global collector for this application.
        .init();

    // spectator
    let galaxy = Galaxy::connect(0, None, None).await.unwrap();

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
    eprintln!("{ship:?}");

    let mut log_tick_tack = false;
    while let Ok(event) = galaxy.next_event().await {
        match event.kind() {
            FlattiverseEventKind::GalaxyTick { .. } if !log_tick_tack => {}
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
