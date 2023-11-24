fn main() {
    let t = String::from("anagram");
    let s = String::from("nagaram");

    println!("{:?}", is_anagram(s, t));
}

fn is_anagram(s: String, t: String) -> bool {
    let mut array = vec![0; 26];

    for char in s.into_bytes() {
        array[(char - b'a') as usize] += 1;
    }

    for char in t.into_bytes() {
        array[(char - b'a') as usize] -= 1;
    }

    for n in array {
        if n != 0 {
            return false;
        }
    }

    true
}
