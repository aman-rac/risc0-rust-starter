use std::env;

use methods::{HASH_ID, HASH_PATH, HeaderMinusHash, HeaderMinusNonce};
use risc0_zkvm_host::{Prover, Receipt, Result};
use risc0_zkvm_serde::{from_slice, to_slice, to_vec};

use log::LevelFilter;
use risc0_zkvm_core:: Digest;


fn serde() {
    struct Logger;
    impl risc0_zkvm_core::Log for Logger {
        fn log(&self, msg: &str) {
            log::info!("{}", msg);
        }
    }
    static LOGGER: Logger = Logger;
    risc0_zkvm_core::set_logger(&LOGGER);
}


fn main(){
    let args: Vec<String> = env::args().collect();

    //let example_name = &args[1];

    //if (example_name == "BITCOIN-MINING-HASH"){

    // ***************** Prover/Host  Section **************************   
        let n_version: u32 = 545259520;
        // let hash_prev_block: &str  = "0x489cd5dbc708c7e541de4d7cd91ce6d0f1613573b7fc5b40d3942ccb9555cf35";
        // let hash_merkle_root: &str  =  "0x18138372fad4b94533cd4881f03dc6c69296dd897234e0cee83f727e2e6b1f63";
        let hash_prev_block: Digest  =  Digest::from_str(&"000000000000000000002d967c209a56b0bf12a1bc57c789e3492b2591acbacf");
        let hash_merkle_root: Digest  =   Digest::from_str(&"e4885a2f0b60edcd1a997e62bf8390ac633b1ae346b83339eca51e34368a833e");

        let n_time: u32 = 1657558254;
        let n_bits: u32  =  386508719; //u32::from_str_radix("9e8dbb5c",16).unwrap();
        let n_nonce: u32  = 2660088668; //u32::from_str_radix("1709a7af",16).unwrap();


        let header_minus_hash = HeaderMinusHash{
            n_version,
            hash_prev_block:  hash_prev_block,
            hash_merkle_root: hash_merkle_root,
            n_time,
            n_bits,
            n_nonce
        };

        let mut prover = Prover::new(&std::fs::read(HASH_PATH).unwrap(), HASH_ID).unwrap();
        prover.add_input(to_vec(&header_minus_hash).unwrap().as_slice()).unwrap();
        

        /*let receipt = prover.run().unwrap();

        let c = from_slice(&receipt.get_journal_vec().unwrap());

        let header:HeaderMinusNonce = c.as_ref().unwrap();*/

        /*let receipt = prover.run().unwrap();
        let result : HeaderMinusNonce = (from_slice(receipt.get_journal_vec().unwrap())).unwrap();
        let result = vec.unwrap();*/

        /*let receipt = prover.run().unwrap();
        
        let vec = &receipt.get_journal_vec();
        let vec_int =  vec.as_mut().unwrap();
        let intermediate = vec_int.as_slice();
        let result = from_slice::<HeaderMinusHash>(intermediate).unwrap();
        */

        /*let receipt = prover.run().unwrap();
        let msg: &Result<Vec<u32>>  = &(receipt.get_journal_vec());
        let int_msg = msg.clone();
        let result: HeaderMinusNonce = from_slice(<Vec<u32> as AsRef<[u32]>>::as_ref(&int_msg.unwrap())).unwrap();
        */

        /*let receipt = prover.run().unwrap();
        let _ = print_header(receipt);*/

        let receipt = prover.run().unwrap();
        let result: HeaderMinusNonce = from_slice(&receipt.get_journal_vec().unwrap()).unwrap();

        println!("The hash of the block is {:?} and I have a valid nonce for it!", result);

        //return result;

    // ***************** Prover/Host  Section **************************   
        receipt.verify(HASH_ID).unwrap();
        println!("receipt passes verification");
    /*}

    else{
        
        // Pick two numbers
        let a: u64 = 17;
        let b: u64 = 23;

        // Multiply them inside the ZKP
        // First, we make the prover, loading the 'multiply' method
        let mut prover = Prover::new(&std::fs::read(MULTIPLY_PATH).unwrap(), MULTIPLY_ID).unwrap();
        // Next we send a & b to the guest
        prover.add_input(to_vec(&a).unwrap().as_slice()).unwrap();
        prover.add_input(to_vec(&b).unwrap().as_slice()).unwrap();
        // Run prover & generate receipt
        let receipt = prover.run().unwrap();

        // Extract journal of receipt (i.e. output c, where c = a * b)
        let c: u64 = from_slice(&receipt.get_journal_vec().unwrap()).unwrap();

        // Print an assertion
        println!("I know the factors of {}, and I can prove it!", c);

        // Here is where one would send 'receipt' over the network...

        // Verify receipt, panic if it's wrong
        receipt.verify(MULTIPLY_ID).unwrap();
    }
    */
}

/*
pub fn print_header(receipt: Receipt) -> Result<HeaderMinusNonce> {
    let msg = ((&receipt).get_journal_vec()?).clone();
    let header = from_slice(msg.as_slice()).unwrap();
    println!("The hash of the block is {:?} and I have a valid nonce for it!", header);

    Ok(header)
}*/