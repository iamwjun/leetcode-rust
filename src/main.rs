fn main() {
    let s = String::from("()[]{}");

    println!("{:?}", is_valid(s));

    println!("{}", 1 << 0);

    println!("{}", 1 << 1);
}

fn is_valid(s: String) -> bool {
    let mut v: Vec<u8> = vec![];

    for char in s.into_bytes() {
        match char {
            b')' | b']' | b'}' => if Some(char) != v.pop() {
                return false;
            },
            _ => v.push(char + (1 << (char & 1)))
        }
    }
    v.is_empty()
}

