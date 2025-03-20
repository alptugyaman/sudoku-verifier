use alloy_sol_types::sol;

// Sudoku size - for 9x9 sudoku
pub const SUDOKU_SIZE: usize = 9;
pub const GRID_SIZE: usize = 3;

sol! {
    /// The public values encoded as a struct that can be easily deserialized inside Solidity.
    struct PublicValuesStruct {
        bool is_valid;
    }
}

/// Checks if the Sudoku board and solution are correct
pub fn verify_sudoku(board: [[u8; SUDOKU_SIZE]; SUDOKU_SIZE], solution: [[u8; SUDOKU_SIZE]; SUDOKU_SIZE]) -> bool {
    // First, we check if the board and solution are consistent with each other
    if !is_consistent(board, solution) {
        return false;
    }

    // We check if the sudoku solution is valid
    is_valid_solution(solution)
}

/// Checks if the values in the puzzle board are consistent with the solution
/// That is, all numbers specified in the puzzle (non-zero) must be in the same position in the solution
fn is_consistent(board: [[u8; SUDOKU_SIZE]; SUDOKU_SIZE], solution: [[u8; SUDOKU_SIZE]; SUDOKU_SIZE]) -> bool {
    for i in 0..SUDOKU_SIZE {
        for j in 0..SUDOKU_SIZE {
            // If there is a number in the puzzle (not 0), it should match the same position in the solution
            if board[i][j] != 0 && board[i][j] != solution[i][j] {
                return false;
            }
        }
    }
    true
}

/// Checks if a Sudoku solution is valid
fn is_valid_solution(solution: [[u8; SUDOKU_SIZE]; SUDOKU_SIZE]) -> bool {
    // Check for each row
    for i in 0..SUDOKU_SIZE {
        let mut used = [false; SUDOKU_SIZE + 1]; // For numbers 1-9
        for j in 0..SUDOKU_SIZE {
            let num = solution[i][j] as usize;
            if num == 0 || used[num] {
                return false;
            }
            used[num] = true;
        }
    }

    // Check for each column
    for j in 0..SUDOKU_SIZE {
        let mut used = [false; SUDOKU_SIZE + 1]; // For numbers 1-9
        for i in 0..SUDOKU_SIZE {
            let num = solution[i][j] as usize;
            if num == 0 || used[num] {
                return false;
            }
            used[num] = true;
        }
    }

    // Check for 3x3 sub-grids
    for box_i in 0..3 {
        for box_j in 0..3 {
            let mut used = [false; SUDOKU_SIZE + 1]; // For numbers 1-9
            for i in 0..3 {
                for j in 0..3 {
                    let num = solution[box_i * 3 + i][box_j * 3 + j] as usize;
                    if num == 0 || used[num] {
                        return false;
                    }
                    used[num] = true;
                }
            }
        }
    }

    true
}

/// Compute the n'th fibonacci number (wrapping around on overflows), using normal Rust code.
pub fn fibonacci(n: u32) -> (u32, u32) {
    let mut a = 0u32;
    let mut b = 1u32;
    for _ in 0..n {
        let c = a.wrapping_add(b);
        a = b;
        b = c;
    }
    (a, b)
}
