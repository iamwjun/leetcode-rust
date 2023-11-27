use std::collections::HashMap;

fn main() {
    let nums: Vec<i32> = vec![1,2,3,1];
    let k = 3;

    println!("{:?}", contains_nearby_duplicate(nums, k));
}

fn contains_nearby_duplicate(nums: Vec<i32>, k: usize) -> bool {
    let k = k as usize;
    let mut map = HashMap::with_capacity(nums.len());

    for (index, value) in nums.iter().enumerate() {
        if let Some(prev) = map.get(value) {
            if index - prev <= k {
                return  true;
            }
            
        }
        map.insert(*value, index);
    }

    false
}
