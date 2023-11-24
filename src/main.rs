use  std::collections::HashMap;

fn main() {
    let s = String::from("egg");
    let t = String::from("add");

    println!("{:?}", is_isomorphic(s, t));
}

fn is_isomorphic(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return  false;
    }

    let mut s_to_t: HashMap<char, char> = HashMap::new();
    let mut t_to_s: HashMap<char, char> = HashMap::new();

    for (char_s, char_t) in s.chars().zip(t.chars()) {
        match (s_to_t.get(&char_s), t_to_s.get(&char_t)) {
            (Some(&map_s), Some(&map_t)) if map_s == char_t && map_t == char_s => continue,
            (None, None) => {
                s_to_t.insert(char_s, char_t);
                t_to_s.insert(char_t, char_s);
            }
            _ => return  false
        }
    }

    true
}
