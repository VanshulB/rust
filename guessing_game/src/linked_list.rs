#[allow(dead_code)]

struct Node {
    pub data: i32,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(data: i32) -> Node {
        Node { data, next: None }
    }

    fn print(&self) {
        println!("{}", self.data);
    }

    fn new_with_next(data: i32, next: Option<Box<Node>>) -> Node {
        Node { data, next }
    }
}

pub struct LinkedList {
    head: Option<Node>,
    length: u64,
}

impl LinkedList {
    fn new() -> LinkedList {
        LinkedList {
            head: None,
            length: 0,
        }
    }
    fn insert(&mut self, data: i32) {
        self.length += 1;
        match self.head {
            None => {
                let node = Node::new(data);
                self.head = Some(node);
            }
            Some(_) => {
                self.length += 1;
                let curr_node = self.head;
                while let Some(ref mut next_node) = curr_node.next {
                    curr_node = next_node;
                }
                let new_node = Node::new(data);
                curr_node.next = Some(Box::new(new_node));
            }
        }
    }
}
