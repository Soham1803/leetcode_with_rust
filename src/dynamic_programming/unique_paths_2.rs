use std::vec;

 
pub fn unique_paths_2_fn () {
    // let obstacle_grid = vec![
    //     vec![0, 0, 0],
    //     vec![0, 1, 0],
    //     vec![0, 0, 0],
    // ];

    let obstacle_grid = vec![
        vec![0, 0],
        vec![1, 1],
        vec![0, 0]
    ];

    let result = unique_paths_with_obstacles(& obstacle_grid);
    println!("Unique paths with obstacles: {}", result);
}

fn unique_paths_with_obstacles(obstacle_grid: & Vec<Vec<i32>>) -> i32 {

    let (m, n) = (obstacle_grid.len(), obstacle_grid[0].len());
    
    let mut dp: Vec<Vec<i32>> = vec![vec![0; n]; m];
    dp[0][0] = 1;

    for i in 0..m {
        for j in 0..n {

            if i == 0 && j == 0 {
                continue;
            }
            
            if obstacle_grid[i][j] == 1 {
                dp[i][j] = 0;
                continue;
            }
            if i == 0  {
                dp[i][j] = dp[i][j-1];
            }
            else if j == 0 {
                dp[i][j] = dp[i-1][j];
            } else {
                dp[i][j] = dp[i-1][j] + dp[i][j-1];
            }
        }
    }
    return dp[m-1][n-1];
}