fn main() {
    let nums = vec![-1,1,0,-3,3];

    println!("{:?}", product_except_self(nums));
}

fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    let len = nums.len();

    let mut left_product = 1;
    let mut left_products = vec![1;len];
    let mut right_product = 1;
    let mut right_products = vec![1;len];
    

    for i in 1..len {
        left_product *= nums[i - 1];
        left_products[i] = left_product;

        right_product *= nums[len - i];
        right_products[len - i - 1] = right_product;
    }

    for i in 0..len {
        result.push(left_products[i] * right_products[i]);
    }

    result
}