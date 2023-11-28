use std::collections::HashSet;

fn main() {
    let nums: Vec<i32> = vec![9,1,4,7,3,-1,0,5,8,-1,6];

    println!("{:?}", longest_consecutive(nums));
}

fn longest_consecutive(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let nums_set: HashSet<i32> = nums.into_iter().collect();
    let mut longest = 0;

    for &num in nums_set.iter() {
        if !nums_set.contains(&(num - 1)) {
            let mut curr_num = num;
            let mut curr_length = 1;

            while nums_set.contains(&(curr_num + 1)) {
                curr_num += 1;
                curr_length += 1;
            }

            longest = longest.max(curr_length);
        }
    }

    longest
}

