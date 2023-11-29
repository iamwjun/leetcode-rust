use std::collections::BTreeSet;

struct SmallestInfiniteSet {
    start: i32,
    add: BTreeSet<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SmallestInfiniteSet {

    fn new() -> Self {
        SmallestInfiniteSet {
            start: 1,
            add: BTreeSet::new()
        }
    }
    
    fn pop_smallest(&mut self) -> i32 {
        match self.add.iter().min() {
            Some(&min) => {
                self.add.remove(&min);
                min
            },
            None => {
                self.start += 1;
                self.start
            },
        }
    }

    fn add_back(&mut self, num: i32) {
        if num < self.start {
            self.add.insert(num);
        }
    }
}

/**
 * Your SmallestInfiniteSet object will be instantiated and called as such:
 * let obj = SmallestInfiniteSet::new();
 * let ret_1: i32 = obj.pop_smallest();
 * obj.add_back(num);
 */

fn main() {
    let mut smallest = SmallestInfiniteSet::new();

    println!("{:?}", smallest.pop_smallest());

    println!("{:?}", smallest.add_back(1));

    println!("{:?}", smallest.add_back(2));

    println!("{:?}", smallest.add_back(3));

    println!("{:?}", smallest.pop_smallest());

    println!("{:?}", smallest.pop_smallest());

    println!("{:?}", smallest.pop_smallest());
}

