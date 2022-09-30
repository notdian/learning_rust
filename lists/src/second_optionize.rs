use std::mem;

struct List {
    head: Link,
}

type Link = Option<Box<Node>>;

struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

impl List {
    fn new() -> Self {
        List { head: None }
    }

    fn push(&mut self, value: i32) {
        let new_node = Box::new(Node {
            value,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    fn pop(&mut self) -> Option<i32> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.value
        })
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, None);
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take()
        }
    }
}
