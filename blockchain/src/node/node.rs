use std::io;
use std::sync::Arc;
use std::sync::RwLock;
use std::collections::HashMap;
use std::net::SocketAddr;
use uuid::Uuid;
use rand::{thread_rng, Rng, rngs::ThreadRng};
use std::time::{Duration, Instant};

use futures::{Future, Stream, Sink};
use futures::sync::mpsc;
use tokio::io as tio;
use tokio::prelude::*;
use tokio::net::{TcpStream, TcpListener};
use tokio::timer::Interval;

use super::messages::Messages;
use super::codec::MessagesCodec;
use crate::blockchain::chain;

type Tx<T> = mpsc::UnboundedSender<Messages<T>>;

#[derive(Clone, Debug)]
pub struct Node<T> {
   pub id: Uuid,
   pub addr: SocketAddr,
   pub peers: HashMap<Uuid, (Tx<T>, SocketAddr)>,
   chain: chain::Chain<T>
}

//impl<T> Node<T>
//where T: 
