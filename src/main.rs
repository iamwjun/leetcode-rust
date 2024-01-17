fn main() {
    let words = vec![
        "cd".to_string(),
        "ac".to_string(),
        "dc".to_string(),
        "ca".to_string(),
        "zz".to_string()
    ];

    println!("{:?}", maximum_number_of_string_pairs(words));
}

fn maximum_number_of_string_pairs(words: Vec<String>) -> i32 {
    let mut ret = 0;

    for (i, word) in words.iter().enumerate() {
        for j in words.iter().skip(i + 1) {
            if *word == j.chars().rev().collect::<String>() {
                ret += 1;
            }
        }
    }

    ret
}
