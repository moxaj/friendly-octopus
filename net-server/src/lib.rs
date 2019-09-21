#![feature(async_closure)]

use std::io;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use futures::{SinkExt, StreamExt};
use generational_arena as arena;
use generational_arena::Arena;
use serde;
use tokio::codec::{FramedRead, FramedWrite};
use tokio::io::split;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver};
use tokio::sync::Mutex;
use tokio_io::split::WriteHalf;

use async_trait::async_trait;
use net::ClientMessage;
use net::encode::{BincodeDecoder, BincodeEncoder};

#[derive(Clone, Copy)]
pub struct ClientId {
    inner: arena::Index
}

#[async_trait]
pub trait NetServer<T, U> {
    async fn run(&mut self) -> io::Result<UnboundedReceiver<(ClientId, ClientMessage<U>)>>;
    async fn send(&mut self, client_id: ClientId, value: T);
    async fn kick(&mut self, client_id: ClientId);
}

pub struct TcpServer<T> {
    port: u16,
    sinks: Mutex<Arena<FramedWrite<WriteHalf<TcpStream>, BincodeEncoder<T>>>>,
}

impl<T> TcpServer<T> {
    fn new(port: u16) -> Self {
        Self {
            port,
            sinks: Mutex::new(Arena::new()),
        }
    }
}

#[async_trait]
impl<T, U> NetServer<T, U> for TcpServer<T>
    where T: serde::Serialize + Unpin + Send + 'static,
          for<'de> U: serde::Deserialize<'de> + Unpin + Send + 'static {
    async fn run(&mut self) -> io::Result<UnboundedReceiver<(ClientId, ClientMessage<U>)>> {
        let (sender, other_receiver) = unbounded_channel::<(ClientId, ClientMessage<U>)>();
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), self.port);
        let mut listener = TcpListener::bind(&addr).await?;
        while let Ok((socket, _)) = listener.accept().await {
            let (stream_raw, sink_raw) = split(socket);
            let sink = FramedWrite::new(sink_raw, BincodeEncoder::<T>::new());
            let mut stream = FramedRead::new(stream_raw, BincodeDecoder::<U>::new());
            let client_id = ClientId { inner: self.sinks.lock().await.insert(sink) };

            let mut sender_clone = sender.clone();
            tokio::spawn(async move {
                if sender_clone.send((client_id, ClientMessage::Connect)).await.is_err() {
                    return;
                }

                while let Some(Ok(message)) = stream.next().await {
                    if sender_clone.send((client_id, ClientMessage::Message(message))).await.is_err() {
                        break;
                    }
                }

                if sender_clone.send((client_id, ClientMessage::Disconnect)).await.is_err() {
                    // Do nothing
                }
            });
        }

        Ok(other_receiver)
    }

    async fn send(&mut self, client_id: ClientId, value: T) {
        if let Some(sink) = self.sinks.lock().await.get_mut(client_id.inner) {
            if sink.send(value).await.is_err() {
                // TODO disconnect client?
            }
        }
    }

    async fn kick(&mut self, client_id: ClientId) {
        self.sinks.lock().await.remove(client_id.inner);
    }
}