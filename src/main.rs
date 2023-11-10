fn main() {
    let spells: Vec<i32> = vec![5, 1, 3];
    let potions: Vec<i32> = vec![1, 2, 3, 4, 5];
    let success: i64 = 7;

    println!("{:?}", successful_pairs(spells, potions, success))
}

fn successful_pairs(spells: Vec<i32>, potions: Vec<i32>, success: i64) -> Vec<i32> {
    let mut res: Vec<i32> = vec![];
    for s in spells {
        let mut total: i32 = 0;
        for p in &potions {
            if  (s as i64) * (*p as i64) >= success {
                total += 1;
            }
        }
        res.push(total);
    }
    res
}