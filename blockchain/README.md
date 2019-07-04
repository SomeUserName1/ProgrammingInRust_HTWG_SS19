# PIR Blockchain
## Blochain
- [x] chain: data structure, transactions consensus mechanisms/block mining
- [x] block: central data structure: stores transactions in a merkle tree 
- [x] generic transactions enabling different payloads

## Node: P2P server using blockchain 
- [ ] node: handling requests/messages from either other nodes or the 
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

