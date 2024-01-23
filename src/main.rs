fn main() {
    let nums = vec![25, 9, 7];

    println!("alternating_subarray {:?}", alternating_subarray(nums));
}

fn alternating_subarray(nums: Vec<i32>) -> i32 {
    let mut ret = -1;

    for (i, m) in nums.iter().enumerate() {
        for (j, n) in nums.iter().skip(i + 1).enumerate() {
            let length = j + 2;
            if n - m == (length - 1) as i32 % 2 {
                ret = ret.max(length as i32);
            } else {
                break;
            }
        }
    }

    ret
}
