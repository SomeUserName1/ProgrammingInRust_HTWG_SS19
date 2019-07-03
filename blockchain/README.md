# PIR Blockchain
## Blochain
- [ ] chain: data structure, transactions consensus mechanisms/block mining
- [ ] block: central data structure: stores transactions in a merkle tree 
- [ ] generic transactions enabling different payloads

## Node: P2P server using blockchain 
- [ ] node: handling requests/messages from either other nodes or the 
command line, accepting incoming connections, answering peer discovery queries, 
bootstrap peer network, create transactions and broadcast transactions and mined blocks
- [ ] Messages & Codec related to networking
- [ ] Optional (storage manager: store relevant information (old blocks, keys, peer 
tables) persistently)

## Crypto: Everything related to cryptography
- [ ] hashing functions 
- [ ] the merkle tree implementation
- [ ] Optional (generation of PGP keys)
- [ ] Optional (signature and verification)
- [ ] Optional (encryption and decryption of messages)

## Optional Storage: Store relevant information that shall not reside in RAM
- [ ] Optional Storage trait
- [ ] Optional working redis backend
- [ ] Optional proper schemas for PGP keys, peer tables, blocks

