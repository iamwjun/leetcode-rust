fn main() {
    let word1 = "xyz";
    let word2 = "pqrs";
    let len1 = word1.len();
    let len2 = word2.len();

    let len = len1 + len2;

    println!("{}", len);

    let mut v: Vec<char> = vec!['a'; 10];

    for (i, w) in word1.chars().enumerate() {
        let index: usize = i + (i * 1);
        v[index] = w
    }

    for (i, w) in word2.chars().enumerate() {
        let index: usize = i + 1 + (i * 1);
        v[index] = w
    }

    let string: String = v.iter().collect();

    println!("{:?}", string)
}
