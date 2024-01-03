use std::cmp;

pub fn edit_distance_fn () {
    let word1 = String::from("horse");
    let word2 = String::from("ros");
    let result = min_distance(word1, word2);
    println!("Edit distance: {}", result);
}

pub fn min_distance(word1: String, word2: String) -> i32 {

    let (ver_len, hor_len) = (word1.len(), word2.len());
    
    let mut cache = vec![vec![i32::MAX; hor_len+1]; ver_len+1];

    for i in 0..=ver_len {
        cache[i][hor_len] = (ver_len - i) as i32;
    }
    for i in 0..=hor_len {
        cache[ver_len][i] = (hor_len - i) as i32;
    }

    for i in (0..ver_len).rev() {
        for j in (0..hor_len).rev() {
            if word1.chars().nth(i) == word2.chars().nth(j) {
                cache[i][j] = cache[i+1][j+1];
            } else {
                cache[i][j] = 1 + cmp::min(cache[i+1][j], cmp::min(cache[i][j+1], cache[i+1][j+1]));
            }
        }
    }

    cache[0][0]
}