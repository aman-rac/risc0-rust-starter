/* */
#![no_main]
#![no_std]

use risc0_zkvm_guest::{env, sha};


risc0_zkvm_guest::entry!(main);
use risc0_zkvm_core::{Digest};

use serde::{Deserialize, Serialize};

//Andrew-review
//use bitcoin::blockdata::block::BlockHeader;


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
    pub hash: Digest//String//&'const mut str
}


pub fn main() {

    let header_minus_hash: HeaderMinusHash = env::read();

    //hashing
    //let hash: &'static str = &sha::digest(&header_minus_hash).to_hex();
    
    //let hash = &(sha::digest(&header_minus_hash).to_hex()[..]);
    let hash: &[u32] = sha::digest(&header_minus_hash).as_slice();
    let hash = Digest::from_slice(hash);

    //hashing
    let header_minus_nonce = HeaderMinusNonce{
        n_version: header_minus_hash.n_version,
        hash_prev_block: header_minus_hash.hash_prev_block,
        hash_merkle_root: header_minus_hash.hash_merkle_root,
        n_time: header_minus_hash.n_time,
        n_bits: header_minus_hash.n_bits,
        hash
    };

    // Commit it to the public journal
    env::commit(&header_minus_nonce);
}
