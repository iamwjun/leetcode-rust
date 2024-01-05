fn main() {
    let heights = vec![5,1,2,3,10];

    println!("{:?}", can_see_persons_count(heights));
}

fn can_see_persons_count(heights: Vec<i32>) -> Vec<i32> {
    let len = heights.len();
    let mut ret = vec![0;len];

    for i in 0..len {
        let curr = heights[i];
        let mut max_height = std::i32::MIN;
        let mut is_greater = false;
        
        for j in i+1..len {
            if heights[j] < curr && !is_greater && heights[j] > max_height {
                ret[i] += 1;
            }

            if is_greater || j > i + 2 {
                max_height = max_height.max(heights[j]);
            }

            if heights[j] >= curr {
                is_greater = true;
                
                if heights[j] > max_height {
                    ret[i] += 1;
                }
            }
        }
    }

    ret
}
