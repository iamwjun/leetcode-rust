use std::collections::VecDeque;

struct FrontMiddleBackQueue {
    deque: VecDeque<i32>
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FrontMiddleBackQueue {

    fn new() -> Self {
        FrontMiddleBackQueue { deque: VecDeque::new() }
    }
    
    fn push_front(&mut self, val: i32) {
        self.deque.insert(0, val);
    }
    
    fn push_middle(&mut self, val: i32) {
        let pos = self.len() / 2;
        self.deque.insert(pos, val);
    }
    
    fn push_back(&mut self, val: i32) {
        self.deque.insert(self.len(), val)
    }
    
    fn pop_front(&mut self) -> i32 {
        if let Some(n) = self.deque.pop_front() {
            return  n;
        }
        -1
    }
    
    fn pop_middle(&mut self) -> i32 {
        if let Some(n) = self.deque.remove(self.middle_index()) {
            return n;
        }
        -1
    }
    
    fn pop_back(&mut self) -> i32 {
        if let Some(n) = self.deque.pop_back() {
            return n;
        }
        -1
    }

    fn middle_index(&self) -> usize {
        (self.len() - 1) / 2
    }

    fn len(&self) -> usize {
        self.deque.len()
    }
}

/**
 * Your FrontMiddleBackQueue object will be instantiated and called as such:
 * let obj = FrontMiddleBackQueue::new();
 * obj.push_front(val);
 * obj.push_middle(val);
 * obj.push_back(val);
 * let ret_4: i32 = obj.pop_front();
 * let ret_5: i32 = obj.pop_middle();
 * let ret_6: i32 = obj.pop_back();
 */

fn main() {
    let mut q = FrontMiddleBackQueue::new();

    println!("{:?}", q.push_front(1));

    println!("{:?}", q.push_back(2));

    println!("{:?}", q.push_middle(3));

    println!("{:?}", q.push_middle(4));

    println!("{:?}", q.deque);

    println!("{:?}", q.pop_front());

    println!("{:?}", q.deque);

    println!("{:?}", q.pop_middle());

    println!("{:?}", q.deque);

    println!("{:?}", q.pop_middle());

    println!("{:?}", q.deque);

    println!("{:?}", q.pop_back());

    println!("{:?}", q.deque);

    println!("{:?}", q.pop_front());
}

