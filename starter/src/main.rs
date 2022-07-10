use std::env;

use methods::{HASH_ID, HASH_PATH, HeaderMinusHash, HeaderMinusNonce};
use risc0_zkvm_host::Prover;
use risc0_zkvm_serde::{from_slice, to_vec};

fn main() {
    let args: Vec<String> = env::args().collect();

    let example_name = &args[1];

    //if (example_name == "BITCOIN-MINING-HASH"){

    // ***************** Prover/Host  Section **************************   
        let n_version: u32 = 3;
        let hash_prev_block: str  = "0x489cd5dbc708c7e541de4d7cd91ce6d0f1613573b7fc5b40d3942ccb9555cf35";
        let hash_merkle_root: str  =  "0x18138372fad4b94533cd4881f03dc6c69296dd897234e0cee83f727e2e6b1f63";
        let n_time: u32 = env::read();
        let n_bits: u32  =  u32::from_str_radix("0x8372fa1813",16).unwrap();
        let n_nonce: u32  =  u32::from_str_radix("0x5dbc708489cd",16).unwrap();


        let header_minus_hash = HeaderMinusHash{
            n_version,
            hash_prev_block,
            hash_merkle_root,
            n_time,
            n_bits,
            n_nonce
        };

        let mut prover = Prover::new(&std::fs::read(HASH_PATH).unwrap(), HASH_ID).unwrap();
        prover.add_input(to_vec(&header_minus_hash).unwrap().as_slice()).unwrap();
        
        let receipt = prover.run().unwrap();

        let c: HeaderMinusNonce = from_slice(&receipt.get_journal_vec().unwrap()).unwrap();
        
        println!("The hash of the block is {:?} and I have a valid nonce for it!", Obj.hash);



    // ***************** Prover/Host  Section **************************   
        receipt.verify(HASH_ID).unwrap();
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
