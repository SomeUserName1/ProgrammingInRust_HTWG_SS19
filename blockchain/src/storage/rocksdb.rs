//! # Rocksdb storage backend
//!
//! Storage backend that persists data in the file system using a RocksDB database.
use failure::Fail;
use rocksdb;

use super::storage::{Result, Storage};
use rocksdb::DB;

/// To be stored are:
/// crypto: nodes pk + sk, revocation certificate and uuid, pk pairs of other nodes
/// node: peer tables aka uuid, ip address pairs
/// blockchain: a copy of the blockchain, only the current transactions shall remain in ram
///
/// storage schema:
/// rocks db supports column families similar to sql tables so just create a column familiy for
/// each set of items to be saved: o
/// 1. ownKeyPairFamily containing pk, sk and the revocation certificate
/// 2. publicKeysFamily conatining Uuid, TPK pairs
/// 3. peerTableFamily: Uuid, IP pairs
/// 4. blockChainFamily: block_count, Block pairs (the chain header shall remain in ram as its
/// frequently changing
#[derive(Debug, Fail)]
#[fail(display = "RocksDB error")]
struct Error(#[fail(cause)] rocksdb::Error);

 impl Storage for DB {
    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>> {
        let result = DB::get(self, &key)
            .map(|opt| opt.map(|dbvec| dbvec.to_vec()))
            .map_err(Error)?;
        Ok(result)
    }

    fn put(&mut self, key: Vec<u8>, value: Vec<u8>) -> Result<()> {
        DB::put(self, &key, &value).map_err(Error)?;
        Ok(())
    }

    fn delete(&mut self, key: &[u8]) -> Result<()> {
        DB::delete(self, &key).map_err(Error)?;
        Ok(())
    }
}
