fn main() {
    let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
    let k = 3;

    println!("{:?}", rotate(&mut nums, k));
    
}

fn rotate(nums: &mut Vec<i32>, k: i32) {
    let m = k as usize % nums.len();
    nums.rotate_right(m)
}