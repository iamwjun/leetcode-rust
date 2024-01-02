fn main() {
    let s = String::from("  hello world  ");

    println!("{}", reverse_words(s));
}

fn reverse_words(s: String) -> String {
    let words: Vec<&str> = s.split_whitespace().rev().into_iter().collect();

    words.join(" ")
}