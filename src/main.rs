fn main() {
    let mut nums1: Vec<i32> = vec![1, 2, 3, 0, 0, 0];
    let m = 3;
    let mut nums2: Vec<i32> = vec![2, 5, 6];
    let n = 3;

    println!("{:?}", merge(&mut nums1, m, &mut nums2, n));

}

fn merge<'a>(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) -> Vec<i32> {
    for i in 0..n {
        nums1[m as usize + i as usize] = nums2[i as usize]
    }
    nums1.sort();

    nums1.to_vec()
}