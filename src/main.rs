fn main() {
    let nums1: Vec<i32> = vec![1, 1, 2];
    let nums2: Vec<i32> = vec![2, 4];

    let (len1, len2) = (nums1.len(), nums2.len());
    let (mut left, mut right) = (0, 0);
    let mut res = nums1[left];

    while left < len1 &&  right < len2{
        if nums1[left] == nums2[right] {
            res = nums1[left]
        }
        if nums1[left] < nums2[right] {
            left += 1
        } else {
            right += 1
        }
    }

    println!("{:?}", res)
}
