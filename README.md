# Flattiverse Rust Connector

Homepage: https://flattiverse.com/
\
GitHub-Dev: https://github.com/flattiverse/
\
C#-Connector: https://github.com/flattiverse/connector-csharp


The rust implementation of the Flattiverse connector.
It uses WebSocket to connect to the Flattiverse servers.

See the examples for how to use this crate
 - [`examples/console.rs`](examples/console.rs):
   Demonstrates very basic usage of the connector.
   Only logs to the console.
   Shows connecting to the server, sending messages, creating a ship, scanning stuff and processing events. 
 - [`examples/console.rs`](examples/console.rs): 
   Additionally, opens a [SDL2](https://crates.io/crates/sdl2)-Window, draws the own ship onto it and demonstrates how to process input-events.

### Hint compilation

This crate uses [rustls](https://github.com/rustls/rustls) for encryption.
If you are building in debug mode, consider adding the following to your `Cargo.toml` to speed up the socket encryption.

```toml
[profile.dev.package."*"]
opt-level = 3
```