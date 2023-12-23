use std::cmp;

pub fn triangle_fn(){

    let triangle = vec![vec![2], vec![3,4], vec![6,5,7], vec![4,1,8,3]];
    println!("The minimum path length to reach to the bottom is: {}.", minimum_total(triangle));
}

fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {

    let length = triangle.len();
    let mut dp = vec![0; length+1];
 

    for i in triangle.iter().rev(){
        for j in 0..i.len() {
            dp[j] = cmp::min(dp[j], dp[j+1]) + i[j];
        }
    }

    return dp[0];
}

// [[1,4,8,6,2,2,1,7],
// [4,7,3,1,4,5,5,1],
// [8,8,2,1,1,8,0,1],
// [8,9,2,9,8,0,8,9],
// [5,7,5,7,1,8,5,5],
// [7,0,9,4,5,6,5,6],
// [4,9,9,7,9,1,9,0]]