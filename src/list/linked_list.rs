use std::fmt::{Debug, Display};

#[derive(Clone)]
pub struct Node<T> {
    line : T,
    next : Option<Box<Node<T>>>,
    prev : Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(data : T) -> Self {
        Node {
            line : data,
            next : None,
            prev : None,
        } 
    }
}

#[derive(Clone)]
pub struct LinkedList<T> {
    head : Option<Box<Node<T>>>,
    tail : Option<Box<Node<T>>>
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList{
            head : None,
            tail : None
        }
    }

    pub fn add(&mut self, data : T) 
    where T: Display + Clone + Debug,
    {
        let node : Node<T> = Node::new(data.clone());
        println!("Data: {:?}", data);
    }
}

#[cfg(test)]
mod tests {
    use crate::list::linked_list::LinkedList;

    #[test]
    fn test_linked_list_new() {
        let l : LinkedList<String> = LinkedList::new();
        l.clone().add("Hello".to_string());
    }
}
