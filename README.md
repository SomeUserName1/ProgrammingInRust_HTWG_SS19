# Rust Crate

Our plan is to write our own blockchain purely in rust. For this, we will make use of rusts concurrency features to mine new 
blocks. We want these blocks to be able to contain generic data or transactions, signed with pgp signatures. The distribution
of the pgp public key may, or may not be included in our project. Also it is our ambition to the decentralize this blockchain
for the sake of maturity and usability. But because we are fairly new to this topic, we are not certain about the amount of 
work which is required to accomplish these goals. But if it is feasible for us, we will decentralize the blockchain to be 
usable over LAN or WAN. Therefor maybe we will make use of tokyo's io functionalities.

# Depedencies
* **time:**  
  &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Utilities for working with time-related functions in Rust.
* **serde:**  
  &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Serde is a framework for serializing and deserializing Rust data structures efficiently 
  and  
  &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;generically
* **serde_derive:**  
  &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Macros 1.1 implementation of ```#[derive(Serialize, Deserialize)]```
* **serde_json:**  
  &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;A JSON serialization file format
* **crypto-hash:**  
  &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Is a Rust wrapper around OS-level implementations of cryptographic hash functions.
* **hex:**    
  &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Encoding and decoding data into/from hexadecimal representation.
* **pgp:**  
  &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Pure rust implementation of OpenPGP. Following RFC4880 and RFC2440.
* **tokyo:**  
  &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;A runtime for writing reliable, asynchronous, and slim applications with the Rust programming language.
