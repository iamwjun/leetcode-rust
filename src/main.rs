fn main() {
    let nums = vec![2,2,1,1,1,2,2];

    println!("{:?}", majority_element(nums));
}

fn majority_element(nums: Vec<i32>) -> i32 {
    let mut counts = std::collections::HashMap::new();

    for n in nums {
        *counts.entry(n).or_insert(0) += 1;
    }

    let mut max_element: Option<i32> = None;
    let mut max_count = 0;

    for (&element, &count) in counts.iter() {
        if count > max_count {
            max_count = count;
            max_element = Some(element);
        }
    }

    max_element.unwrap()
}