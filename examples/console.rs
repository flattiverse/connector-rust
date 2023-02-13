use flattiverse_connector::universe_group::{FlattiverseEvent, UniverseGroup};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut universe_group = UniverseGroup::join_url(
        "wss://www.flattiverse.com/api/universes/beginnersGround.ws",
        env!("API_KEY"),
        None,
    )
    .await?;

    loop {
        // Just to be sure that we received details about ourself
        if let FlattiverseEvent::PlayerFullUpdate(player_id) = universe_group
            .next_event()
            .await
            .expect("Failed to process initial events")
        {
            eprintln!("Your player id is {player_id:?}");
            break;
        }
    }

    // send a happy group message!
    eprintln!(
        "SEND MSG GROUP RESULT {:?}",
        universe_group
            .chat("[Group] Hellö wie geht`s denn Soße?`; --")
            .await
            .unwrap()
            .await
    );

    // send a happy team message!
    eprintln!(
        "SEND MSG TEAM RESULT {:?}",
        universe_group[universe_group.player().team]
            .chat("[Team] Hellö wie geht`s denn Soße?`; --")
            .await
            .unwrap()
            .await
    );

    // send a happy message to ourself!
    eprintln!(
        "SEND MSG PLAYER RESULT {:?}",
        universe_group[universe_group.player_id()]
            .chat("[INNER-VOICE] Hellö wie geht`s denn Soße?`; --")
            .await
            .unwrap()
            .await
    );

    // create a new ship
    let controllable = universe_group
        .new_ship("MeinShipper")
        .await
        .expect("Failed to send new ship request")
        .await
        .expect("Failed to create new ship");

    // let another task take care about rotating the scanner
    tokio::spawn({
        // we new to clone our Arc (Atomic Ref Counter) because we *move* it to another thread
        let controllable = Arc::clone(&universe_group[controllable]);
        async move {
            // awake the engines... well, the ship at least
            controllable
                .r#continue()
                .await
                .expect("Failed to send continue request")
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
                        .expect("Failed to request scanner update")
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

    // just to let everyone know
    eprintln!("SHUTTING DOWN");
    Ok(())
}
