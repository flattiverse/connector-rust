use flattiverse_connector::galaxy_hierarchy::{Galaxy, NamedUnit};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        // all spans/events with a level higher than TRACE (e.g, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(tracing::Level::DEBUG)
        // sets this to be the default, global collector for this application.
        .init();

    // spectator
    let galaxy = Galaxy::connect(None, None).await.unwrap();

    eprintln!("Connected to galaxy: {:?}", galaxy.name());
    eprintln!("    {:?}", galaxy.description());
    eprintln!("Myself: {:?}", galaxy.player().name);

    tokio::time::sleep(Duration::from_secs(30)).await;

    // just to let everyone know
    eprintln!("SHUTTING DOWN");
    Ok(())
}
