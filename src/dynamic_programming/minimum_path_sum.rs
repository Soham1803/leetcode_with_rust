use std::{cmp, vec};


pub fn minimum_path_sum_fn(){

    let grid = vec![vec![1,3,1], vec![1,5,1], vec![4,2,1]];
    println!("The minimum path sum is: {}.", min_path_sum(& grid));
}

fn min_path_sum(grid: & Vec<Vec<i32>>) -> i32 {

    let m = grid.len();
    let n = grid[0].len();

    let mut dp = vec![vec![0; n+1]; m+1];

    for i in 0..=(grid.len()) {
        dp[i][m] = i32::MAX;
    }

    for i in 0..3 {
        for j in 0..3 {

            let p = 2 - i;
            let q = 2 - j;
            dp[p][q] = cmp::min(dp[p+1][q], dp[p][q+1]) + grid[p][q];
        }
    }

    return dp[0][0];
}