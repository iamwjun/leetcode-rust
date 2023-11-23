fn main() {
    let numbers: Vec<i32> = vec![2,7,11,15];
    let  target = 9;

    println!("{:?}", two_sum(numbers, target));
}

fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let len = numbers.len();

    for i in 0..len {
        for j in (i + 1)..len {
            if numbers[i] + numbers[j] == target {
                return  vec![i as i32 + 1, j as i32 + 1];
            }
        }
    }

    vec![]
}
