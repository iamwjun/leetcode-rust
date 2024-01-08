fn main() {
    let heights = vec![11, 19, 12, 15, 14, 18, 7, 1, 8, 9];

    println!("{:?}", can_see_persons_count(heights));
}

fn can_see_persons_count(heights: Vec<i32>) -> Vec<i32> {
    let len = heights.len();
    let mut ret: Vec<i32> = vec![0; len];
    let mut stack: Vec<i32> = Vec::new();

    for (i, &h) in heights.iter().enumerate().rev() {
        while !stack.is_empty() && *stack.last().unwrap() < h {
            stack.pop();
            ret[i] += 1;
        }
        if !stack.is_empty() {
            ret[i] += 1;
        }

        stack.push(h);
    }

    ret
}
