struct Node<T> {
    next: Option<Box<Node<T>>>,
    value: T,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Self { next: None, value }
    }

    fn add(&mut self, node: Node<T>) {
        match &self.next {
            None => self.next = Some(Box::new(node)),
            Some(_) => {}
        }
    }

    fn append(&mut self, value: T) {
        if let Some(node) = &mut self.next {
            node.append(value);
        } else {
            self.add(Node::new(value));
        }
    }

    fn len(&self) -> usize {
        let mut current = self;
        let mut count = 1;
        while let Some(ref rest) = current.next {
            count += 1;
            current = rest;
        }
        count
    }
}

fn main() {}
