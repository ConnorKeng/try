#![allow(dead_code)]

#[derive(Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Node<T> {
        Node { value, next: None }
    }
}

impl<T> Node<T> {
    // pub fn get_next(&self) -> Option<&Node<T>> {
    //     (*self.next).as_ref()
    // }
    // pub fn get_next_mut(&mut self) -> Option<&mut Node<T>> {
    //     (*self.next).as_mut()
    // }
}

impl<T> std::fmt::Display for Node<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(next) = &self.next {
            write!(f, "[{}]->{}", self.value, next)
        } else {
            write!(f, "[{}]", self.value)
        }
    }
}
