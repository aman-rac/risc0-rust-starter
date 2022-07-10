/* */
#![no_main]
#![no_std]

use risc0_zkvm_guest::{env, sha};
//extern crate alloc;
//use alloc::{string::String};

use serde::{Deserialize, Serialize};

risc0_zkvm_guest::entry!(main);

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HeaderMinusHash{
    pub n_version: u32,
    pub hash_prev_block: &'static str,
    pub hash_merkle_root: &'static str,
    pub n_time: u32,
    pub n_bits: u32,
    pub n_nonce: u32
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HeaderMinusNonce<'a>{
    pub n_version: u32,
    pub hash_prev_block: &'static str,
    pub hash_merkle_root: &'static str,
    pub n_time: u32,
    pub n_bits: u32,
    pub hash: &'a str//String//&'const mut str
}

pub fn init() {

    let header_minus_hash: HeaderMinusHash = env::read();

    //hashing
    //let hash: &'static str = &sha::digest(&header_minus_hash).to_hex();

    let header_minus_nonce = HeaderMinusNonce{
        n_version: header_minus_hash.n_version,
        hash_prev_block: header_minus_hash.hash_prev_block,
        hash_merkle_root: header_minus_hash.hash_merkle_root,
        n_time: header_minus_hash.n_time,
        n_bits: header_minus_hash.n_bits,
        hash: &sha::digest(&header_minus_hash).to_hex()[..]
    };

    // Commit it to the public journal
    env::commit(&header_minus_nonce);
}
