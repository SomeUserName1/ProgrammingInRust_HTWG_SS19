//! # Rocksdb storage backend
//!
//! Storage backend that persists data in the file system using a RocksDB database.
use failure::Fail;
#[cfg(not(test))]
use rocksdb;
#[cfg(test)]
use rocksdb_mock as rocksdb;

use crate::storage::{Result, Storage};

/// Rocksdb backend
pub type Backend = rocksdb::DB;

#[derive(Debug, Fail)]
#[fail(display = "RocksDB error")]
struct Error(#[fail(cause)] rocksdb::Error);

impl Storage for Backend {
    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>> {
        let result = Backend::get(self, &key)
            .map(|opt| opt.map(|dbvec| dbvec.to_vec()))
            .map_err(Error)?;
        Ok(result)
    }

    fn put(&mut self, key: Vec<u8>, value: Vec<u8>) -> Result<()> {
        Backend::put(self, key, value).map_err(Error)?;
        Ok(())
    }

    fn delete(&mut self, key: &[u8]) -> Result<()> {
        Backend::delete(self, &key).map_err(Error)?;
        Ok(())
    }
}
