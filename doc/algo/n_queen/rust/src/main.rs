const N_QUEEN: usize = 8;

static mut BOARD: [[u8; N_QUEEN]; N_QUEEN] = [[0; N_QUEEN]; N_QUEEN];
static mut SOLUTION: u8 = 0;

fn is_safe(row: usize, col: usize) -> bool {
    let mut i;
    let mut j;

    // 左方向をチェック
    for i in 0..col {
        if unsafe { BOARD[row][i] } != 0 {
            return false;
        }
    }

    // 左上方向をチェック
    i = row;
    j = col;
    while i > 0 && j > 0 {
        if unsafe { BOARD[i - 1][j - 1] } != 0 {
            return false;
        }
        i -= 1;
        j -= 1;
    }

    // 左下方向をチェック
    i = row;
    j = col;
    while j > 0 && i < N_QUEEN - 1 {
        if unsafe { BOARD[i + 1][j - 1] } != 0 {
            return false;
        }
        i += 1;
        j -= 1;
    }

    true
}

fn print_board() {
    for i in 0..N_QUEEN {
        for j in 0..N_QUEEN {
            if unsafe { BOARD[i][j] } != 0 {
                print!("Q ");
            } else {
                print!("- ");
            }
        }
        println!();
    }
    println!();
}

fn solve_queens(col: usize) -> bool {
    if col >= N_QUEEN {
        unsafe {
            SOLUTION += 1;
            println!("Solution No {}: ", SOLUTION);
            print_board();
        }
        return true;
    }

    let mut found_solution = false;
    for i in 0..N_QUEEN {
        if is_safe(i, col) {
            unsafe {
                BOARD[i][col] = 1;
            }

            // 解を見つけたらフラグを立てる
            found_solution = solve_queens(col + 1) || found_solution;

            unsafe {
                BOARD[i][col] = 0; // バックトラック
            }
        }
    }

    found_solution
}

fn main() {
    unsafe {
        for i in 0..N_QUEEN {
            for j in 0..N_QUEEN {
                BOARD[i][j] = 0;
            }
        }
    }

    if !solve_queens(0) {
        println!("No solution found.");
    }
}