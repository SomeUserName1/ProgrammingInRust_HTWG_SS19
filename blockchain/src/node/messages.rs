use std::net::SocketAddr;
// use sequoia_openpgp as openpgp;
use uuid::Uuid;
use serde::{Serialize, Deserialize}; // , de::{DeserializeOwned}};

use crate::blockchain::{chain::Chain, block::Block, transaction::{Transaction}}; //, Transactional}};

/// Define messages in terms of being a request, response or a broadcast
/// Leave pgp for now, so also forget about signing for the moment
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum Messages<T> {
    // Request: Ping a node to register to it as new peer. SYNC
    Ping((Uuid, SocketAddr)), // openpgp::TPK)),
     // Response: Respond to a ping by sending the own PK, IP and version of the chain. ACK
    Pong((Uuid, SocketAddr, Option<Chain<T>>)), // openpgp::TPK, 
    // Broadcast: Gossip the PK and IP of others to find conflicts and connect
    // the network.
    PeerList(Vec<(Uuid, SocketAddr)>),
    // Broadcast: Broadcast just mined block
    Block(Block<T>),
    // Broadcast: broadcast a transaction
    Transaction(Transaction<T>),
    // broadcast the latest signed transaction. A Signed Transaction should be signed by both
    // parties
    //CompleteTransaction((Uuid, Uuid, Transaction<T>)),
}
