fn main() {
    let num: i32 = 12;

    println!("{:?}", maximum_swap(num));
}

fn maximum_swap(mut num: i32) -> i32 {
    let mut vec: Vec<i32> = Vec::new();

    while num > 0 {
        vec.insert(0, num % 10);
        num /= 10;
    }

    for (i, m) in vec.iter().enumerate() {
        if let Some((index, max_value)) = vec
            .iter()
            .skip(i + 1)
            .enumerate()
            .max_by_key(|&(_, &value)| value)
        {
            let max_index = index + i + 1;
            if max_value > m {
                vec.swap(i, max_index);
                break;
            }
        }
    }

    vec.iter().fold(0, |acc, &x| acc * 10 + x)
}
