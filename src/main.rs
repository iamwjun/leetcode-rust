fn main() {
    let mut s = String::from("abcd");

    // println!("{}", make_smallest_palindrome(s));

    let nums = vec![1];

    println!("{}", find_peak_element(nums));
}

fn find_peak_element(nums: Vec<i32>) -> i32 {
    if nums.len() < 3 {
        return *nums.iter().max().unwrap();
    }
    for i in 1..nums.len() - 1 {
        if nums[i] > nums[i - 1] && nums[i] > nums[i+1] {
            return i as i32;
        }
    }
    -1
}

fn make_smallest_palindrome(s: String) -> String {
    let s_chars: Vec<char> = s.chars().collect();
    println!("{:?}", s_chars);
    
    let mut t_chars = Vec::new();
    for char in s_chars {
        t_chars.push(char);
        t_chars.push('#');
    }

    println!("{:?}", t_chars);
    s
}


