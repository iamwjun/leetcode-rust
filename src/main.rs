fn main() {
    let prices = vec![3,2,1,0,4];

    println!("{:?}", can_jump(prices));
    
}

fn can_jump(nums: Vec<i32>) -> bool {
    let len = nums.len();
    let mut max_reach = 0;

    for (i, v) in nums.iter().enumerate() {
        if i > max_reach {
            return  false;
        }

        max_reach = max_reach.max(i + (*v as usize));

        if max_reach >= len - 1 {
            break;
        }
    }
    true
}