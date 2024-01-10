fn main() {
    let s = String::from("leetscode");
    let dictionary: Vec<String> = vec!["leet".to_string(),"code".to_string(),"leetcode".to_string()];

    println!("{:?}", min_extra_char(s, dictionary));
}

fn min_extra_char(s: String, dictionary: Vec<String>) -> i32 {
    -1
}
