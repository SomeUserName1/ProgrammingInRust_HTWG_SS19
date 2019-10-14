# PIR Blockchain

## Description: 
Our goal was to provide a asynchronous-networking-cabaple decentralized blockchain, that is able to be set up with different (generic) Transaction payloads, which would have been signed with pgp and the traffic would have been encrypted by pgp. The keys as well as peer tables and the elder blocks should have been stored persistently to reduce the ram usage.  

## Blockchain
- [x] chain: data structure, transactions consensus mechanisms/block mining
- [x] block: central data structure: stores transactions in a merkle tree 
- [x] generic transactions enabling different payloads

## Node: P2P server using blockchain 
- [x] node: handling requests/messages from either other nodes or the 
command line, accepting incoming connections, answering peer discovery queries, 
bootstrap peer network, create transactions and broadcast transactions and mined blocks
- [x] Messages & Codec related to networking
- [ ] Optional (storage manager: store relevant information (old blocks, keys, peer 
tables) persistently)

## Crypto: Everything related to cryptography
- [x] hashing functions 
- [x] the merkle tree implementation
- [x] (generation of PGP keys)
- [ ] Optional (signature and verification)
- [ ] Optional (encryption and decryption of messages)

## Optional Storage: Store relevant information that shall not reside in RAM
- [x] Storage trait
- [ ] Optional working rocksdb backend
- [ ] Optional proper schemas for PGP keys, peer tables, blocks

## Things not considered
- A Wallet for managing and reestablishing possibly differen accounts on a node  
- scalability & security  
- channel based networking as in the lightning protocol used by bitcoin
- proper routing to not spawn a channel for each client in the network as in the modified kademila protocol used by Ethereum 2.0  
- Further Consensus mechanisms

## Dependencies
```
serde = { version = "1.0.94", features = ["derive"]}
serde_json = "1.0.39"
serde_derive = "1.0.94"
```  
Derivable serialization traits used for messages and structs to be used in networking.  
```
bytes = "0.4"
time = "0.1"
getopts = "0.2"
failure = "0.1.5"
```  
Used in error handling, timing, reading and writing to buffers (networking codec) and parsing cli arguments.  
```
sequoia-openpgp = "0.8" # see bottom note
sha3 = "0.8.2"
```
Obviously used for the crypto module for hashing, the merkle tree, signing and encryption.  

```
tokio = "0.1"
tokio-io = "0.1.12"
tokio-codec = "0.1.1"
tokio-core = "0.1"
futures = "0.1"
tokio-timer = "0.1"
uuid = { version = "0.7", features = ["serde", "v4"] }
```  
Used to write the p2p server so that it doesnt block the node e.g. when mining (no messages are receivable anymore when doing it synchronously).  

```
rocksdb = "0.10"
```
Database for persistent storage of peer tables, pgp keys and blockchain data structures when not mining.  
