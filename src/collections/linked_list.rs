use std::fmt::Display;

struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        return Node { value, next: None };
    }
}

pub struct LinkedList<T> {
    first: Option<Box<Node<T>>>,
    count: usize
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        return LinkedList { first: None, count: 0 };
    }

    pub fn insert(&mut self, value: T, mut index: usize) -> bool {
        if index > self.count {
            return false;
        }
        let mut new_node = Box::new(Node::new(value));
        self.count += 1;

        if index == 0 {
            let old_node = self.first.take();

            new_node.next = old_node;
            self.first = Some(new_node);
            
            return true;
        }
        let mut before_target_node = self.first.as_mut().unwrap();
        while index > 1 {
            before_target_node = before_target_node.next.as_mut().unwrap();
            index -= 1;
        }
        new_node.next = before_target_node.next.take();
        before_target_node.next = Some(new_node);

        return true; 
    }

    pub fn remove_at(&mut self, mut index: usize) -> bool {
        if index >= self.count {
            return false;
        }
        self.count -= 1;

        if index == 0 {
            let old_node = self.first.take();
            self.first = old_node.unwrap().next.take();

            return true;
        }
        let mut before_target_node = self.first.as_mut().unwrap();
        while index > 1 {
            before_target_node = before_target_node.next.as_mut().unwrap();
            index -= 1;
        }
        let target_node = before_target_node.next.as_mut().unwrap();
        before_target_node.next = target_node.next.take();

        return true;
    }
}

impl<T : Display> LinkedList<T> {
    pub fn to_string(&self) -> String {
        let mut result = String::new();
        let mut current_node = &self.first;

        while let Some(node) = current_node {
            result.push_str(&node.value.to_string());
            if node.next.is_some() {
                result.push_str(" -> ");
            }
            current_node = &node.next;
        }
        return result;
    }
}

