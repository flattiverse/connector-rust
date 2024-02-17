use flattiverse_connector::Universe;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut galaxy = Universe::join(
        "wss://www.flattiverse.com/game/galaxies/0",
        // admin
        "CE43AE41B96111DB66D75AB943A3042755B98F10E6A09AF0D4190B0FFEC13EE8",
        // spectator
        // "0000000000000000000000000000000000000000000000000000000000000000",
        0,
    )
    .await
    .unwrap();

    /*
        tokio::spawn({
            let connection = galaxy.connection().clone();
            async move {
                for i in 1..1337 {
                    match connection.is_even(i).await {
                        Ok(result) => {
                            eprintln!("{i} is even: {result}")
                        }
                        Err(e) => {
                            eprintln!("Error: {e:?}");
                            if e.kind() == GameErrorKind::ConnectionClosed {
                                break;
                            }
                        }
                    }
                }
            }
        });
    */
    while let Ok(event) = galaxy.receive().await {
        eprintln!("{event:?}");
    }

    /*
        let mut universe_group = UniverseGroup::join_url(
            "wss://www.flattiverse.com/api/universes/beginnersGround.ws",
            env!("API_KEY"),
            None,
        )
        .await?;

        eprintln!("I am {}", universe_group.player().name);

        // send a happy group message!
        eprintln!(
            "SEND MSG GROUP RESULT {:?}",
            universe_group
                .chat("[Group] Hellö wie geht`s denn Soße?`; --")
                .await
        );

        // send a happy team message!
        eprintln!(
            "SEND MSG TEAM RESULT {:?}",
            universe_group[universe_group.player().team]
                .chat("[Team] Hellö wie geht`s denn Soße?`; --")
                .await
        );

        // send a happy message to ourself!
        eprintln!(
            "SEND MSG PLAYER RESULT {:?}",
            universe_group[universe_group.player_id()]
                .chat("[INNER-VOICE] Hellö wie geht`s denn Soße?`; --")
                .await
        );

        // create a new ship
        let controllable = universe_group
            .new_ship("MeinShipper")
            .await
            .expect("Failed to create new ship");

        // let another task take care about rotating the scanner
        tokio::spawn({
            // we new to clone our Arc (Atomic Ref Counter) because we *move* it to another thread
            let controllable = Arc::clone(&controllable);
            async move {
                // awake the engines... well, the ship at least
                controllable
                    .r#continue()
                    .await
                    .expect("Failed to awake controllable to live");

                loop {
                    for direction in [270.0, 0.0, 90.0, 180.0] {
                        // just relax a bit on the current view
                        sleep(Duration::from_secs(5)).await;
                        // update the scanner
                        eprintln!("Scan now {direction}°");
                        controllable
                            .set_scanner(direction, 300.0, 60.0, true)
                            .await
                            .expect("Failed to update scanner");
                    }
                }
            }
        });

        loop {
            // process all the events!
            match universe_group.next_event().await {
                Ok(event) => match event {
                    // apparently someone sent a message to us
                    FlattiverseEvent::ChatUnicastEvent(msg) => {
                        eprintln!(
                            "[P][{}]: {}",
                            match universe_group.get_player(msg.source) {
                                Some(player) => format!("{}", player.name),
                                None => format!("{:?}", msg.source),
                            },
                            msg.message
                        )
                    }
                    // apparently someone sent a message to our team
                    FlattiverseEvent::ChatTeamcastEvent(msg) => {
                        eprintln!(
                            "[T][{}]: {}",
                            match universe_group.get_player(msg.source) {
                                Some(player) => format!("{}", player.name),
                                None => format!("{:?}", msg.source),
                            },
                            msg.message
                        )
                    }
                    // apparently someone sent a message to our group
                    FlattiverseEvent::ChatMulticast(msg) => {
                        eprintln!(
                            "[G][{}]: {}",
                            match universe_group.get_player(msg.source) {
                                Some(player) => format!("{}", player.name),
                                None => format!("{:?}", msg.source),
                            },
                            msg.message
                        )
                    }
                    // whatever... just log it
                    other => eprintln!("EVENT: {other:?}"),
                },
                Err(e) => {
                    // oh no!
                    eprintln!("ERROR: {e:?}");
                    break;
                }
            }
        }
    **/

    // just to let everyone know
    eprintln!("SHUTTING DOWN");
    Ok(())
}
