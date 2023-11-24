fn main() {
    let t = String::from("anagram");
    let s = String::from("nagaram");

    println!("{:?}", is_anagram(s, t));
}

fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return  false;
    }

    let mut chars_s: Vec<char> = s.chars().collect();
    let mut chars_t: Vec<char> = t.chars().collect();
    chars_s.sort();
    chars_t.sort();

    let string_s: String = chars_s.iter().collect();
    let string_t: String = chars_t.iter().collect();

    string_s == string_t
}
