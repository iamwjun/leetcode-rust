fn main() {
    let prices = vec![2,3,1,1,4];

    println!("{:?}", can_jump(prices));
    
}

fn can_jump(nums: Vec<i32>) -> bool {
    let mut curr = 0;

    while curr < nums.len() {
        curr += nums[curr] as usize
    }

    curr == nums.len()
}