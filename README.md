
# Rust Crate: Own Blockchain
---

## Participants:

Felix Mayer, Stephan Perren, Fabian Klopfer

## Description:

Our plan is to write our own blockchain purely in rust. For this, we will make use of rusts concurrency features to mine new 
blocks. We want these blocks to be able to contain generic data or transactions, signed with pgp signatures. The distribution
of the pgp public key may, or may not be included in our project. Also it is our ambition to the decentralize this blockchain
for the sake of maturity and usability. But because we are fairly new to this topic, we are not certain about the amount of 
work which is required to accomplish these goals. But if it is feasible for us, we will decentralize the blockchain to be 
usable over LAN or WAN. Therefor maybe we will make use of tokyo's io functionalities.

## Depedencies
* **time:**  
  &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Utilities for working with time-related functions in Rust.
* **serde:**  
  &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Serde is a framework for serializing and deserializing Rust data structures efficiently  and generically.
* **serde_derive:**  
  &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Macros 1.1 implementation of ```#[derive(Serialize, Deserialize)]```.
* **serde_json:**  
  &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;A JSON serialization file format.
* **crypto-hash:**  
  &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Is a Rust wrapper around OS-level implementations of cryptographic hash functions.
* **hex:**    
  &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Encoding and decoding data into/from hexadecimal representation.
* **pgp:**  
  &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Pure rust implementation of OpenPGP. Following RFC4880 and RFC2440.
* **tokyo:**  
  &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;A runtime for writing reliable, asynchronous, and slim applications with the Rust programming language.



# Related Work
Short Version:
(https://www.youtube.com/watch?v=U8GGZ4TqlQs)[Video]
(https://github.com/tensor-programming/Rust_block_chain)[Repo]

Long Version:
(https://www.youtube.com/playlist?list=PLwnSaD6BDfXL0RiKT_5nOIdxTxZWpPtAv)[Playlist]
(https://medium.com/geeklaunch/blockchain-in-rust-01-blocks-hashing-4192f2265d3d)[Blog]
(https://github.com/GeekLaunch/blockchain-rust/tree/bc663ec59d8a2490552e5dc7d998311acf8dd305)[Repo]

Existing Frameworks
(https://exonum.com/doc/version/latest/get-started/design-overview/)[Architecture]
(https://exonum.com/doc/version/latest/get-started/create-service/)[Example implementation using the framework]
(https://github.com/exonum/exonum)[Repo]
