//! Bir sudoku tahtası ve çözümünün doğruluğunu kontrol eden program.
//! Tahta ve çözüm doğruysa ZK kanıtı oluşturur.

// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);

use alloy_sol_types::SolType;
use sudoku_lib::{verify_sudoku, PublicValuesStruct, SUDOKU_SIZE};

pub fn main() {
    // Bulmaca ve çözümünü okuyalım
    let board = sp1_zkvm::io::read::<[[u8; SUDOKU_SIZE]; SUDOKU_SIZE]>();
    let solution = sp1_zkvm::io::read::<[[u8; SUDOKU_SIZE]; SUDOKU_SIZE]>();

    // Sudoku doğrulamasını yapalım
    let is_valid = verify_sudoku(board, solution);

    // Doğrulama sonucunu kamu değeri olarak belirtelim
    let bytes = PublicValuesStruct::abi_encode(&PublicValuesStruct { is_valid });

    // Kamu değerlerine commit edelim
    sp1_zkvm::io::commit_slice(&bytes);
}
