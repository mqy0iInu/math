#include <stdio.h>
#include <stdint.h>
#include <stdbool.h>

#define N_QUEEN     8

int board[N_QUEEN][N_QUEEN];
static uint8_t s_solution = 0;

bool is_safe(int row, int col)
{
    int i, j;

    // 左方向をチェック
    for (i = 0; i < col; i++)
        if (board[row][i])
            return false;

    // 左上方向をチェック
    for (i = row, j = col; i >= 0 && j >= 0; i--, j--)
        if (board[i][j])
            return false;

    // 左下方向をチェック
    for (i = row, j = col; j >= 0 && i < N_QUEEN; i++, j--)
        if (board[i][j])
            return false;

    return true;
}

void print_board()
{
    for (int i = 0; i < N_QUEEN; i++) {
        for (int j = 0; j < N_QUEEN; j++) {
            if (board[i][j])
                printf("Q ");
            else
                printf("- ");
        }
        printf("\n");
    }
    printf("\n");
}

bool solve_queens(int col)
{
    if (col >= N_QUEEN) {
        s_solution++;
        printf("Solution No %d:\n", s_solution);
        print_board();
        return true;
    }

    bool found_solution = false;
    for (int i = 0; i < N_QUEEN; i++) {
        if (is_safe(i, col)) {
            board[i][col] = 1;

            // 解を見つけたらフラグを立てる
            found_solution = solve_queens(col + 1) || found_solution;

            board[i][col] = 0; // バックトラック
        }
    }

    return found_solution;
}

int main()
{
    for (int i = 0; i < N_QUEEN; i++) {
        for (int j = 0; j < N_QUEEN; j++) {
            board[i][j] = 0;
        }
    }

    if (!solve_queens(0)) {
        printf("No solution found.\n");
    }

    return 0;
}