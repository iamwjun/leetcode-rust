fn main() {
    let max_heights = vec![5, 3, 4, 1, 1];

    println!("{:?}", maximum_sum_of_heights(max_heights));
}

fn maximum_sum_of_heights(max_heights: Vec<i32>) -> i64 {
    let ret = -1;

    if let Some((index, max_value)) = max_heights
        .iter()
        .enumerate()
        .max_by_key(|&(_, &value)| value)
    {
        for i in 0..index {
            println!("left {}", i);
        }

        for j in index + 1..max_heights.len() {
            println!("right {}", j);
        }
    }

    ret
}
