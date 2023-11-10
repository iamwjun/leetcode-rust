fn main() {
    let nums1: Vec<i32> = vec![1, 1, 2];
    let nums2: Vec<i32> = vec![2, 4];

    let (mut left, mut right) = (0, 0);
    let max_len = std::cmp::max(nums1.len(), nums2.len());
    let mut res = -1;

    while left < max_len {
        match (nums1.get(left), nums2.get(right)) {
            (Some(v1), Some(v2)) => {
                if v1 == v2 {
                    res = *v1;
                    break;
                } else if v1 < v2 {
                    left += 1;
                } else {
                    right += 1;
                }
            },
            (Some(_v1), None) => break,
            (None, Some(_v2)) => break,
            (None, None) => break
        }
    }

    println!("{:?}", res)
}
