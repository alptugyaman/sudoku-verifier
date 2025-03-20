use sp1_sdk::{include_elf, HashableKey, Prover, ProverClient};

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const FIBONACCI_ELF: &[u8] = include_elf!("fibonacci-program");

//! Command to generate verification key for the Sudoku verifier program
//!
//! Usage:
//! ```bash
//! cargo run -p sudoku-script --bin vkey
//! ```

fn main() {
    let prover = ProverClient::builder().cpu().build();
    let (_, vk) = prover.setup(FIBONACCI_ELF);
    println!("{}", vk.bytes32());
}