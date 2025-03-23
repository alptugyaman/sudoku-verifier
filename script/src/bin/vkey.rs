//! Command to generate verification key for the Sudoku verifier program
//!
//! Usage:
//! ```bash
//! cargo run -p sudoku-script --bin vkey
//! ```

use sp1_sdk::{include_elf, HashableKey, Prover, ProverClient};

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const SUDOKU_ELF: &[u8] = include_elf!("sudoku-program");

fn main() {
    let prover = ProverClient::builder().cpu().build();
    let (_, vk) = prover.setup(SUDOKU_ELF);
    println!("{}", vk.bytes32());
}
