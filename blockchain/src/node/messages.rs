use std::net::SocketAddr;

use uuid::Uuid;
use serde::{Serialize, Deserialize};

use crate::blockchain::{chain::Chain, block::Block, transaction::Transaction};

// Leave pgp for now, so also forget about signing for the moment
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum Messages<T>
{
    // Request: Ping a node to register to it as new peer. SYNC
    Ping((Uuid, SocketAddr)),//, openpgp::TPK)),
     // Response: Respond to a ping by sending the own PK, IP and version of the chain. ACK
    Pong((Uuid, SocketAddr, Chain<T>)),//, openpgp::TPK)),
    // Response (without request): Gossip the PK and IP of others to find conflicts and connect
    // the network.
    PeerList(Vec<(Uuid, SocketAddr)>),
    // Response (without request): Broadcast to just mined block
    Block((Uuid, Block<T>)),
    // Request: ask a peer to sign a transaction and broadcast it
    Transaction((Uuid, Transaction<T>)),
    // broadcast the latest signed transaction. A Signed Transaction should be signed by both
    // parties
    //CompleteTransaction((Uuid, Uuid, Transaction<T>)),
}
