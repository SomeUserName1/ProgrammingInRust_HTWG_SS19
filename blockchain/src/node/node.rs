use std::collections::{VecDeque, HashMap};
use std::net::SocketAddr;
use std::time::{Duration, Instant};
use std::sync::{Arc, RwLock};

use futures::{Future, Stream, Sink};
use futures::sync::mpsc;
use tokio::io;
use tokio::prelude::*;
use tokio::codec;
use tokio::net::{TcpStream, TcpListener};
use tokio::timer::Interval;
use uuid::Uuid;
//use sequoia_openpgp as openpgp;
use crate::blockchain::chain::Chain;
use crate::blockchain::transaction::{Transaction, Transactional};
use crate::crypto::keys; // {keys, sign, cipher};

use super::messages::Messages;
use super::codec::MessagesCodec;

type Tx<T> = mpsc::UnboundedSender<Messages<T>>;
type Rx<T> = mpsc::UnboundedReceiver<Messages<T>>;

#[derive(Clone, Debug)]
pub struct Node<T> {
    inner: Arc<RwLock<NodeInner<T>>>,
}

#[derive(Clone, Debug)]
pub struct NodeInner<T> {
   pub id: Uuid,
   //keys: openpgp::TPK,
   pub addr: SocketAddr,
   pub peers: HashMap<Uuid, (Tx<T>, SocketAddr)>,
   chain: Option<(u32, Chain<T>)>,
   alt_chains: VecDeque<(u32, Chain<T>)>,
}

impl<T> Node<T> 
where T: Transactional + Send + Sync + 'static 
{
    fn new(addr: SocketAddr) -> Node<T> {
        Node {
            inner: Arc::new(RwLock::new(NodeInner::<T>::new(addr))),
        }
    }

    pub fn run<I: Iterator<Item=SocketAddr>>(&self, addrs: I) -> Result<(), io::Error> {
        let inner = self.inner.clone();
       // spawn a server to accept incoming connections and spawn clients, which handle the
       // messages for each peer one
       tokio::run(inner.serve(addrs).map_err(|e| println!("{}", e)));

      Ok(())
    }
}

impl<T> NodeInner<T>
where T: Transactional + 'static + Send + Sync,
      Self: 'static 
{
    pub fn new(addr: SocketAddr) -> NodeInner<T> {
        let id = Uuid::new_v4();
        let (_keys, _) = keys::generate(id).expect("Failed to generate keys!");
        NodeInner {
            id,
            //keys,
            addr,
            peers: HashMap::new(),
            chain: None,
            alt_chains: VecDeque::new(),
        }
    }

    fn start_client(&self, addr: SocketAddr) -> impl Future<Item=(), Error=io::Error> + 'static {
        println!("Starting client for {}", addr);
        let inner = self.clone();
        // Define the client
         TcpStream::connect(&addr).and_then(|socket| {
            println!("connected! local: {:?}, peer: {:?}", socket.local_addr(), socket.peer_addr());
            let framed_socket = codec::Framed::new(socket, MessagesCodec::<T>::new());

            let (sink, stream) = framed_socket.split();
            let (tx, rx): (Tx<T>, Rx<T>) = mpsc::unbounded();

            // process messages from other clients
            let read = stream.for_each(|msg| {
                    inner.clone().process(msg, tx.clone())
            })
            .then(|e| {
                println!("{:?}", e);
                Ok(())
            });
            tokio::spawn(read);

            // Send Ping to bootstrap
            mpsc::UnboundedSender::unbounded_send(&tx.clone(),
                                                  Messages::<T>::Ping((inner.id, inner.addr.clone())))
               .expect("Ping failed");

            tokio::spawn(sink.send_all(
                    rx.map_err(|e| io::Error::new(io::ErrorKind::Other, "Error")))
                        .then(|_| Err(()))
            );
            Ok(())
        })
    }

    pub fn serve<I: Iterator<Item=SocketAddr>>(&self, addrs: I) -> impl Future<Item=(), Error=io::Error> + 'static {
        let inner = self.clone();
        // for each address in the initial peer table, spawn a client to handle the messages
        // sent by this client
        for addr in addrs {
            tokio::spawn(
                inner.start_client(addr)
                .then(move |x| {
                    println!("client {} started {:?}", addr, x);
                    Ok(())
            }));
        }

        let cache_reset = Interval::new(Instant::now(), Duration::from_secs(30*60)).for_each(|_| {
            inner.alt_chains.retain(|(count, chain)| count > &50);
            Ok(())
            }).map_err(|e| panic!("interval errored, {:?}", e));
        // Delete the list of alternative chains all 30 min
        tokio::spawn(
           cache_reset
        );

       // start gossiping the peer lists to others
       tokio::spawn(self.gossip(Duration::from_secs(3)).then(|_| {
           println!("gossiped");
           Ok(())
       }));

        println!("Starting server");

        // Listen for incoming connections, accept all and start a client for each.
        let listener =  TcpListener::bind(&self.addr).unwrap();
        println!("listening on {}", self.addr);

        let srv = listener.incoming()
            .for_each(move |socket| {
                let peer = socket.peer_addr().unwrap();
                tokio::spawn(
                    inner.start_client(peer).then(move |x| {
                        Ok(())
                    }));
                Ok(())
            });
        srv
    }

    fn process(&self, msg: Messages<T>, tx: Tx<T>) -> Result<(), io::Error> {
        let inner = self.clone();
        match msg {
            Messages::<T>::Ping(m) => inner.handle_ping(m, tx),
            Messages::<T>::Pong(m) => inner.handle_pong(m, tx),
            Messages::<T>::PeerList(m) => inner.handle_gossip(m),
            Messages::<T>::Transaction(m) => inner.integrate_transaction(m),
        }
    }

    fn gossip(&self, duration: Duration) -> impl Future<Item=(), Error=io::Error> + 'static {
        let inner = self.clone();
        Interval::new(Instant::now(), duration).for_each(move |_| {
            let m = inner.peers.iter()
                .map(|(k, v)| (k.clone(), v.1.clone()))
                .collect();
        for (tx, addr) in inner.peers.values() {
            tx.unbounded_send(Messages::<T>::PeerList(m)).expect("Shit hit the fan");
        }
            Ok(())
        })
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    
    }



    fn handle_ping(&mut self, m: (Uuid, SocketAddr), tx: Tx<T>) -> Result<(), io::Error> {
        let inner = self.clone();
        println!("Received ping from {:?}", m);

        match self.peers.get(&m.0) {
            Some(_) => Ok(()),
            None => {
                let tx1 = tx.clone();
                inner.peers.insert(m.0, (tx, m.1));
                tx.send( Messages::<T>::Pong((inner.id, inner.addr, inner.chain.unwrap().1)))
                    .map_err(|_| io::Error::new(io::ErrorKind::Other, "tx failed"));
                Ok(())
            }
        };
        Ok(())
    }

    fn handle_pong(&mut self, m: (Uuid, SocketAddr, Chain<T>), tx: Tx<T>) -> Result<(), io::Error> {
        println!("received pong {:?}", m);
        match self.chain {
            Some((count, self_chain)) => {
                //chains match, all good and break, else one needs majority voting
                //consensus
                match self_chain.eq(&m.2) {
                    true => self.chain.unwrap().0 = count + 1,
                    false => self.majority_consensus(m.2),
                }
            }
            None => self.chain = Some((1, m.2)),
        }

        match self.peers.get(&m.0) {
            Some(_) => Ok(()),
            None => {
                self.peers.insert(m.0, (tx, m.1));
                Ok(())
            }
        }
    }

    fn handle_gossip(self, m: Vec<(Uuid, SocketAddr)>) -> Result<(), io::Error> {
        let inner = self.clone();
        for (uuid, addr) in m {
            if !self.peers.contains_key(&uuid) {
                tokio::spawn(inner.start_client(addr).then(|_| {
                    println!("Started client for address {}", addr);
                    Ok(())
                }));
            }
        };
        Ok(())
    }

    fn integrate_transaction(&self, m: Transaction<T>) -> Result<(), io::Error> {
        match self.chain {
            Some((count, chain)) => {
                chain.add_transaction(&mut vec!(m));
                if chain.get_no_curr_trans().eq(&0) {
                    for (tx, addr) in self.peers.values() {
                        tx.unbounded_send( Messages::<T>::Pong((self.id, self.addr, self.chain.unwrap().1)))
                        .map_err(|_| io::Error::new(io::ErrorKind::Other, "tx failed"));
                    };
                };
                Ok(())
            }
            None => Ok(()),
        }

    }

    fn majority_consensus(&self, chain: Chain<T>) {
        if self.alt_chains.len() < 1 {
           self.alt_chains.push_back((1, chain));
            return;
        }

        let matched = false;
        for (count, sec_chain) in self.alt_chains {
            if sec_chain.eq(&chain) {
                count += 1;
                if count > self.chain.unwrap().0 {
                    let tmp = self.chain;
                    self.chain = Some((count, sec_chain));
                    self.alt_chains.push_front(tmp.unwrap());
                }
            }
        }
        if matched.eq(&false) {
            self.alt_chains.push_back((1, chain));
        }
    }
}
