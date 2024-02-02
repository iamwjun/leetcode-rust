use std::collections::HashMap;

fn main() {
    let nums = vec![2,1,3,3,2];

    println!("{:?}", minimum_seconds(nums));
}

fn minimum_seconds(nums: Vec<i32>) -> i32 {
    let mut map: HashMap<&i32, i32> = HashMap::new();

    for num in nums.iter() {
        *map.entry(num).or_insert(0) += 1;
    }

    (map.len() - 1) as i32
}
