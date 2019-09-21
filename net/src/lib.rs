pub mod encode;

pub enum ClientMessage<T> {
    Connect,
    Disconnect,
    Message(T)
}
