use crate::node::Node;

// structure representing the stack of nodes
pub struct Stack<T> {
    pub top: Option<Box<Node<T>>>,
    pub length: usize
}

// implementation to structure 'Stack'
impl<T: std::fmt::Display> Stack<T> {
     // create a new stack
    pub fn start() -> Self {
        Stack { top: None, length: 0 }
    }

    // insert a node to the stack, creating a new node
    pub fn insert(&mut self, value: T) {

        // create a new node
        let mut new_node = Box::new(Node::new(value));

        // passing the next node to the current node
        new_node.next = self.top.take();

        self.top = Some(new_node); // put on the new node as top
        self.length += 1; // increment the length
    }

    // show the stack
    pub fn show(&self) {
        assert!(!self.is_empty());

        let mut node = &self.top; // get the reference of the top node

        // print the node list
        while let Some(current_node) = node {
            println!("[] -> {}", current_node.data);
            node = &current_node.next; // passing the current node to the top node
        }
    }

    // verify if the stack is empty
    pub fn is_empty(&self) -> bool {
        if self.top.is_none() { // if the top is none -> return true
            return true;
        } else {
            return false; // else -> return false
        }
    }

    // delete the top node from the stack
    pub fn pop(&mut self) -> T {
        assert!(!self.is_empty());

        let node = self.top.take().expect(""); // get the top node from the stack

        self.top = node.next; // passing the node to the top node -> Previous
        self.length -= 1; // decrement the length of the stack

        node.data // return the data of the node

    }

    // delete the linked nodes of the stack
    pub fn clear(&mut self) {
        self.top = None;
        self.length = 0;
    }

    // show the top of the stack
    pub fn top(&self) {
        assert!(!self.is_empty());

        if let Some(top) = &self.top {
            println!("\n#[Show top]");
            println!("[top] -> {}", top.data);
        }
    }
}