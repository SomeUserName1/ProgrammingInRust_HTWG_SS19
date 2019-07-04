use std::net::SocketAddr;
// use sequoia_openpgp as openpgp;
use uuid::Uuid;
use serde::{Serialize, Deserialize}; 

use crate::blockchain::{chain::Chain, transaction::{Transaction}}; 

/// Define messages in terms of being a request, response or a broadcast
/// FIXME: Error: openpgp::Tpk is not send so also not sync so it cant be used with futures...
///         How to change the design? remap everything with tokio to sequential?
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Messages<T> {
    // Request: Ping a node to register to it as new peer. SYNC
    Ping((Uuid, SocketAddr)), // openpgp::TPK)),
     // Response: Respond to a ping by sending the own PK, IP and version of the chain. ACK
    Pong((Uuid, SocketAddr, Chain<T>)), // openpgp::TPK, 
    // Broadcast: Gossip the PK and IP of others to find conflicts and connect
    // the network.
    PeerList(Vec<(Uuid, SocketAddr)>),
    // Broadcast: broadcast a transaction
    Transaction(Transaction<T>),
    // broadcast the latest signed transaction. A Signed Transaction should be signed by both
    // parties
    //CompleteTransaction((Uuid, Uuid, Transaction<T>)),
}
