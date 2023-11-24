fn main() {
    let t = String::from("anagram");
    let s = String::from("nagaram");

    println!("{:?}", is_anagram(s, t));
}

fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return  false;
    }

    let mut map_s = std::collections::HashMap::new();

    for char in s.chars() {
        *map_s.entry(char).or_insert(0) += 1;
    }

    for char in t.chars() {
        if let Some(count) = map_s.get_mut(&char) {
            if *count > 0 {
                *count -= 1;
            } else {
                return  false;
            }
        } else {
            return false;
        }
    }

    true
}
