fn main() {
    let nums = vec![2,2,1,1,1,2,2];

    println!("{:?}", majority_element(nums));
}

fn majority_element(mut nums: Vec<i32>) -> i32 {
    nums.sort();
    nums[nums.len() / 2]
}