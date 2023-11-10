fn main() {
    let spells: Vec<i32> = vec![5, 1, 3];
    let potions: Vec<i32> = vec![1, 2, 3, 4, 5];
    let success: i32 = 7;

    println!("{:?}", successful_pairs(spells, potions, success))
}

fn search_index(potions: Vec<i32>, spell: i32, success: i32) -> i32 {
    let mut size = potions.len() as i32;
    let mut left = 0;
    let mut right = size;
    while left < right {
        let mid = left + size / 2;
        
        if spell * potions[mid] >= success {
            left = mid + 1;
        } else {
            right = mid;
        }

        size = right - left;
    }

    0
}

fn successful_pairs(spells: Vec<i32>, mut potions: Vec<i32>, success: i32) -> Vec<i32> {
    let len = potions.len();
    let mut res: Vec<i32> = vec![];

    potions.sort_unstable();
    
    for s in spells {
        println!("spells {s}");
        let mut total = 0;
        match potions.binary_search_by(|&x| {
            if x * s  <= success {
                std::cmp::Ordering::Equal
            } else {
                std::cmp::Ordering::Less
            }
        }) {
            Ok(index) => {
                println!("element is {}, s is {}, success is {}", potions[index], s, success);
                total = len - index + 1
            },
            Err(_) => {
                println!("no found");
            },
        }
        res.push(total.try_into().unwrap())
    }
    res
}