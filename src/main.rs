fn main() {
    let s = String::from("  hello world  ");

    println!("{}", reverse_words(s));
}

fn reverse_words(s: String) -> String {
    s.split_whitespace().rev().collect::<Vec<&str>>().join(" ")
}