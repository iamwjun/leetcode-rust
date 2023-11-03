fn main() {
    let word1: &str = "ab";
    let word2: &str = "pqrs";
    let count1 = word1.chars().count();
    let count2 = word2.chars().count();

    let total_char_count = count1 + count1;

    const len = total_char_count;

    let mut v = [""; len];

    // for w in word1.chars() {
    //     v.push(w)
    // }
    v[0] = "a";

    println!("{:?}", v.join(""))
}
