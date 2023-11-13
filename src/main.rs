fn main() {
    let mut spells: Vec<i32> = vec![5, 1, 3];
    let mut potions: Vec<i32> = vec![1, 2, 3, 4, 5];
    let success: i64 = 7;

    potions.sort_unstable();
    let len = potions.len() as i32;

    for s in spells.iter_mut() {
        let idx = potions.partition_point(|&x| (x as i64 * *s as i64) < success) as i32;
        if idx == len {
            *s = 0;
        } else {
            *s = len - idx;
        }
    }

    println!("{:?}", spells);
}