fn main() {
    let s: Vec<String> = vec![
        String::from("flower"),
        String::from("flow"),
        String::from("flight"),
    ];

    println!("{:?}", longest_common_prefix(s));
}

fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut res = String::new();

    if let Some(str) = strs.get(0) {
        for (i, s) in str.chars().enumerate() {
            let mut is_breack = false;
            for string in strs.iter().skip(1) {
                if Some(s) != string.chars().nth(i) {
                    is_breack = true;
                    break;
                }
            }
            if is_breack {
                break;
            }
            res.push(s);
        }
    }

    res
}