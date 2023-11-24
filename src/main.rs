fn main() {
    let ransom_note = String::from("aa");
    let magazine = String::from("aab");

    println!("{:?}", can_construct(ransom_note, magazine));
}

fn can_construct(ransom_note: String, magazine: String) -> bool {
    let mut char_count = std::collections::HashMap::new();

    for char in magazine.chars() {
        *char_count.entry(char).or_insert(0) += 1;
    }

    for r in ransom_note.chars() {
        if let Some(count) = char_count.get_mut(&r) {
            if *count > 0 {
                *count -= 1;
            } else {
                return false;
            }
        } else {
            return  false;
        }
    }

    true
}
