use std::collections::HashSet;

fn main() {
    let nums = vec![3,2,3,4,2];

    println!("{:?}", distinct_difference_array(nums));
}

fn distinct_difference_array(nums: Vec<i32>) -> Vec<i32> {
    let len = nums.len();
    let mut ret: Vec<i32> = vec![0;len];

    for i in 0..len {
        let left: HashSet<_> = nums[0..(i + 1)].iter().cloned().collect();
        let right: HashSet<_> = nums[(i + 1)..len].iter().cloned().collect();
        ret[i] = left.len() as i32 - right.len() as i32;
    }

    ret
}
