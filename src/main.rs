fn main() {
    let numbers: Vec<i32> = vec![2,7,11,15];
    let  target = 9;

    println!("{:?}", two_sum(numbers, target));
}

fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    for (i, num) in numbers.iter().enumerate() {
        let value = target - num;
        if let Some(index) = numbers.iter().rev().position(|&x| x == value) {
            return vec![i as i32 + 1, index as i32 + 1];
        }
    }
    vec![]
}
