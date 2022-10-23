use std::fmt;
use std::option::Option;

pub struct LinkedList<T: fmt::Display + std::cmp::PartialEq + Clone> {
    head: Option<Box<Node<T>>>,
    size: usize,
}

struct Node<T: fmt::Display + std::cmp::PartialEq + Clone> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T: fmt::Display + std::cmp::PartialEq + Clone> Node<T> {
    pub fn new(value: T, next: Option<Box<Node<T>>>) -> Node<T> {
        Node {value: value, next: next}
    }
}

impl<T: fmt::Display + std::cmp::PartialEq + Clone> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList {head: None, size: 0}
    }
    
    pub fn get_size(&self) -> usize {
        self.size
    }
    
    pub fn is_empty(&self) -> bool {
        self.get_size() == 0
    }
    
    pub fn push_front(&mut self, value: T) {
        let new_node: Box<Node<T>> = Box::new(Node::new(value, self.head.take()));
        self.head = Some(new_node);
        self.size += 1;
    }
    
    pub fn pop_front(&mut self) -> Option<T> {
        let node: Box<Node<T>> = self.head.take()?;
        self.head = node.next;
        self.size -= 1;
        Some(node.value)
    }
}


impl<T: fmt::Display + std::cmp::PartialEq + Clone> fmt::Display for LinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut current: &Option<Box<Node<T>>> = &self.head;
        let mut result = String::new();
        loop {
            match current {
                Some(node) => {
                    result = format!("{} {}", result, node.value);
                    current = &node.next;
                },
                None => break,
            }
        }
        write!(f, "{}", result)
    }
}

impl<T: fmt::Display + std::cmp::PartialEq + Clone> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut current = self.head.take();
        while let Some(mut node) = current {
            current = node.next.take();
        }
    }
}
impl<T: fmt::Display + std::cmp::PartialEq + Clone> Clone for LinkedList<T> {
    fn clone(&self) -> Self {
        // Self { head: self.head.clone(), size: self.size.clone() }
        let mut list: LinkedList<T> = LinkedList::new();
        let mut current: &Option<Box<Node<T>>> = &self.head;
        loop {
            match current {
                Some(node) => {
                    list.push_front(node.value.clone());
                    current = &node.next;
                },
                None => break,
            }
        }
        list
    }
}


impl<T: fmt::Display + std::cmp::PartialEq + Clone> PartialEq for LinkedList<T> {
    fn eq(&self, other: &Self) -> bool {
        // self.isbn == other.isbn
        if self.size != other.size {
            false
        } else {
            let mut flag = true;
            let mut current_self: &Option<Box<Node<T>>> = &self.head;
            let mut current_other = &self.head;
            loop {
                match current_self {
                    Some(node_self) => {
                        // result = format!("{} {}", result, node.value);
                        match current_other {
                            Some(node_other) => {
                                if node_other.value != node_self.value {
                                    flag = false;
                                    break;
                                }
                                current_other = &node_other.next;
                            }, 
                            None => {
                                flag = false;
                                break;
                            }
                        }
                        
                        current_self = &node_self.next;
                    },
                    None => {
                        match current_other {
                            Some(node_other) => {
                                flag = false;
                                break;
                            }, 
                            None => {
                                break;
                            }
                        }
                    },
                }
            }
            flag
        }
    }
}
