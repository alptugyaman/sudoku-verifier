use alloy_sol_types::sol;

// Sudoku boyutu - 9x9 sudoku için
pub const SUDOKU_SIZE: usize = 9;
pub const GRID_SIZE: usize = 3;

sol! {
    /// The public values encoded as a struct that can be easily deserialized inside Solidity.
    struct PublicValuesStruct {
        bool is_valid;
    }
}

/// Sudoku tahtasının ve çözümünün doğru olup olmadığını kontrol eder
pub fn verify_sudoku(board: [[u8; SUDOKU_SIZE]; SUDOKU_SIZE], solution: [[u8; SUDOKU_SIZE]; SUDOKU_SIZE]) -> bool {
    // İlk olarak, board ve çözümün birbirine uygunluğunu kontrol ediyoruz
    if !check_board_solution_consistency(board, solution) {
        return false;
    }

    // Sudoku çözümünün doğruluğunu kontrol ediyoruz
    if !is_valid_solution(solution) {
        return false;
    }

    true
}

/// Bulmaca tahtasındaki değerlerin çözümle tutarlı olup olmadığını kontrol eder
/// Yani, bulmacada belirtilen tüm sayılar (0 olmayan) çözümde aynı yerde olmalıdır
fn check_board_solution_consistency(board: [[u8; SUDOKU_SIZE]; SUDOKU_SIZE], solution: [[u8; SUDOKU_SIZE]; SUDOKU_SIZE]) -> bool {
    for i in 0..SUDOKU_SIZE {
        for j in 0..SUDOKU_SIZE {
            // Eğer bulmacada bir sayı varsa (0 değilse), çözümdeki aynı konumdaki sayıyla eşleşmeli
            if board[i][j] != 0 && board[i][j] != solution[i][j] {
                return false;
            }
        }
    }
    true
}

/// Sudoku çözümünün geçerli olup olmadığını kontrol eder
fn is_valid_solution(solution: [[u8; SUDOKU_SIZE]; SUDOKU_SIZE]) -> bool {
    // Her satır için kontrol
    for row in 0..SUDOKU_SIZE {
        let mut used = [false; SUDOKU_SIZE + 1]; // 1-9 sayıları için
        for col in 0..SUDOKU_SIZE {
            let num = solution[row][col];
            if num == 0 || num > 9 || used[num as usize] {
                return false;
            }
            used[num as usize] = true;
        }
    }

    // Her sütun için kontrol
    for col in 0..SUDOKU_SIZE {
        let mut used = [false; SUDOKU_SIZE + 1]; // 1-9 sayıları için
        for row in 0..SUDOKU_SIZE {
            let num = solution[row][col];
            if num == 0 || num > 9 || used[num as usize] {
                return false;
            }
            used[num as usize] = true;
        }
    }

    // 3x3 alt kareler için kontrol
    for box_row in 0..GRID_SIZE {
        for box_col in 0..GRID_SIZE {
            let mut used = [false; SUDOKU_SIZE + 1]; // 1-9 sayıları için
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
