struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

pub struct LinkedList<T> {
    length: i64,
    head: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    pub fn empty() -> LinkedList<T> {
        LinkedList::<T> {
            head: None,
            length: 0,
        }
    }

    pub fn push(&mut self, item: T) {
        let old_head = self.head.take();
        let new_head = Box::new(Node::<T> {
            value: item,
            next: old_head,
        });
        self.head = Some(new_head);
        self.length = self.length + 1;
    }

    pub fn size(&self) -> i64 {
        self.length
    }
}
