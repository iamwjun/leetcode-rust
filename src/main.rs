fn main() {
    let heights = vec![11, 19, 12, 15, 14, 18, 7, 1, 8, 9];

    println!("{:?}", can_see_persons_count(heights));
}

fn can_see_persons_count(heights: Vec<i32>) -> Vec<i32> {
    let len = heights.len();
    let mut ret = vec![0; len];

    for i in 0..len {
        let mut max_height = std::i32::MIN;
        let mut greater_curr = false;
        let mut is_greater = false;

        for j in i + 1..len {
            if heights[j] < heights[i] {
                if heights[j] > max_height && (!is_greater || !greater_curr) {
                    ret[i] += 1;
                }
            } else {
                if heights[j] > max_height {
                    ret[i] += 1;
                }
                break;
            }

            if max_height >= heights[j] && !is_greater {
                is_greater = true;
            }

            max_height = max_height.max(heights[j]);

            if max_height >= heights[i] && !greater_curr {
                greater_curr = true;
            }
        }
    }

    ret
}
