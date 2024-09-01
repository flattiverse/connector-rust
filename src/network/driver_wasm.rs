use crate::galaxy_hierarchy::Galaxy;
use crate::network::packet::MultiPacketBuffer;
use crate::network::{ConnectError, Connection, ConnectionHandle, SenderData};
use crate::FlattiverseEvent;
use async_channel::Receiver;
use bytes::BytesMut;
use std::sync::Arc;
use web_sys::js_sys::{ArrayBuffer, JsString, Uint8Array};
use web_sys::wasm_bindgen::closure::Closure;
use web_sys::wasm_bindgen::JsCast;
use web_sys::{Blob, CloseEvent, MessageEvent, WebSocket};

pub async fn connect(
    url: &str,
    f: impl FnOnce(ConnectionHandle, Receiver<FlattiverseEvent>) -> Arc<Galaxy>,
) -> Result<Arc<Galaxy>, ConnectError> {
    debug!("Connecting to {url:?}");
    match WebSocket::new(&url) {
        Ok(websocket) => {
            debug!("Target URL seems fine");
            websocket.set_binary_type(web_sys::BinaryType::Arraybuffer);
            let (data_sender, mut data_receiver) = tokio::sync::mpsc::channel(124);
            let (event_sender, event_receiver) = async_channel::unbounded();

            let handle = ConnectionHandle::from(data_sender.clone());
            let galaxy = f(handle.clone(), event_receiver);
            let connection = Arc::new(Connection {
                handle,
                galaxy: Arc::downgrade(&galaxy),
                sender: event_sender.clone(),
            });

            let on_message_callback = Closure::<dyn FnMut(_)>::new({
                let connection = connection.clone();
                let data_sender = data_sender.clone();
                let websocket = websocket.clone();
                move |msg: MessageEvent| {
                    let array = if let Ok(buffer) = msg.data().dyn_into::<ArrayBuffer>() {
                        Uint8Array::new(&buffer)
                    } else if let Ok(blob) = msg.data().dyn_into::<Blob>() {
                        Uint8Array::new(&blob)
                    } else if let Ok(text) = msg.data().dyn_into::<JsString>() {
                        warn!("Received msg that was not expectd {text}");
                        return;
                    } else {
                        warn!("Unexpected message received");
                        return;
                    };

                    debug!("received msg, len={}", array.byte_length());
                    let data = {
                        // copying the data from into rust / wasm
                        let mut bytes = BytesMut::zeroed(array.byte_length() as usize);
                        array.copy_to(&mut bytes[..]);
                        bytes
                    };

                    let mut packet = MultiPacketBuffer::from(data);
                    while let Some(packet) = packet.next_packet() {
                        if let Err(e) = connection.handle(packet) {
                            error!("Failed to send ConnectionEvent {e:?}");
                            let _ = data_sender.try_send(SenderData::Close);
                            let _ = websocket.close();
                        }
                    }
                }
            });
            websocket.set_onmessage(Some(on_message_callback.as_ref().unchecked_ref()));
            on_message_callback.forget();

            let on_close_callback = Closure::<dyn FnMut(_)>::new({
                let websocket = websocket.clone();
                let connection = connection.clone();
                move |msg: CloseEvent| {
                    let error = ConnectError::game_error_from_http_status_code(msg.code());
                    warn!(
                        "Received close request: {msg:?}/code={} {error:?}",
                        msg.code()
                    );

                    connection.on_close();
                    let _ = data_sender.try_send(SenderData::Close);
                    let _ = websocket.close();
                }
            });
            websocket.set_onclose(Some(on_close_callback.as_ref().unchecked_ref()));
            on_close_callback.forget();

            wasm_bindgen_futures::spawn_local(async move {
                debug!("FUTURE SPAWNED");

                loop {
                    match data_receiver.recv().await {
                        Some(SenderData::Close) => {
                            warn!("WebSocket Sender received close request");
                            connection.on_close();
                            break;
                        }
                        Some(SenderData::Packet(packet)) => {
                            if let Err(e) = websocket.send_with_u8_array(&packet.into_buf()[..]) {
                                error!("Failed to send Packet: {e:?}");
                                connection.on_close();
                                break;
                            }
                        }
                        None => {
                            warn!("Receiver connection lost");
                            break;
                        }
                    }
                }

                warn!("SENDER IS SHUTTING DOWN");
                let _ = websocket.close();
            });

            Ok(galaxy)
        }
        Err(e) => Err(ConnectError::Unknown(format!("{e:?}"))),
    }
}
