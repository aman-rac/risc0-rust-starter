/* */
#![no_main]
#![no_std]

include!(concat!(env!("OUT_DIR"), "/methods.rs"));

use serde::{Deserialize, Serialize};
use risc0_zkvm_core::Digest;


#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct HeaderMinusHash{
    pub n_version: i32,
    pub hash_prev_block: Digest,
    pub hash_merkle_root: Digest,
    pub n_time: u32,
    pub n_bits: u32,
    pub n_nonce: u32
}


#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct HeaderMinusNonce{
    pub n_version: i32,
    pub hash_prev_block: Digest,
    pub hash_merkle_root: Digest,
    pub n_time: u32,
    pub n_bits: u32,
    pub hash: Digest //String//&'const mut str
}

