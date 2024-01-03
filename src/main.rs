fn main() {
    let nums = vec![1, 2, 3, 4];

    println!("{:?}", product_except_self(nums));
}

fn product_except_self(mut nums: Vec<i32>) -> Vec<i32> {
    let product = nums.iter().fold(1, |acc, e| acc * e);

    for num in &mut nums {
        *num = product / *num;
    }

    nums
}