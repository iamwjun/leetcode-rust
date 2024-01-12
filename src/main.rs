use std::collections::HashMap;

fn main() {
    let words1: Vec<String> = vec![
        "leetcode".to_string(),
        "is".to_string(),
        "amazing".to_string(),
        "as".to_string(),
        "is".to_string()
    ];
    let words2: Vec<String> = vec![
        "amazing".to_string(),
        "leetcode".to_string(),
        "is".to_string()
    ];

    println!("{:?}", count_words(words1, words2));
}

fn count_words(words1: Vec<String>, words2: Vec<String>) -> i32 {
    let mut map: HashMap<String, u32> = HashMap::new();

    for word in words1 {
        map.entry(word)
            .and_modify(|value| { *value += 2 })
            .or_insert(1);
    }

    for word in words2 {
        map.entry(word).and_modify(|value| { *value += 1 });
    }

    map.values().filter(|&&value| value == 2).count() as i32
}
