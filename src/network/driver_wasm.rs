use crate::network::packet::MultiPacketBuffer;
use crate::network::{ConnectError, Connection, ConnectionEvent, ConnectionHandle, SenderData};
use bytes::BytesMut;
use web_sys::js_sys::{ArrayBuffer, JsString, Uint8Array};
use web_sys::wasm_bindgen::closure::Closure;
use web_sys::wasm_bindgen::JsCast;
use web_sys::{Blob, CloseEvent, MessageEvent, WebSocket};

#[cfg(feature = "wasm-debug")]
mod debug {
    #[wasm_bindgen::prelude::wasm_bindgen]
    extern "C" {
        #[wasm_bindgen::prelude::wasm_bindgen(js_namespace = console)]
        pub fn log(s: &str);
    }
}

#[cfg(feature = "wasm-debug")]
macro_rules! console_log {
    ($($t:tt)*) => (debug::log(&format_args!($($t)*).to_string()))
}

#[cfg(not(feature = "wasm-debug"))]
macro_rules! console_log {
    ($($t:tt)*) => {
        let _ = format_args!($($t)*);
    };
}

pub async fn connect(url: &str) -> Result<Connection, ConnectError> {
    console_log!("Connecting to {url:?}");
    match WebSocket::new(&url) {
        Ok(websocket) => {
            console_log!("Target URL seems fine");
            websocket.set_binary_type(web_sys::BinaryType::Arraybuffer);
            let (back_sender, back_receiver) = async_channel::unbounded();
            let (sender, receiver) = async_channel::unbounded();

            let on_message_callback = Closure::<dyn FnMut(_)>::new({
                let back_sender = back_sender.clone();
                let websocket = websocket.clone();
                move |msg: MessageEvent| {
                    let array = if let Ok(buffer) = msg.data().dyn_into::<ArrayBuffer>() {
                        Uint8Array::new(&buffer)
                    } else if let Ok(blob) = msg.data().dyn_into::<Blob>() {
                        Uint8Array::new(&blob)
                    } else if let Ok(text) = msg.data().dyn_into::<JsString>() {
                        console_log!("Received msg that was not expectd {text}");
                        return;
                    } else {
                        console_log!("Unexpected message received");
                        return;
                    };

                    console_log!("received msg, len={}", array.byte_length());
                    let data = {
                        // copying the data from into rust / wasm
                        let mut bytes = BytesMut::zeroed(array.byte_length() as usize);
                        array.copy_to(&mut bytes[..]);
                        bytes
                    };

                    let mut packet = MultiPacketBuffer::from(data);
                    while let Some(packet) = packet.next_packet() {
                        if let Err(e) = back_sender.try_send(ConnectionEvent::Packet(packet)) {
                            console_log!("Failed to send ConnectionEvent {e:?}");
                            let _ = websocket.close();
                        }
                    }
                }
            });
            websocket.set_onmessage(Some(on_message_callback.as_ref().unchecked_ref()));
            on_message_callback.forget();

            let on_close_callback = Closure::<dyn FnMut(_)>::new({
                let back_sender = back_sender.clone();
                let websocket = websocket.clone();
                move |msg: CloseEvent| {
                    let error = ConnectError::game_error_from_http_status_code(msg.code());
                    console_log!(
                        "Received close request: {msg:?}/code={} {error:?}",
                        msg.code()
                    );

                    let _ = back_sender.try_send(ConnectionEvent::GameError(error));
                    let _ = back_sender.try_send(ConnectionEvent::Closed(None));
                    let _ = websocket.close();
                }
            });
            websocket.set_onclose(Some(on_close_callback.as_ref().unchecked_ref()));
            on_close_callback.forget();

            wasm_bindgen_futures::spawn_local(async move {
                console_log!("FUTURE SPAWNED");

                loop {
                    match receiver.recv().await {
                        Ok(SenderData::Packet(packet)) => {
                            if let Err(e) = websocket.send_with_u8_array(&packet.into_buf()[..]) {
                                console_log!("Faild to send Packet: {e:?}");
                                let _ = back_sender.try_send(ConnectionEvent::Closed(Some(
                                    format!("Failed to send message: {e:?}"),
                                )));
                                break;
                            }
                        }
                        Err(e) => {
                            console_log!("Receiver Error: {e:?}");
                            break;
                        }
                    }
                }

                console_log!("SENDER IS SHUTTING DOWN");
                let _ = websocket.close();
            });

            Ok(Connection::from_existing(
                ConnectionHandle::from(sender),
                back_receiver,
            ))
        }
        Err(e) => Err(ConnectError::Unknown(format!("{e:?}"))),
    }
}
