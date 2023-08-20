mod node;
mod stack;

use stack::Stack;

fn main() {
    let mut stack: Stack<i32> = Stack::start();

    stack.insert(7);

    println!("\n#[Current length] -> {}", stack.length);
    println!("#[Current stack]");
    stack.show();

    stack.top();

    stack.pop();

    stack.insert(8);
    stack.insert(5);

    stack.pop();

    stack.insert(9);

    println!("\n#[Current length] -> {}", stack.length);
    println!("#[Current stack]");
    stack.show();

    stack.top();

    stack.clear();

    println!("\n#[Current length] -> {}", stack.length);

}
