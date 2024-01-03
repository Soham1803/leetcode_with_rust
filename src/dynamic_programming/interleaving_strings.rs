

pub fn interleaving_strings_fn () {
    let s1 = String::from("aabcc");
    let s2 = String::from("dbbca");
    let s3 = String::from("aadbbcbcac");
    let result = is_interleave(s1, s2, s3);
    println!("Interleaving strings: {}", result);
}

fn is_interleave(s1: String, s2: String, s3: String) -> bool {

    if s1.len()+s2.len() != s3.len() { return false; }
    
    return dp(&s1, &s2, &s3);
}

fn dp(
    s1: &String,
    s2: &String,
    s3: &String,
) -> bool {

    let mut dp_arr: Vec<Vec<bool>> = vec![vec![false; s1.len()+1]; s2.len()+1];
    dp_arr[s1.len()][s2.len()] = true;

    for i in (0..s1.len()+1).rev() {
        for j in (0..s2.len()+1).rev() {

            if i < s1.len() && s1.chars().nth(i) == s3.chars().nth(i+j) && dp_arr[i+1][j] {
                dp_arr[i][j] = true;
            }
            if j < s2.len() && s2.chars().nth(j) == s3.chars().nth(i+j) && dp_arr[i][j+1] {
                dp_arr[i][j] = true;
            }
        }
    }
    dp_arr[0][0]
}