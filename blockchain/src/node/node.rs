use std::collections::{VecDeque, HashMap};
use std::net::SocketAddr;
use std::time::{Duration, Instant};
use std::sync::{Arc, RwLock};

use futures::{Future, Stream, Sink};
use futures::sync::mpsc;
use tokio::io;
use tokio::codec;
use tokio::net::{TcpStream, TcpListener};
use tokio::timer::Interval;
use uuid::Uuid;
//use sequoia_openpgp as openpgp;
use crate::blockchain::chain::Chain;
use crate::blockchain::transaction::{Transaction, Transactional};

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
    fn new(addr: &SocketAddr) -> Node<T> {
        Node {
            inner: Arc::new(RwLock::new(NodeInner::<T>::new(*addr))),
        }
    }

    pub fn run<I: 'static + Iterator<Item=SocketAddr>>(&self, addrs: I) -> Result<(), io::Error> {
        let node = self.inner.clone();
       // spawn a server to accept incoming connections and spawn clients, which handle the
       // messages for each peer one
       tokio::run((node.read().unwrap()).serve(addrs).map_err(|e| println!("{}", e)));

      Ok(())
    }
}

impl<T> NodeInner<T>
where T: Transactional + 'static + Send + Sync,
      Self: 'static 
{
    pub fn new(addr: SocketAddr) -> NodeInner<T> {
        let id = Uuid::new_v4();
//        let (_keys, _) = keys::generate(id).expect("Failed to generate keys!");
        NodeInner {
            id,
            //keys,
            addr,
            peers: HashMap::new(),
            chain: None,
            alt_chains: VecDeque::new(),
        }
    }

    fn start_client(&self, addr: &SocketAddr) -> impl Future<Item=(), Error=io::Error> {
        println!("Starting client for {}", addr);
        let inner = self.clone();
        // Define the client
         let client = TcpStream::connect(&addr).and_then(move |socket| {
            println!("connected! local: {:?}, peer: {:?}", socket.local_addr(), socket.peer_addr());
            let framed_socket = codec::Framed::new(socket, MessagesCodec::<T>::new());

            let (sink, stream) = framed_socket.split();
            let (tx, rx): (Tx<T>, Rx<T>) = mpsc::unbounded();
            
            let tx1 = tx.clone();
            let inner1 = inner.clone();
            // process messages from other clients
            let read = stream.for_each(move |msg| {
                    inner1.clone().process(msg, &tx1.clone())
            })
            .then(|e| {
                println!("{:?}", e);
                Ok(())
            });
            tokio::spawn(read);
            
            let tx2 = tx.clone();
            let inner2 = inner.clone();
            // Send Ping to bootstrap
            mpsc::UnboundedSender::unbounded_send(&tx2.clone(),
                                                  Messages::<T>::Ping((inner2.id, inner2.addr.clone())))
               .expect("Ping failed");

            tokio::spawn(sink.send_all(
                    rx.map_err(|_| io::Error::new(io::ErrorKind::Other, "Error, {}", )))
                        .then(|_| Err(()))
            );
            Ok(())
        });
         client
    }

    pub fn serve<I: Iterator<Item=SocketAddr>>(&self, addrs: I) -> impl Future<Item=(), Error=io::Error> {
        let inner = self.clone();
        // for each address in the initial peer table, spawn a client to handle the messages
        // sent by this client
        for addr in addrs {
            tokio::spawn(
                inner.start_client(&addr)
                .then(move |x| {
                    println!("client {} started {:?}", addr, x);
                    Ok(())
            }));
        }
        
        let inner1 = inner.clone();
        let cache_reset = Interval::new(Instant::now(), Duration::from_secs(30*60)).for_each(move |_| {
            inner1.clone().alt_chains.retain(|(count, _)| count > &50);
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
                    inner.start_client(&peer).then(move |_| {
                        Ok(())
                    }));
                Ok(())
            });
        srv
    }

    fn process(&self, msg: Messages<T>, tx: &Tx<T>) -> Result<(), io::Error> {
        let mut inner = self.clone();
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
            let m: Vec<(Uuid, SocketAddr)> = inner.peers.iter()
                .map(|(k, v)| (k.clone(), v.1.clone()))
                .collect();
            let mut m1 = m.clone();
        for (tx, _) in inner.peers.values() {
            tx.unbounded_send(Messages::<T>::PeerList(m1)).expect("Shit hit the fan");
             m1 = m.clone();
        }
            Ok(())
        })
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    
    }



    fn handle_ping(&mut self, m: (Uuid, SocketAddr), tx: &Tx<T>) -> Result<(), io::Error> {
        let mut inner = self.clone();
        println!("Received ping from {:?}", m);

        match self.peers.get(&m.0) {
            None => {
                let tx1 = tx.clone();
                inner.peers.insert(m.0, (tx1, m.1));
                let tx2 = tx.clone();
                let _ = tx2.unbounded_send( Messages::<T>::Pong((inner.id, inner.addr, inner.chain.unwrap().1)))
                    .map_err(|_| io::Error::new(io::ErrorKind::Other, "tx failed"));
                Ok(())
            },
            _ => Ok(()),
        }
    }

    fn handle_pong(&mut self, m: (Uuid, SocketAddr, Chain<T>), tx: &Tx<T>) -> Result<(), io::Error> {
        println!("received pong {:?}", m);

        let chain1 = self.chain.clone();
        match chain1 {
            Some((count, self_chain)) => {
                //chains match, all good and break, else one needs majority voting
                //consensus
                match self_chain.eq(&m.2) {
                    true => self.chain = Some((count + 1, self_chain)),
                    false => self.majority_consensus(m.2),
                }
            }
            None => self.chain = Some((1, m.2)),
        }

        match self.peers.get(&m.0) {
            None => {
                self.peers.insert(m.0, (tx.clone(), m.1));
                Ok(())
            },
            _ => Ok(())

        }
    }

    fn handle_gossip(&self, m: Vec<(Uuid, SocketAddr)>) -> Result<(), io::Error> {
        let inner = self.clone();
        for (uuid, addr) in m {
            if !self.peers.contains_key(&uuid) {
                tokio::spawn(inner.start_client(&addr).then(move |_| {
                    println!("Started client for address {}", addr.clone());
                    Ok(())
                }));
            }
        };
        Ok(())
    }

    fn integrate_transaction(&mut self, m: Transaction<T>) -> Result<(), io::Error> {
        let chain1 = self.chain.clone();
        match chain1 {
            Some((_, mut chain)) => {
                chain.add_transaction(&mut vec!(m));
                if chain.get_no_curr_trans().eq(&0) {
                    for (tx, _) in self.peers.values() {
                        let chain1 = self.chain.clone();
                        let _ = tx.unbounded_send( Messages::<T>::Pong((self.id, self.addr, chain1.unwrap().1)))
                        .map_err(|_| io::Error::new(io::ErrorKind::Other, "tx failed"));
                    };
                };
                Ok(())
            }
            None => Ok(()),
        }
    }

    fn majority_consensus(&mut self, chain: Chain<T>) {
        if self.alt_chains.len() < 1 {
           self.alt_chains.push_back((1, chain.clone()));
            return;
        }

       let alt_chains1 = self.alt_chains.clone(); 
        let matched = false;
        for (mut count, sec_chain) in alt_chains1 {
            if sec_chain.eq(&chain) {
                let chain1 = self.chain.clone();
                count += 1;
                if count > chain1.unwrap().0.clone() {
                    let tmp = self.chain.clone();
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
