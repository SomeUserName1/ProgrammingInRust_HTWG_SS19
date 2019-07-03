use std::sync::Arc;
use std::sync::RwLock;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::time::{Duration, Instant};
use std::error;

use futures::{Future, Stream, Sink};
use futures::sync::mpsc;
use tokio::io;
use tokio::prelude::*;
use tokio::codec;
use tokio::net::{TcpStream, TcpListener};
use tokio::timer::Interval;
use uuid::Uuid;
use sequoia_openpgp as openpgp;

use super::messages::Messages;
use super::codec::MessagesCodec;

use crate::blockchain::chain::Chain;
use crate::blockchain::transaction::{Transaction, Transactional};
use crate::crypto::pgp;

type Tx<T> = mpsc::UnboundedSender<Messages<T>>;

#[derive(Clone, Debug)]
pub struct Node<T> {
   pub id: Uuid,
   keys: openpgp::TPK,
   pub addr: SocketAddr,
   pub peers: HashMap<Uuid, (Tx<T>, SocketAddr)>,
   chain: Option<Chain<T>>,
}

impl<T> Node<T>
where T: Transactional 
{
    fn new(addr: SocketAddr) -> Node<T> {
        let id = Uuid::new_v4();
        let (keys, _) = pgp::generate(id).expect("Failed to generate keys!");
        Node {
            id,
            keys,
            addr,
            peers: HashMap::new(),
            chain: None,
        }
    }

    pub fn run<I: Iterator<Item=SocketAddr>>(&self, addrs: I) -> Result<(), io::Error> {

       for addr in addrs {
            tokio::spawn(
                Node::start_client(&self, addr)
                .then(move |x| {
                    println!("client {} started {:?}", addr, x);
                    Ok(())
            }));
       }

       tokio::spawn(self.gossip(Duration::from_secs(1)).then(|_| {
           println!("gossiped");
           Ok(())
       }));
        
       tokio::run(Node::serve(&self));

      Ok(()) 
    }

    fn start_client(&self, addr: SocketAddr) -> impl Future<Item=(), Error=io::Error> {
        println!("Starting client for {}", addr);

        // Define the client
        TcpStream::connect(&addr).and_then(move |socket| {
            println!("connected! local: {:?}, peer: {:?}", socket.local_addr(), socket.peer_addr());
            let framed_socket = codec::Framed::new(socket, MessagesCodec::<T>::new());

            let (sink, stream) = framed_socket.split();
            let (tx, rx) = mpsc::unbounded();
            
            // process messages from other clients
            let read = stream.for_each(move |msg| {
                tokio::spawn(
                    Node::process(tx.clone(), msg)
                );
                Ok(())
            })
            .map_err(|e| {
                println!("Error occured {}", e);   
            });
            tokio::spawn(read);

            // Send Ping to bootstrap
            mpsc::UnboundedSender::unbounded_send(&tx.clone(), 
                                                  Messages::<T>::Ping((self.id, self.addr.clone())))
                .expect("Ping failed");
            
            tokio::spawn(sink.send_all(rx.
                                       map_err(|e| io::Error::new(io::ErrorKind::Other, "Error")))
                        .then(|_| Err(()))
            );
            Ok(())
        })
    }

    fn serve(&self) -> impl Future<Item=(), Error=io::Error> {
        println!("Starting server");

        // Listen for incoming connections, accept all and start a client for each.
        TcpListener::bind(&self.addr)
            .and_then(move |socket| {
            println!("listening on {}", self.addr);
            socket.incoming()
                .map_err(|e| println!("Accept failed with error: {:?}", e))
                .for_each(|socket| {
                    tokio::spawn(
                         Node::start_client(&self, socket.peer_addr().unwrap())
                            .then(|x| {
                                println!("spawned new client {}: {:?}", socket.peer_addr().unwrap(), x);
                                Ok(())
                        }));
                    Ok(())
                });
            Ok(())
        })
    }
}
