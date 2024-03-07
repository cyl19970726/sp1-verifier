//! A simple script to generate and verify the proof of a given program.

use sp1_core::{SP1Prover,SP1ProofWithIO, SP1Stdin, SP1Verifier};
use sp1_core::utils::BabyBearBlake3;
use serde_json;
// use std::fs;
const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");
fn main() {
    let file_content = include_bytes!("../../pre_data/proof-with-io.json");
    let file_content_str =
        core::str::from_utf8(file_content).expect("Failed to convert file content to string");
    let previous_proof: SP1ProofWithIO<BabyBearBlake3> =
        serde_json::from_str(file_content_str).expect("Failed to parse JSON");

    // Generate proof.
    let mut stdin = SP1Stdin::new();
    stdin.write(&previous_proof);

    let mut proof = SP1Prover::prove(ELF, stdin).expect("proving failed");
    // Verify proof.
    SP1Verifier::verify(ELF, &proof).expect("verification failed");

    // Save proof.
    proof
        .save("proof-with-io.json")
        .expect("saving proof failed");

    println!("succesfully generated and verified proof for the program!")
}
