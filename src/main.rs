fn main() {
    let mut s = String::from("abcd");

    println!("{}", make_smallest_palindrome(s));
}

fn make_smallest_palindrome(s: String) -> String {
    let s_chars: Vec<char> = s.chars().collect();
    println!("{:?}", s_chars);
    
    let mut t_chars = Vec::new();
    for char in s_chars {
        t_chars.push(char);
        t_chars.push('#');
    }

    println!("{:?}", t_chars);
    s
}


