use flattiverse_connector::universe_group::{FlattiverseEvent, UniverseGroup};
use sdl2::event::{Event, WindowEvent};
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

const WINDOW_WIDTH: u32 = 1280;
const WINDOW_HEIGHT: u32 = 1024;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // get the sdl2 stuff going
    let sdl = sdl2::init()?;
    let video_subsystem = sdl.video()?;

    // open a new window to the user
    let window = video_subsystem
        .window(env!("CARGO_PKG_NAME"), WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .maximized()
        .opengl()
        .resizable()
        .build()
        .map_err(|e| e.to_string())?;

    // get the actual width and height (because the OS might not have cared about our size requests)
    let (mut width, mut height) = window.size();

    // get the canvas which is used for drawing
    let mut canvas = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    // because we care about inputs
    let mut events = sdl.event_pump()?;

    let mut universe_group = UniverseGroup::join_url(
        "wss://www.flattiverse.com/api/universes/beginnersGround.ws",
        env!("API_KEY"),
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
    let controllable_id = universe_group
        .new_ship("MeinShipper")
        .await
        .expect("Failed to send new ship request")
        .await
        .expect("Failed to create new ship");

    // let another task take care about rotating the scanner
    tokio::spawn({
        // we new to clone our Arc (Atomic Ref Counter) because we *move* it to another thread
        let controllable = Arc::clone(&universe_group[controllable_id]);
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

    'outer: loop {
        // process all the inputs
        while let Some(event) = events.poll_event() {
            // look, a lot of input events to choose from!
            match event {
                // apparently we want to quit
                Event::Quit { .. } => break 'outer,
                // someone told us to quit
                Event::AppTerminating { .. } => break 'outer,
                Event::AppLowMemory { .. } => {}
                Event::AppWillEnterBackground { .. } => {}
                Event::AppDidEnterBackground { .. } => {}
                Event::AppWillEnterForeground { .. } => {}
                Event::AppDidEnterForeground { .. } => {}
                Event::Display { .. } => {}
                Event::Window { win_event, .. } => {
                    match win_event {
                        WindowEvent::Resized(w, h) => {
                            width = w as u32;
                            height = h as u32;
                            eprintln!("New window size {width} / {height}");
                        }
                        WindowEvent::SizeChanged(w, h) => {
                            width = w as u32;
                            height = h as u32;
                            eprintln!("New window size {width} / {height}");
                        }
                        // dont care about the others
                        _ => {}
                    }
                }
                Event::KeyDown { .. } => {}
                Event::KeyUp { .. } => {}
                Event::TextEditing { .. } => {}
                Event::TextInput { .. } => {}
                Event::MouseMotion { .. } => {}
                Event::MouseButtonDown { .. } => {}
                Event::MouseButtonUp { .. } => {}
                Event::MouseWheel { .. } => {}
                Event::JoyAxisMotion { .. } => {}
                Event::JoyBallMotion { .. } => {}
                Event::JoyHatMotion { .. } => {}
                Event::JoyButtonDown { .. } => {}
                Event::JoyButtonUp { .. } => {}
                Event::JoyDeviceAdded { .. } => {}
                Event::JoyDeviceRemoved { .. } => {}
                Event::ControllerAxisMotion { .. } => {}
                Event::ControllerButtonDown { .. } => {}
                Event::ControllerButtonUp { .. } => {}
                Event::ControllerDeviceAdded { .. } => {}
                Event::ControllerDeviceRemoved { .. } => {}
                Event::ControllerDeviceRemapped { .. } => {}
                Event::FingerDown { .. } => {}
                Event::FingerUp { .. } => {}
                Event::FingerMotion { .. } => {}
                Event::DollarGesture { .. } => {}
                Event::DollarRecord { .. } => {}
                Event::MultiGesture { .. } => {}
                Event::ClipboardUpdate { .. } => {}
                Event::DropFile { .. } => {}
                Event::DropText { .. } => {}
                Event::DropBegin { .. } => {}
                Event::DropComplete { .. } => {}
                Event::AudioDeviceAdded { .. } => {}
                Event::AudioDeviceRemoved { .. } => {}
                Event::RenderTargetsReset { .. } => {}
                Event::RenderDeviceReset { .. } => {}
                Event::User { .. } => {}
                Event::Unknown { .. } => {}
            }
        }

        // draw ourself into the center
        if let Some(controllable) = universe_group.get_controllable(controllable_id) {
            // reset previous changes
            canvas.set_draw_color(Color::BLACK);
            canvas.clear();

            canvas
                .circle(
                    width as i16 / 2,
                    height as i16 / 2,
                    dbg!(controllable.radius * 10.0) as i16,
                    Color::GREEN,
                )
                .unwrap();

            // TODO now you draw all the other units

            // present our art on the display
            canvas.present();
        }

        // process all events that we received while doing other stuff
        while let Some(event) = universe_group.poll_next_event().await {
            match event {
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
                    break 'outer;
                }
            }
        }

        // aim for 50-60 fps
        sleep(Duration::from_millis(20)).await;
    }

    // just to let everyone know
    eprintln!("SHUTTING DOWN");
    sleep(Duration::from_secs(1)).await;
    Ok(())
}
