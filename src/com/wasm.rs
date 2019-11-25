use std::io::Error as IoError;
use std::io::ErrorKind as IoErrorKind;

use futures::channel::mpsc::Sender;
use futures::channel::mpsc::Receiver;
use futures::channel::mpsc::channel;
use block_modes::BlockMode;
use crate::crypt::{Aes128Cbc, to_blocks, AES128CBC_BLOCK_BYTE_LENGTH};
use crate::packet::Packet;
use wasm_bindgen::prelude::*;
use web_sys::ErrorEvent;
use web_sys::MessageEvent;
use futures::{StreamExt};
use futures::Sink;
use futures::Stream;
use bytes::{BytesMut, BufMut};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebSocket};
use js_sys::{ArrayBuffer, DataView};
use futures_util::task::{Context, Poll};
use wasm_bindgen::__rt::core::pin::Pin;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

const BLOCK_LENGTH: usize = AES128CBC_BLOCK_BYTE_LENGTH;

pub struct Connection {
    version: u16,
    sink: WebSink,
    stream: WebStream,
}

impl Connection {
    pub async fn connect(user: &str, password: &str) -> Result<Self, IoError> {

        //  TcpSocket::new("galaxy.flattiverse.com", 80);
        // Connect to an echo server
        let ws = WebSocket::new("wss://echo.websocket.org")
            .map_err(|e| IoError::from(IoErrorKind::ConnectionRefused))?;

        let onerror_callback = Closure::wrap(Box::new(move |e: ErrorEvent| {
            console_log!("error event: {:?}", e);
        }) as Box<dyn FnMut(ErrorEvent)>);
        ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
        onerror_callback.forget();

        let cloned_ws = ws.clone();
        let onopen_callback = Closure::wrap(Box::new(move |_| {
            console_log!("socket opened");
            match cloned_ws.send_with_str("ping") {
                Ok(_) => console_log!("message successfully sent"),
                Err(err) => console_log!("error sending message: {:?}", err),
            }
        }) as Box<dyn FnMut(JsValue)>);
        ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
        onopen_callback.forget();

        let (mut ws_sender, receiver) = channel(1024);

        let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
            // handle message
            let response = e.data();
            let buffer = ArrayBuffer::from(response);
            let len = buffer.byte_length() as usize;
            let view = DataView::new(&buffer, 0, len);
            let mut vec = Vec::with_capacity(len);
            for i in 0..len {
                vec[i] = view.get_uint8(i);
            }
            ws_sender.try_send(vec);
        }) as Box<dyn FnMut(MessageEvent)>);
        // set message event handler on WebSocket
        ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
        // forget the callback to keep it alive
        onmessage_callback.forget();

        let mut sink = WebSink {
            sender: ws,
        };

        let mut stream = WebStream {
            read_cache: BytesMut::new(),
            receiver,
        };




        let iv = Self::random_init_vector();
        let mut packet_data = [0u8; 64];

        let user_hash = crate::crypt::sha256(&user.to_lowercase());
        debug!("user hash: {} {:x?}", user_hash.len(), user_hash);

        (&mut packet_data[0..16]).copy_from_slice(&iv);
        for i in 0..32 {
            packet_data[i + 16] = user_hash[i] ^ iv[i % 16];
        }

        let password_hash = crate::crypt::hash_password(user, password);
        debug!("pass hash: {:x?}", password_hash);




        // let mut stream = connect.await?;
        // stream.set_nodelay(true)?;
        sink.write_all(&packet_data[..48]).await?;
        stream.read_exact(&mut packet_data[..48]).await?;

        let (server_iv, data) = (&packet_data[..48]).split_at(16);
        debug!("server iv: {} {:x?}", server_iv.len(), &server_iv[..]);
        debug!(" local iv: {} {:x?}", data.len(), &data[..]);

        let mut send = Aes128Cbc::new_var(&password_hash[..], &iv[..]).unwrap();
        let mut recv = Aes128Cbc::new_var(&password_hash[..], &server_iv[..]).unwrap();

        recv.decrypt_blocks(to_blocks(&mut packet_data[16..16+32]));
        for i in 16..32 {
            packet_data[i] = packet_data[i] ^ packet_data[i + 16];
        }

        //send.encrypt(&mut RefReadBuffer::new(&challenge[..16]), &mut RefWriteBuffer::new(&mut packet_data[..16]), false).unwrap();
        send.encrypt_blocks(to_blocks(&mut packet_data[16..32]));
        sink.write_all(&packet_data[16..32]).await?;
        stream.read_exact(&mut packet_data[..16]).await.expect("Wrong password");
        debug!("Connected to flattiverse server");

        let version = u16::from(packet_data[14]) + u16::from(packet_data[15]) * 256;

        if version != 1 {
            panic!("Invalid protocol version: {}", version);
        } else {
            debug!("Using protocol version {}", version);
        }

        /*

        let protocol = Flattiverse::new(send, recv);
        let framed = Framed::new(stream, protocol);
        let (sink, stream) = framed.split();
        */

        let version = 1;

        Ok(Self {
            version,
            sink,
            stream,
        })
    }

    pub fn version(&self) -> u16 {
        self.version
    }

    pub async fn send(&mut self, packet: Packet) -> Result<(), IoError> {
        unimplemented!()
    }

    pub async fn flush(&mut self) -> Result<(), IoError> {
        unimplemented!()
    }

    pub async fn receive(&mut self) -> Option<Result<Packet, IoError>> {
        unimplemented!()
    }

    pub fn split(self) -> (impl Sink<Packet, Error = IoError>, impl Stream<Item = Result<Packet, IoError>>) {
        (self.sink, self.stream)
    }

    fn random_init_vector() -> [u8; 16] {
        rand::random()
    }
}

struct WebSink {
    sender: WebSocket,
}

impl WebSink {
    pub async fn write_all(&mut self, data: &[u8]) -> Result<(), IoError> {
        let array = ArrayBuffer::new(data.len() as u32);
        let view = DataView::new(&array, 0, data.len());
        for i in 0..data.len() {
            view.set_uint8(i, data[i]);
        }
        self.sender
            .send_with_array_buffer(&array)
            .map_err(|e| IoError::from(IoErrorKind::ConnectionAborted))
    }
}

impl Sink<Packet> for WebSink {
    type Error = IoError;

    fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        unimplemented!()
    }

    fn start_send(self: Pin<&mut Self>, item: Packet) -> Result<(), Self::Error> {
        unimplemented!()
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        unimplemented!()
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        unimplemented!()
    }
}

struct WebStream {
    read_cache: BytesMut,
    receiver: Receiver<Vec<u8>>,
}

impl WebStream {
    pub async fn read_exact(&mut self, target: &mut [u8]) -> Result<(), IoError> {
        while self.read_cache.len() < target.len() {
            if let Some(next) = self.receiver.next().await {
                self.read_cache.put_slice(&next[..]);
            } else {
                return Err(IoError::from(IoErrorKind::UnexpectedEof));
            }
        }
        let read = self.read_cache.split_to(target.len());
        target.copy_from_slice(&read[..]);
        Ok(())
    }
}

impl Stream for WebStream {
    type Item = Result<Packet, IoError>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        unimplemented!()
    }
}