//! Program that checks the validity of a sudoku board and solution.
//! If the board and solution are correct, it generates a ZK proof.

// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);

use sp1_sdk::{SP1Stdin, SP1Prover, SP1Verifier};
use sudoku_lib::{PublicValuesStruct, SUDOKU_SIZE, verify_sudoku};
use alloy_sol_types::SolType;

// The entrypoint to the program.
fn main() {
    let mut stdin = SP1Stdin::new();

    // Read the puzzle and solution
    let board: [[u8; SUDOKU_SIZE]; SUDOKU_SIZE] = stdin.read();
    let solution: [[u8; SUDOKU_SIZE]; SUDOKU_SIZE] = stdin.read();

    // Perform Sudoku validation
    let is_valid = verify_sudoku(board, solution);

    // Set the validation result as the public value
    let public_values = PublicValuesStruct { is_valid };

    // Commit to the public values
    SP1Prover::commit(&public_values);
}
