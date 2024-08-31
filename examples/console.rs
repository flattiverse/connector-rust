use flattiverse_connector::galaxy_hierarchy::Galaxy;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // spectator
    let galaxy = Galaxy::connect(None, None).await.unwrap();

    // just to let everyone know
    eprintln!("SHUTTING DOWN");
    Ok(())
}
