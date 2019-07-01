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

//#[derive(Clone, Debug, Deserialize, Serialize)]
//pub enum Msg<T>
//    where T: Transactional + Serialize + Clone + Debug
//{
    // Request: Ping a node to register to it as new peer. SYNC
//    Ping((uuid::Uuid, net::SocketAddr, openpgp::TPK)),
    // Response: Respond to a ping by sending the own PK, IP and version of the chain. ACK
//    Pong((uuid::Uuid, net::SocketAddr, openpgp::TPK, chain::Chain<T>)),
    // Response (without request): Gossip the PK and IP of others to find conflicts and connect
    // the network.
//    PeerList(Vec<(uuid::Uuid, net::SocketAddr)>),
    // Response (without request): Broadcast to just mined block
//    Block((uuid::Uuid, block::Block<T>)),
    // Request: ask a peer to sign a transaction and broadcast it
//    SignTransaction((uuid::Uuid, transaction::Transaction<T>)),
    // broadcast the latest signed transaction. A Signed Transaction should be signed by both
//    /// parties
//    CompleteTransaction((uuid::Uuid, uuid::Uuid, transaction::Transaction<T>)),
//}
