fn main() {
    let mut nums = vec![1, 1, 1, 2, 2, 3];

    println!("{:?}", remove_duplicates(&mut nums));
}

fn remove_duplicates(nums: &mut Vec<i32>) -> usize {
    if nums.len() <= 2 {
        return nums.len();
    }

    let mut unique_index = 2;

    for i in 2..nums.len() {
        if nums[i] != nums[unique_index - 2] {
            nums[unique_index] = nums[i];
            unique_index += 1;
        }
    }

    unique_index
}