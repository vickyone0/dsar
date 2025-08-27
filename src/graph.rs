pub fn num_of_island(mut grid: Vec<Vec<char>>) -> i32 {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    fn dfs(r: isize, c: isize, grid: &mut Vec<Vec<char>>) {
        // Stop if outside grid or it's water
        if r < 0 || c < 0 || r as usize >= grid.len() || c as usize >= grid[0].len() {
            return;
        }
        if grid[r as usize][c as usize] == '0' {
            return;
        }

        // Mark this land as visited
        grid[r as usize][c as usize] = '0';

        // Visit neighbors (up, down, left, right)
        dfs(r - 1, c, grid);
        dfs(r + 1, c, grid);
        dfs(r, c - 1, grid);
        dfs(r, c + 1, grid);
    }

    // Go through every box in the map
    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == '1' {
                count += 1; // Found a new island
                dfs(r as isize, c as isize, &mut grid); // Explore the whole island
            }
        }
    }

    count
}