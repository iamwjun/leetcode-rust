fn main() {
    let ransom_note = String::from("aa");
    let magazine = String::from("aab");

    println!("{:?}", can_construct(ransom_note, magazine));
}

fn can_construct(ransom_note: String, magazine: String) -> bool {
    let mut vec = vec![0; 26];

    magazine.bytes().for_each(|e| vec[e as usize - 97] += 1);

    let bytes = ransom_note.as_bytes();
    for b in bytes {
        let i: usize = (*b - 97) as usize;
        vec[i] -= 1;
        if vec[i] < 0 {
            return false;
        }
    }

    true
}
