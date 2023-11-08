fn main() {
    let word1 = "xyzuvw";
    let word2 = "pqrs";
    let max_len = std::cmp::max(word1.len(), word2.len());
    let mut res = String::from("");

    for i in 0..max_len {
        if i < word1.len() {
            res.push(word1.chars().nth(i).unwrap());
        }

        if i < word2.len() {
            res.push(word2.chars().nth(i).unwrap());
        }
    }

    println!("{}", res)
}
