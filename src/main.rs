fn main() {
    let s = String::from("ABFCACDB");

    println!("{:?}", min_length(s));
}

fn min_length(s: String) -> usize {
    let ab = "AB".as_bytes();
    let cd = "CD".as_bytes();
    s.as_bytes()
        .windows(2)
        .filter(|&window| window == ab)
        .count()
        + s.as_bytes()
            .windows(2)
            .filter(|&window| window == cd)
            .count()
}
