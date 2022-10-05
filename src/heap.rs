pub struct FibHeap<T> {
    count: i32,
    tree_count: i32,
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T>{
    value: T,
    next: Link<T>,
}

impl<T> FibHeap<T> {
    pub fn new() -> Self {
        FibHeap {
            count: 0,
            tree_count: 0,
            head: None}
    }

    pub fn insert(&mut self, val: T) {
        let new_node = Box::new(Node {
            value: val,
            next: self.head.take(),
        });
        self.count += 1;
        self.tree_count += 1;
        self.head = Some(new_node);
    }
}
