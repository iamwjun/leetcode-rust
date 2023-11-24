fn main() {
    let ransom_note = String::from("aa");
    let magazine = String::from("aab");

    println!("{:?}", can_construct(ransom_note, magazine));
}

fn can_construct(ransom_note: String, magazine: String) -> bool {
    let mut r = ransom_note.chars().peekable();

    for m in magazine.chars() {
        if let Some(&p) = r.peek() {
            if m == p {
                r.next();
            }
        }
    }
    r.peek().is_none()
}
