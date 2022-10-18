use std::rc::Rc;
type Link = Option<Rc<Node>>;

pub struct FibHeap {
    count: i32,
    tree_count: i32,
    trees: Vec<Link>,
}

struct Node{
    value: i32,
    parent: Link,
    children: Vec<Link>,
}

impl FibHeap {
    pub fn new() -> Self {
        FibHeap {
            count: 0,
            tree_count: 0,
            trees: Vec::new()}
    }

    pub fn insert(&mut self, val: i32) {
        let new_node = Rc::new(Node {
            value: val,
            parent: None,
            children: Vec::new()
        });
        self.count += 1;
        self.tree_count += 1;
        self.trees.push(Some(new_node))
    }

    pub fn print_heap(&self) {
        for x in &self.trees {
            match x {
                Some(node) => println!("{}", node.value),
                None => println!("I'm confused"),
            };
        }
    }
}

type Pointer<T> = Option<Rc<NodeList<T>>>;

pub struct CircularList<T> {
    start: Pointer<T>,
}

struct NodeList<T> {
    element: T,
    next: Pointer<T>,
    prev: Pointer<T>,
}

impl<T> CircularList<T> {
    pub fn new() -> Self {
        CircularList { start: None,}
    }

    pub fn insert_end(&mut self, elem: T) {
        match &self.start{
            Some(node) => {
                let p = Rc::new(NodeList::<T> {
                    element: elem,
                    next: Some(node.clone()),
                    prev: Some(node.clone()),
                });
                node.prev.as_mut().unwrap().next = Some(p);
                node.prev = Some(p);
            },
            None =>{
                self.start = Some(Rc::new(NodeList::<T> {
                    element: elem,
                    next: None,
                    prev: None,
                }));
                self.start.unwrap().next = self.start;
                self.start.unwrap().prev = self.start;
            },
        }
        let p = Rc::new(NodeList::<T>{
            element: elem,
            next: self.start.clone(),
            prev: self.start.unwrap().prev,
        });
        self.start.unwrap().prev = Some(p);
    }
}
