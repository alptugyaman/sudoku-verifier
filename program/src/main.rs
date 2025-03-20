//! Program that checks the validity of a sudoku board and solution.
//! If the board and solution are correct, it generates a ZK proof.

// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);

use alloy_sol_types::SolType;
use sudoku_lib::{verify_sudoku, PublicValuesStruct, SUDOKU_SIZE};

pub fn main() {
    // Read the puzzle and solution
    let board = sp1_zkvm::io::read::<[[u8; SUDOKU_SIZE]; SUDOKU_SIZE]>();
    let solution = sp1_zkvm::io::read::<[[u8; SUDOKU_SIZE]; SUDOKU_SIZE]>();

    // Perform Sudoku validation
    let is_valid = verify_sudoku(board, solution);

    // Set the validation result as the public value
    let bytes = PublicValuesStruct::abi_encode(&PublicValuesStruct { is_valid });

    // Commit to the public values
    sp1_zkvm::io::commit_slice(&bytes);
}
