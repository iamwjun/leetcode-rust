### 链表
```Rust
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    fn insert(&mut self, val: i32) {
        let node = Box::new(ListNode {
            val,
            next: self.next.take(),
        });
        self.next = Some(node);
    }

    fn pring(&self) {
        let mut current = Some(self);
        while let Some(node) = current {
            println!("val is {}", node.val);
            current = node.next.as_ref().map(|n| &**n);
        }
        println!("None")
    }
}

fn create_list_node(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut ret = None;
    let mut pre_node = &mut ret;

    for v in vec.iter() {
        let node = Some(Box::new(ListNode::new(*v)));
        *pre_node = node;

        match pre_node {
            Some(n) => { pre_node = &mut n.next },
            None => { unreachable!(); },
        }
    }

    ret
}

fn main() {
    // 手动赋值
    // let mut head = ListNode::new(4);
    // head.next = Some(Box::new(ListNode::new(5)));
    // head.next.as_mut().unwrap().next = Some(Box::new(ListNode::new(2)));
    // head.next.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(10)));

    // head.pring();

    let vec = vec![5, 2, 13, 3, 8];
    let head = create_list_node(vec);

    println!("head is {:?}", head);
}
```