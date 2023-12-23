pub fn longest_increasing_subsequence_fn(){

    let nums = vec![10,9,2,5,3,7,101,18];
    println!("The length of longest increasing subsequence is: {}.", length_of_lis(nums));
}


pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    nums.into_iter().fold(Vec::new(), |mut stk, n| {
        let i = stk.binary_search(&n).unwrap_or_else(|x| x);
        if i == stk.len() {
            stk.push(n);
        } else {
            stk[i] = n;
        }
        
        stk
    }).len() as i32
}
