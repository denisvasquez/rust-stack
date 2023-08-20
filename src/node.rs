// structure representing a node in the stack
pub struct Node<T> {
    pub data: T,
    pub next: Option<Box<Node<T>>>,
}

// implementation/methods fot the structure `node`
impl<T> Node<T> {
    
    // create a new node, returning the new instance of the structure
    pub fn new(value: T) -> Self { // return a 'self' structure
        Node {
            data: value,
            next: None
        }
    }
}