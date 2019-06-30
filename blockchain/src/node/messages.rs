use std::net;

use uuid;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum Msg {
    Ping((uuid::Uuid, net::SocketAddr)),
    Pong((uuid::Uuid, net::SocketAddr)),
    Payload(String),
    AddrVec(Vec<(uuid::Uuid, net::SocketAddr)>),
}
