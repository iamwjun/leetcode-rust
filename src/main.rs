fn main() {
    let s = String::from("   fly me   to   the moon  ");

    println!("{:?}", length_of_last_word(s));
    
}

fn length_of_last_word(s: String) -> i32 {
    let mut len = 0;
    for w in s.chars().rev() {
        if w != ' ' {
            len += 1;
        }
        if w == ' ' && len > 0 {
            break;
        }
    }
    len
}