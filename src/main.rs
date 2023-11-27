use std::collections::HashMap;

fn main() {
    let strs: Vec<&str> = vec!["eat", "tea", "tan", "ate", "nat", "bat"];

    println!("{:?}", group_anagrams(strs));
}

fn group_anagrams(strs: Vec<&str>) -> Vec<Vec<String>> {
    let mut groups: HashMap<Vec<char>, Vec<String>> = HashMap::new();

    for s in strs {
        let mut chars: Vec<char> = s.chars().collect();
        chars.sort_unstable();

        groups.entry(chars).or_insert(Vec::new()).push(s.to_string())
    }

    groups.into_values().collect()
}
