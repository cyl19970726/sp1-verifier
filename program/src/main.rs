//! A simple program to be proven inside the zkVM.

#![no_main]
sp1_zkvm::entrypoint!(main);
// use serde_json::Value; // Generic JSON.
use sp1_core::{SP1ProofWithIO, SP1Verifier};
use sp1_core::utils::BabyBearBlake3;
const ELF: &[u8] = include_bytes!("../../pre_data/riscv32im-succinct-zkvm-elf");
pub fn main() {
       // read generic JSON example inputs.
       let proof_ptr = sp1_zkvm::io::read::<SP1ProofWithIO<BabyBearBlake3>>();
   
        SP1Verifier::verify(ELF, &proof_ptr).expect("verification failed");
}

