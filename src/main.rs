use std::collections::HashSet;

fn main() {
    let n = 7;

    println!("{:?}", is_happy(n));
}

fn is_happy(n: i32) -> bool {
    fn get_digits(mut n: i32) -> Vec<i32> {
        let mut vec: Vec<i32> = Vec::new();

        while n!=0 {
            vec.push(n % 10);
            n /= 10
        }

        vec
    }

    fn sum_digits(v: Vec<i32>) -> i32 {
        v.iter().map(|d| d * d).sum()
    }

    let mut has_calculated: HashSet<i32> = HashSet::new();
    let mut curr = n;

    while curr != 1 && !has_calculated.contains(&curr)  {
        has_calculated.insert(curr);
        let digits = get_digits(curr);
        curr = sum_digits(digits);
    }

    curr == 1
}
