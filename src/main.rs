use std::collections::HashMap;

fn main() {
    let nums: Vec<i32> = vec![-1,-2,-3,-4,-5];
    let target = -8;

    println!("{:?}", two_sum(nums, target));
}

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<&i32, usize> = HashMap::new();

    for (i, v) in nums.iter().enumerate() {
        let m = target - v;
        match map.get(&m) {
            Some(p) => return vec![i as i32, *p as i32],
            None => map.insert(v, i)
        };
    }

    vec![]
}
