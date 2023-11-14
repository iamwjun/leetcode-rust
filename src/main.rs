fn main() {
    let mut nums = vec![3,2,2,3];
    let val = 3;

    println!("{:?}", remove_element(&mut nums, val));

}

fn remove_element(nums: &mut Vec<i32>, val: i32) -> usize {
    nums.retain(|&x| x != val);
    nums.len()
}