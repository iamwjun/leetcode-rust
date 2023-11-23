fn main() {
    let numbers: Vec<i32> = vec![2,7,11,15];
    let  target = 9;

    println!("{:?}", two_sum(numbers, target));
}

fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let (mut a, mut b) = (0, numbers.len() - 1);

    loop {
        match target.cmp(&(numbers[a] + numbers[b])) {
            std::cmp::Ordering::Equal => return vec![a as i32 + 1, b as i32 +1],
            std::cmp::Ordering::Greater => a += 1,
            std::cmp::Ordering::Less => b -= 1
        }
    }
}
