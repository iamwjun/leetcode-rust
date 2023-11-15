fn main() {
    let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
    let k = 3;

    println!("{:?}", rotate(&mut nums, k));
    
}

fn rotate(nums: &mut Vec<i32>, mut k: i32) {
    while k > 0 {
        if let Some(last) = nums.pop() {
            nums.insert(0, last);
        }
        k -= 1;
    }
}