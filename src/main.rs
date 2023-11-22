fn main() {
    let roman_numeral = String::from("IX");

    println!("{:?}", roman_to_int(roman_numeral));
}

fn roman_to_int(s: String) -> i32 {
    let roman_map = std::collections::HashMap::from([
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ]);
    
    let mut res = 0;

    for (i, char) in s.chars().enumerate() {
        if i < s.len() - 1 && roman_map[&char] < roman_map[&s.chars().nth(i + 1).unwrap()] {
            res -= roman_map[&char];
        } else {
            res += roman_map[&char];
        }
    }

    res
}
