fn main() {
    let intervals: Vec<Vec<i32>> = vec![vec![1,6], vec![8,10], vec![15,18]];

    println!("{:?}", merge(intervals));
}

fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    intervals.sort_unstable_by_key(|a| a[0]);

    let mut vec: Vec<Vec<i32>> = vec![];
    vec.push(intervals.first().unwrap().clone());

    for interval in intervals.iter().skip(1) {
        if interval[0] <= vec.last().unwrap()[1] {
            vec.last_mut().unwrap()[1] = vec.last().unwrap()[1].max(interval[1]);
        } else {
            vec.push(interval.clone());
        }
    }

    vec
}

