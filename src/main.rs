fn main() {
    let haystack = String::from("but");
    let needle = String::from("sad");

    println!("{:?}", str_str(haystack, needle));
}

fn str_str(haystack: String, needle: String) -> i32 {
    match haystack.find(&needle) {
        Some(i) => i as i32,
        None => -1
    }
}
