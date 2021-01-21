use std::iter::FromIterator;

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(element: T) -> Box<Self> {
        Box::new(Self {
            data: element,
            next: None,
        })
    }
}

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
    length: usize,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            length: 0,
        }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn push(&mut self, element: T) {
        self.length += 1;
        let mut new_node = Node::new(element);
        new_node.next = self.head.take();
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            self.length -= 1;
            let node = self.head.take().unwrap();
            self.head = node.next;
            Some(node.data)
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|h| &h.data)
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut rev_list = Self::new();
        let mut node = self.head;
        while let Some(n) = node {
            rev_list.push(n.data);
            node = n.next;
        }
        rev_list
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = Self::new();
        for i in iter {
            list.push(i);
        }
        list
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut v = Vec::with_capacity(self.len());
        let mut node = self.head;
        while let Some(n) = node {
            v.push(n.data);
            node = n.next;
        }
        v.reverse();
        v
    }
}
