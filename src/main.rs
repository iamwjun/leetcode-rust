fn main() {
    let intervals: Vec<Vec<i32>> = vec![vec![2,3],vec![4,5],vec![6,7],vec![8,9],vec![1,10]];

    println!("{:?}", merge(intervals));
}

fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut vec: Vec<Vec<i32>> = vec![];
    let mut curr = vec![];
    intervals.sort_unstable_by(|a, b| a[0].partial_cmp(&b[0]).unwrap());

    fn is_between(number: i32, vecs: &Vec<i32>) -> bool {
        number >= vecs[0] && number <= vecs[1]
    }

    for num in intervals {
        if curr.is_empty() {
            curr = num;
        } else {
            if is_between(num[0], &curr)
            || is_between(num[1], &curr)
            || is_between(curr[0], &num)
            || is_between(curr[0], &num) {
                curr = [num[0].min(curr[0]), num[1].max(curr[1])].to_vec()
            } else {
                vec.push(curr);
                curr = num;
            }
        }
    }
    if !curr.is_empty() {
        vec.push(curr)
    }
    vec
}

