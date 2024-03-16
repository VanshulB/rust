#[allow(dead_code)]
pub struct LinkedList {
    head: Link,
}
#[allow(dead_code)]
struct Node {
    element: u32,
    next: Link,
}
type Link = Option<Box<Node>>;

#[allow(dead_code)]
impl LinkedList {
    fn empty() -> LinkedList {
        LinkedList { head: None }
    }
    fn push(&mut self, element: u32) {
        let old_head = self.head.take();
        let new_head = Some(Box::new(Node::new_with_next(element, old_head)));
        self.head = new_head;
    }
}

#[allow(dead_code)]
impl Node {
    fn new(element: u32) -> Node {
        Node {
            element,
            next: None,
        }
    }
    fn new_with_next(element: u32, next: Option<Box<Node>>) -> Node {
        Node { element, next }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut linked_list = LinkedList::empty();
        linked_list.push(1024);
        linked_list.push(12);
    }
}
