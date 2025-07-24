// src/main.rs
#[derive(Debug)]
struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

#[derive(Debug)]
pub struct LinkedList {
    head: Option<Box<Node>>,
}

impl LinkedList {
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    pub fn push_front(&mut self, val: i32) {
        let new_node = Box::new(Node {
            val,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    pub fn pop_front(&mut self) -> Option<i32> {
        if let Some(node) = self.head.take() {
            self.head = node.next;
            Some(node.val)
        } else {
            None
        }
    }

    pub fn peek(&self) -> Option<i32> {
        self.head.as_ref().map(|node| node.val)
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn print_list(&self) {
        let mut current = self.head.as_ref();
        while let Some(node) = current {
            print!("{} -> ", node.val);
            current = node.next.as_ref();
        }
        println!("None");
    }
}


fn main() {
    let mut list = LinkedList::new();

    list.push_front(10);
    list.push_front(20);
    list.push_front(30);

    list.print_list(); // 30 -> 20 -> 10 -> None

    println!("Popped: {:?}", list.pop_front()); // 30
    list.print_list(); // 20 -> 10 -> None
}
