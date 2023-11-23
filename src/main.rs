fn main() {
    let s = String::from("0P");

    println!("{:?}", is_palindrome(s));
}

fn is_palindrome(s: String) -> bool {
    let s = s.chars().filter(|&c| c.is_alphanumeric()).collect::<String>().to_lowercase();
    let reversed = s.chars().rev().collect::<String>();
    s == reversed
}
