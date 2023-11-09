fn main() {
    let s = "01000111";
    // let s = "00111";
    // let s = "111";
    let (mut x, mut y, mut z) = (0, 0, 0);
    let mut pre_char = 'a';

    for (i, c) in s.chars().enumerate() {
        if c == '1' {
            z += 1;
            x = std::cmp::max(x, std::cmp::min(y, z) * 2);
        } else if i == 0 || pre_char == '1' {
            y = 1;
            z = 0;
        } else {
            y += 1
        }
        pre_char = c;
    }

    println!("{}", x)
}
