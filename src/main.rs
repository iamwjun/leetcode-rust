fn main() {
    let nums: Vec<i32> = vec![0,1,2,4,5,7];

    println!("{:?}", summary_ranges(nums));
}

fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
    let mut vec: Vec<String> = Vec::new();
    let mut start = 0;

    for end in 1..=nums.len() {
        if end == nums.len() || nums[end] != nums[end - 1] + 1 {
            if start == end - 1 {
                vec.push(nums[start].to_string());
            } else {
                vec.push(format!("{}->{}", nums[start], nums[end - 1]));
            }
            start = end;
        }
    }
    vec
}

