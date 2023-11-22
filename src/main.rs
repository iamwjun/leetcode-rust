fn main() {
    let numeral = 3;

    println!("{:?}", int_to_roman(numeral));
}

fn int_to_roman(mut num: i32) -> String {
    let mut result = String::new();

    let roman_map: Vec<(&str, i32)> = vec![
        ("M", 1000),
        ("CM", 900),
        ("D", 500),
        ("CD", 400),
        ("C", 100),
        ("XC", 90),
        ("L", 50),
        ("XL", 40),
        ("X", 10),
        ("IX", 9),
        ("V", 5),
        ("IV", 4),
        ("I", 1),
    ];

    for (i, v) in roman_map {
        while num >= v {
            result.push_str(i);
            num -= v;
        }
    }

    result
}
