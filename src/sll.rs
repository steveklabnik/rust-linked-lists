use std::fmt;
use std::mem;

pub struct LinkedList<T> {
    head: Node<T>,
    len: int
}

pub struct Node<T> {
    next: Option<Box<Node<T>>>,
    data: T
}

pub struct NodeIterator<'r, T:'r> {
    slice: Option<&'r Node<T>>,
    index: int
}


impl<T> LinkedList<T> {
    pub fn new(item: T) -> LinkedList<T> {
        LinkedList {
            head: Node::new(item),
            len: 0i
        }
    }

    pub fn push(&mut self, item: T) {
        self.head.push(item);
        self.len += 1;
    }

    pub fn pop(&mut self) {
        self.head.pop();
        self.len -= 1;
    }

    pub fn insert(&mut self, item: T, index: int) {
        let index_: int = if index > self.len {
                        fail!("List index out of range");
                    } else if index < 0 {
                        self.len + index - 1
                    } else {
                        index
                    };
        self.head.insert(item, index_);
        self.len += 1;
    }

    pub fn remove(&mut self, index: int) {
        let index_: int = if index > self.len {
                        fail!("List index out of range");
                    } else if index < 0 {
                        self.len + index - 1
                    } else {
                        index
                    };
        self.head.remove(index_);
        self.len -= 1;
    }

    pub fn iter(&mut self) -> NodeIterator<T> {
        self.head.iter()
    }
}

impl<T> Node<T> {
    fn new(item: T) -> Node<T> {
        Node {
            next: None,
            data: item
        }
    }

    fn push(&mut self, item: T) {
        match self.next {
            Some(ref mut next) => return next.push(item),
            None => {}
        };
            self.next = Some(box Node::new(item))
    }

    fn pop(&mut self) {
        match self.next {
            Some(ref mut first_next) => {
                match first_next.next {
                    Some(_) => { return first_next.pop() },
                    None => {}
                }
            },
            None => {}
        }
        self.next = None;
    }

    fn insert(&mut self, item: T, index: int) {
        match self.next {
            Some(ref mut next) => {
                if index > 0 {
                    return next.insert(item, index - 1)
                }
            },
            None => {}
        };

        let chain_node: Option<Box<Node<T>>> = mem::replace(&mut self.next, None);
        let mut new_node: Box<Node<T>> = box Node::new(item);
        new_node.next = chain_node;
        self.next = Some(new_node);
    }

    fn remove(&mut self, index: int) {
        match self.next {
            Some(ref mut next) => {
                if index > 0 {
                    return next.remove(index - 1);
                }
            },
            None => {}
        };

        self.next = match self.next {
            Some(ref mut target) => { mem::replace(&mut target.next, None) },
            None => { fail!("List index out of range.") }
        };
    }

    fn iter(&self) -> NodeIterator<T> {
        NodeIterator { slice: Some(self), index: 0i }
    }
}

impl<'r, T> Iterator<&'r Node<T>> for NodeIterator<'r, T> {
    fn next(&mut self) ->  Option<&'r Node<T>> {
        match self.slice {
            Some(node) => {
                self.slice = node.next.as_ref().map(|box_ref_node| &**box_ref_node);
                self.index += 1;
                Some(node)
            },
            None => None
        }
    }
}

impl<T: fmt::Show> fmt::Show for LinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LList: Size: {2}, Data: {0}, Addr: {1}", self.head.data, self.head.next, self.len)
    }
}

impl<T: fmt::Show> fmt::Show for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LNode: Data: {0}, Addr: {1}", self.data, self.next)
    }
}
