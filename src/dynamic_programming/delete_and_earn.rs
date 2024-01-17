use std::collections::HashMap;
use std::cmp;


pub fn delete_and_earn_fn() {
    println!("delete_and_earn");
    let nums = vec![2,2,3,3,3,4];
    let result = delete_and_earn(nums);
    println!("result:{}", result);
}

fn delete_and_earn(nums: Vec<i32>) -> i32 {
        
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut new_nums: Vec<i32> = Vec::new();

    for i in nums {
        if let Some(value) = map.get_mut(&i) {
            *value += 1;
        } else {
            map.insert(i, 1);
            new_nums.push(i);
        }
    }

    new_nums.sort();

    let new_im_nums = &new_nums;

    let (mut earn1, mut earn2) = (0, new_im_nums[0] * map.get(&new_im_nums[0]).unwrap());

    for i in 1..new_im_nums.len() {

        let multiplier = map.get(&new_im_nums[i]).unwrap_or(&1);

        if new_im_nums[i-1] != new_im_nums[i]-1 {

            let temp = cmp::max(new_im_nums[i]* multiplier + earn2, earn2);
            earn1 = earn2;
            earn2 = temp;
        } else {
            let temp = cmp::max(new_im_nums[i]* multiplier + earn1, earn2);
            earn1 = earn2;
            earn2 = temp;
        }
    }
    
    earn2
}