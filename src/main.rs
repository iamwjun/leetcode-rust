fn main() {
    let mut nums = vec![3,2,2,3];

    println!("{:?}", remove_element(&mut nums));

}

fn remove_element(nums: &mut Vec<i32>) -> usize {
    let mut unique_set = std::collections::HashSet::new();
    nums.retain(|&x| unique_set.insert(x));
    nums.len()
}