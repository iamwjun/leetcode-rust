fn main() {
    let s = String::from("abc");
    let t = String::from("ahbgadc");

    println!("{:?}", is_subsequence(s, t));
}

fn is_subsequence(s: String, t: String) -> bool {
    let mut s_iter = s.chars().peekable();
    println!("{:?}", s_iter);
    
    for char_t in t.chars() {
        println!("t for {}", char_t);
        if let Some(&chart_s) = s_iter.peek() {
            println!("{}", &chart_s);
            if char_t == chart_s {
                s_iter.next();
            }
        }
    }
    println!("{:?}", s_iter);
    s_iter.peek().is_none()
}
