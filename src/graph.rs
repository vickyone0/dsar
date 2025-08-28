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



////////////////// rotten orange probelm
pub fn oranges_rotting(mut grid:Vec<Vec<i32>>) -> i32{
    let rows = grid.len();
    let cols = grid[0].len();
    let mut fresh_count = 0;
    let mut current = Vec::new();

    //collect all rotten organges
    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c]==2 {
                current.push((r,c));
            } else if grid[r][c] ==1 {
                fresh_count +=1;
            }
        }
    }

    let directions = [(1,0),(-1,0),(0,1),(0,-1)];
    let mut minutes = 0;

    //BFS level by level

    while !current.is_empty() {
        let mut next = Vec::new();
        for &(r,c) in &current {
            for (dr, dc) in &directions {
                let nr = r as isize + dr;
                let nc = c as isize + dc;

                if nr >= 0 && nc >=0 && (nr as usize) < rows && (nc as usize) <cols {

                    let nr_u = nr as usize;
                    let nc_u = nc as usize;

                    if grid[nr_u][nc_u] == 1{
                        grid[nr_u][nc_u] = 2;
                        fresh_count -=1;
                        next.push((nr_u,nc_u));
                    }
                }
            }
        } 
        if !next.is_empty() {
            minutes +=1;
        }

        current = next;
        
    }

    if fresh_count ==0 {minutes} else {
        -1
    }
}