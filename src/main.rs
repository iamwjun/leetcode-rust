struct MinStack {
    stack: Vec<i32>,
    min: i32,
    last: usize,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {

    fn new() -> Self {
        MinStack {
            stack: Vec::new(),
            last: 0,
            min: std::i32::MAX
        }
    }
    
    fn push(&mut self, val: i32) {
        self.stack.push(val);
        self.last += 1;
        self.min = self.min.min(val);
    }
    
    fn pop(&mut self) {
        self.stack.remove(self.last - 1);
        self.last -= 1;
        
        match self.stack.iter().min() {
            Some(&min) => self.min = min,
            _ => self.min = std::i32::MAX
        }
    }
    
    fn top(&mut self) -> i32 {
        *self.stack.get(self.last - 1).unwrap()
    }
    
    fn get_min(&self) -> i32 {
        self.min
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */

fn main() {
    let mut min_stack = MinStack::new();

    println!("{:?}", min_stack.push(-2));

    println!("{:?}", min_stack.push(0));

    println!("{:?}", min_stack.push(-3));

    println!("{:?}", min_stack.get_min());

    println!("{:?}", min_stack.stack);

    println!("pop {:?}", min_stack.pop());

    println!("{:?}", min_stack.stack);

    println!("{:?}", min_stack.top());

    println!("{:?}", min_stack.get_min());

    
}


