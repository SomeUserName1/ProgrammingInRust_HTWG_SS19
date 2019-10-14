use std::fmt::Write;

use sha3::{Sha3_512, Digest};
use serde;

pub fn hash<T: serde::Serialize>(item: &T) -> String {

    let input = serde_json::to_string(&item).unwrap();
    let mut hasher = Sha3_512::new();
    hasher.input(input.as_bytes());
    let res = hasher.result();
    let vec_res = res.to_vec();

    hex_to_string(vec_res.as_slice())
}

pub fn hex_to_string(vec_res: &[u8]) -> String {
    let mut s = String::new();
    for b in vec_res {
        write!(&mut s, "{:x}", b).expect("unable to write");
    }
    s
}
