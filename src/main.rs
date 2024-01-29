fn main() {
    let ring = String::from("godding");
    let key = String::from("godding");

    println!("{:?}", find_rotate_steps(ring, key));
}

fn find_rotate_steps(ring: String, key: String) -> i32 {
    let mut ret = 0;
    let mut chars: Vec<char> = ring.chars().collect();

    for k in key.chars() {
        while Some(k) != chars.get(0).copied() {
            if let Some(last_item) = chars.pop() {
                chars.insert(0, last_item);
            }
            ret += 1
        }
    }

    ret
}
