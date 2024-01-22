use std::collections::HashMap;

fn main() {
    let num: i32 = 2736;

    println!("{:?}", maximum_swap(num));
}

fn maximum_swap(num: i32) -> i32 {
    let number_str = num.to_string();
    println!("{:?}", number_str);
    let mut map = HashMap::new();

    for (i, v) in number_str.chars().enumerate() {
        map.entry(i).or_insert(v.to_string());
    }

    println!("{:?}", map);

    1
}
