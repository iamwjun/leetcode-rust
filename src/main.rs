struct NumArray {
    nums: Vec<i32>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {

    fn new(nums: Vec<i32>) -> Self {
        NumArray { nums }
    }
    
    fn update(&mut self, index: i32, val: i32) {
        if (index as usize) < self.nums.len() {
            // 更新数组中的元素
            self.nums[index as usize] = val;
        }
    }
    
    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.nums[(left as usize)..=(right as usize)].iter().sum()
    }
}

fn main() {
    let mut num_array = NumArray::new(vec![1, 2, 3, 4, 5]);

    println!("Original Array: {:?}", num_array.nums);

    num_array.update(2, 10);

    println!("Updated Array: {:?}", num_array.nums);

    let sum = num_array.sum_range(1, 3);
    println!("Sum of range: {}", sum);
}