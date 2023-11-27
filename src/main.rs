use std::collections::HashMap;

fn main() {
    let nums: Vec<i32> = vec![1,2,3,1];
    let k = 3;

    println!("{:?}", contains_nearby_duplicate(nums, k));
}

fn contains_nearby_duplicate(nums: Vec<i32>, k: usize) -> bool {
    let mut map = HashMap::new();

    for (i, v) in nums.iter().enumerate() {
        match map.get(v) {
            Some(p)  => {
                if i - p <= k {
                    return  true;
                } else {
                    map.insert(v, i);
                }
            },
            None => {
                map.insert(v, i);
            }
        }
    }
    false
}
