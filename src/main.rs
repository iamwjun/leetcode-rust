fn main() {
    let s = String::from("/../");

    println!("{:?}", simplify_path(s));
}

fn simplify_path(path: String) -> String {
    let mut v: Vec<&str> = vec![];
    
    for part in path.split('/') {
        match part {
            "" | "." => (),
            ".." => if !v.is_empty() {
                v.pop();
            },
            _ => v.push(part)
        }
    }

    format!("/{}", v.join("/"))
}

