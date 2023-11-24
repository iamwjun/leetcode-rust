use  std::collections::HashMap;

fn main() {
    let pattern = String::from("abba");
    let s = String::from("dog cat cat fish");
    let words: Vec<&str> = s.split(" ").collect();
    println!("{:?}", words);
    println!("{:?}", word_pattern(pattern, s));
}

fn word_pattern(pattern: String, s: String) -> bool {
    let words: Vec<&str> = s.split(" ").collect();

    if words.len() != pattern.len() {
        return false;
    }

    let mut p_to_s: HashMap<char, &str> = HashMap::new();
    let mut s_to_p: HashMap<&str, char> = HashMap::new();

    for (letter, word) in pattern.chars().zip(words.iter()) {
        match (p_to_s.get(&letter), s_to_p.get(word)) {
            (Some(map_p), Some(map_s)) if map_p == word && map_s == map_s => continue,
            (None, None) => {
                p_to_s.insert(letter, &word);
                s_to_p.insert(word, letter);
            },
            _ => return false,
        }
    }

    true
}
