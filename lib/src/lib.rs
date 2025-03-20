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
    if !check_board_solution_consistency(board, solution) {
        return false;
    }

    // We check if the sudoku solution is valid
    if !is_valid_solution(solution) {
        return false;
    }

    true
}

/// Checks if the values in the puzzle board are consistent with the solution
/// That is, all numbers specified in the puzzle (non-zero) must be in the same position in the solution
fn check_board_solution_consistency(board: [[u8; SUDOKU_SIZE]; SUDOKU_SIZE], solution: [[u8; SUDOKU_SIZE]; SUDOKU_SIZE]) -> bool {
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
    for row in 0..SUDOKU_SIZE {
        let mut used = [false; SUDOKU_SIZE + 1]; // For numbers 1-9
        for col in 0..SUDOKU_SIZE {
            let num = solution[row][col];
            if num == 0 || num > 9 || used[num as usize] {
                return false;
            }
            used[num as usize] = true;
        }
    }

    // Check for each column
    for col in 0..SUDOKU_SIZE {
        let mut used = [false; SUDOKU_SIZE + 1]; // For numbers 1-9
        for row in 0..SUDOKU_SIZE {
            let num = solution[row][col];
            if num == 0 || num > 9 || used[num as usize] {
                return false;
            }
            used[num as usize] = true;
        }
    }

    // Check for 3x3 sub-grids
    for box_row in 0..GRID_SIZE {
        for box_col in 0..GRID_SIZE {
            let mut used = [false; SUDOKU_SIZE + 1]; // For numbers 1-9
            for row in 0..GRID_SIZE {
                for col in 0..GRID_SIZE {
                    let num = solution[box_row * GRID_SIZE + row][box_col * GRID_SIZE + col];
                    if num == 0 || num > 9 || used[num as usize] {
                        return false;
                    }
                    used[num as usize] = true;
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
