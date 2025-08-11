pub fn n_queen(n:i32)-> Vec<Vec<String>> {

    let mut board = vec![-1; n as usize];

    let mut solutions = Vec::new();

    solve_n_queens(&mut board, 0, n, &mut solutions);
    solutions

}

fn solve_n_queens(board: &mut Vec<i32>, row: i32, n: i32, solutions: &mut Vec<Vec<String>>) {
    if row==n {
        let mut current_solution = Vec::new();
        for &col in board.iter(){
            let mut row_str = vec!['.'; n as usize];
            row_str[col as usize] = 'Q';
            current_solution.push(row_str.into_iter().collect());

        }
        solutions.push(current_solution);
        return;
    }

    for col in 0..n {
        if is_safe(board, row, col){
            board[row as usize] = col;
            solve_n_queens(board, row + 1, n, solutions);
        }
    }
}

fn is_safe(board: & Vec<i32>, row: i32, col: i32) -> bool {
    for i in 0..row {
        let queen_col = board[i as usize];
        if queen_col == col || (queen_col - col).abs() == (i - row).abs()
        {
            return  false;
        }
    }
    true
}