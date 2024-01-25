fn main() {
    let max_heights: Vec<i32> = vec![5,2,4,4];

    println!("{:?}", maximum_sum_of_heights(max_heights));
}

fn maximum_sum_of_heights(max_heights: Vec<i32>) -> i64 {
    let mut maxinum: i64 = 0;
    let len = max_heights.len();

    for index in 0..len {
        let mut ret: i64 = max_heights[index] as i64;
        let mut pre = max_heights[index];

        for i in (0..index).rev() {
            if max_heights[i] > pre  {
                ret += pre as i64;
            } else {
                ret += max_heights[i] as i64;
                pre = max_heights[i];
            }
        }

        pre = max_heights[index];
        for j in index + 1..len {
            if max_heights[j] > pre  {
                ret += pre as i64;
            } else {
                ret += max_heights[j] as i64;
                pre = max_heights[j];
            }
        }

        maxinum = maxinum.max(ret);
    }


    maxinum
}
