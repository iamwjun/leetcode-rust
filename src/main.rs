fn main() {
    let intervals: Vec<Vec<i32>> = vec![vec![2,3],vec![4,5],vec![6,7],vec![8,9],vec![1,10]];

    println!("{:?}", merge(intervals));
}

fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if intervals.is_empty() {
        return vec![];
    }
    let mut sort_intervals = intervals;
    sort_intervals.sort_unstable_by_key(|a| a[0]);

    let mut vec: Vec<Vec<i32>> = vec![];
    let mut curr_intervals = sort_intervals[0].clone();

    for interval in sort_intervals.iter().skip(1) {
        if interval[0] <= curr_intervals[1] {
            curr_intervals[1] = curr_intervals[1].max(interval[1]);
        } else {
            vec.push(curr_intervals);
            curr_intervals = interval.to_vec();
        }
    }

    vec.push(curr_intervals);
    
    vec
}

