//! # HashMap storage backend
//!
//! Storage backend that keeps data in a heap-allocated HashMap.
use std::collections::HashMap;
use super::storage::{Result, Storage};

/// HashMap backend
pub type Backend = HashMap<Vec<u8>, Vec<u8>>;

impl Storage for Backend {
    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>> {
        Ok(Backend::get(self, key).map(|slice| slice.to_vec()))
    }

    fn put(&mut self, key: Vec<u8>, value: Vec<u8>) -> Result<()> {
        Backend::insert(self, key, value);
        Ok(())
    }

    fn delete(&mut self, key: &[u8]) -> Result<()> {
        Backend::remove(self, key);
        Ok(())
    }
}
