#![feature(async_closure)]

use std::io;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use futures::TryStreamExt;
use generational_arena as arena;
use serde;
use tokio::codec::{FramedRead, FramedWrite};
use tokio::net::TcpListener;
use tokio::prelude::*;
use tokio::sync::Lock;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver};

use async_trait::async_trait;
use net::{BincodeDecoder, BincodeEncoder};

#[derive(Clone, Copy)]
pub struct ClientId {
    inner: arena::Index
}

#[async_trait]
pub trait NetServer<T, U> {
    async fn run(&mut self, mut receiver: UnboundedReceiver<(ClientId, T)>) -> io::Result<UnboundedReceiver<(ClientId, U)>>;
}

pub struct TcpServer {
    port: u16
}

#[async_trait]
impl<T, U> NetServer<T, U> for TcpServer
    where T: serde::Serialize + Unpin + Send + 'static,
          for<'de> U: serde::Deserialize<'de> + Unpin + Send + 'static {
    async fn run(&mut self, mut receiver: UnboundedReceiver<(ClientId, T)>) -> io::Result<UnboundedReceiver<(ClientId, U)>> {
        let (sender, other_receiver) = unbounded_channel::<(ClientId, U)>();
        let addr = SocketAddr::new(IpAddr::from(Ipv4Addr::LOCALHOST), self.port);
        let mut socket = TcpListener::bind(&addr).await?;

        let mut sinks = Lock::new(arena::Arena::new());
        let mut sinks_clone = sinks.clone();

        tokio::spawn(async move {
            while let Ok((socket, _)) = socket.accept().await {
                let (stream_raw, sink_raw) = socket.split();
                let sink = FramedWrite::new(sink_raw, BincodeEncoder::<T>::new());
                let stream = FramedRead::new(stream_raw, BincodeDecoder::<U>::new());
                let client_id = ClientId { inner: sinks.lock().await.insert(sink) };
                let sender_clone = sender.clone();
                tokio::spawn(stream
                    .map(move |message| message.map(|message| (client_id, message)))
                    .map_err(|_| ())
                    .forward(sender_clone.sink_map_err(|_| ()))
                    .map(|_| ()));
            }
        });
        tokio::spawn(async move {
            while let Some((client_id, message)) = receiver.next().await {
                if let Some(sink) = sinks_clone.lock().await.get_mut(client_id.inner) {
                    if !sink.send(message).await.is_ok() {
                        if sink.close().await.is_err() {
                            // TODO
                        }
                    }
                }
            }
        });
        Ok(other_receiver)
    }
}
