# PIR Blockchain
## Overall Architecture
1. node: chain, mining, transaction, mining & further consensus; Pulling all other parts together + cli
mechanisms (main structs & server aka backend)
2. crypto: KeyGen, Hashing, Encryption, signature, Merkle (cryptography)
3. p2p: Messages, Routing/Peer list, Channels (Communication)
4. storage: persist information (chain bookkeeping & mining)
5. __Optional__ wallet: we would need a wallet __in theory__ to manage keys, create tranactions, provide a cli and sth 
UI-like

## Node: Main Module

The node module shall serve as the main module of the backend:
- [ ] blockchain: chain, block, transaction, consensus mechanisms
- [ ] server: handling requests/messages from either other nodes or the 
command line
- [ ] client: bootstrap peer network, create transactions and channels
- [ ] storage manager: store relevant information (old blocks, keys, peer 
tables) persistently


## Crypto: Everything related to cryptography
- [ ] hashing functions 
- [ ] the merkle tree implementation
- [ ] generation of PGP keys
- [ ] signature and verification
- [ ] encryption and decryption of messages

## P2P: Overlay P2P network in the style of [Kademila](https://sarwiki.informatik.hu-berlin.de/Kademlia)
- [ ] Messages & Codec related to networking
- [ ] Server: accepting incoming connections, answering peer discovery queries
- [ ] Client: connecting to peers, querying for peers
- [ ] Session: keep track of active connections and peer tables 

## Storage: Store relevant information that shall not reside in RAM
- [ ] Storage trait
- [ ] working redis backend
- [ ] proper schemas for PGP keys, peer tables, blocks



## Ressourcen:
[Overall Example](https://github.com/witnet/witnet-rust/)

### Node
[Witnet node](https://github.com/witnet/witnet-rust/tree/master/node/src)  

### Crypto
[Witnet crypto](https://github.com/witnet/witnet-rust/tree/master/crypto/src)  
[Web of Trust](https://de.wikipedia.org/wiki/Web_of_Trust)  
[Sequoia PGP Example](https://docs.sequoia-pgp.org/sequoia_guide/)  
Use [Ready to use Merkle Tree](https://spinresear.ch/merkle.rs/merkle/index.html)  
or else [Merkle Tree Example/Implementation](https://github.com/SpinResearch/merkle.rs/blob/master/src/merkletree.rs)  
[Sha3](https://docs.rs/sha3/0.8.2/sha3/)  

### P2P:  
[Blog on Kademila for decentralized plattforms](https://medium.com/coinmonks/a-brief-overview-of-kademlia-and-its-use-in-various-decentralized-platforms-da08a7f72b8f)  
[Paper](pdos.csail.mit.edu/~petar/papers/maymounkov-kademlia-lncs.pdf)  
[Actix P2P example](https://github.com/mariocao/actix-p2p/tree/master/src)  

### Storage:
[Witnet storage](https://github.com/witnet/witnet-rust/tree/master/storage/src)  
[Redis Rust driver](https://docs.rs/redis/0.11.0-beta.1/redis/)  


