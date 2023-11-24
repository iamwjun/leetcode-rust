fn main() {
    let t = String::from("anagram");
    let s = String::from("nagaram");

    println!("{:?}", is_anagram(s, t));
}

fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return  false;
    }

    let mut array = vec![0; 26];

    for char in s.into_bytes() {
        array[(char - b'a') as usize] += 1;
    }

    for char in t.into_bytes() {
        if array[(char - b'a') as usize] > 0 {
            array[(char - b'a') as usize] -= 1;
        } else {
            return false;
        }
    }

    true
}
