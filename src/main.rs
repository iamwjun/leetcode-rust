fn main() {
    let nums: Vec<i32> = vec![5, 10, 1, 5, 2];
    let k = 1;

    println!("{:?}", sum_indices_with_k_set_bits(nums, k));
}

fn sum_indices_with_k_set_bits(nums: Vec<i32>, k: i32) -> i32 {
    nums.iter().enumerate().fold(0, |acc, (i, &num)| {
        acc + if i.count_ones() == k as u32 { num } else { 0 }
    })
}
