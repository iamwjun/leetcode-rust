fn main() {
    let numbers: Vec<i32> = vec![2,7,11,15];
    let  target = 9;

    println!("{:?}", two_sum(numbers, target));
}

fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: std::collections::HashMap<&i32, usize> = std::collections::HashMap::new();

    for (i, num) in numbers.iter().enumerate() {
        let value = target - num;
        if let Some(&index) = map.get(&value) {
            return vec![index as i32, i as i32 + 1];
        }
        map.insert(num, i + 1);
    }
    vec![]
}
