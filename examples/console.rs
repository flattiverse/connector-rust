use flattiverse_connector::galaxy_hierarchy::Galaxy;
use tracing_subscriber::fmt::format::FmtSpan;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        // all spans/events with a level higher than TRACE (e.g, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(tracing::Level::DEBUG)
        .with_line_number(true)
        .with_env_filter("flattiverse_connector=trace")
        .with_span_events(FmtSpan::CLOSE)
        // sets this to be the default, global collector for this application.
        .init();

    // spectator
    let galaxy = Galaxy::connect(None, None).await.unwrap();

    eprintln!("Connected to galaxy: {:?}", galaxy.name());
    eprintln!("    {:?}", galaxy.description());
    eprintln!("Myself: {:?}", galaxy.player().name);

    while let Ok(event) = galaxy.next_event().await {
        eprintln!("{event}");
    }

    // just to let everyone know
    eprintln!("SHUTTING DOWN");
    Ok(())
}
