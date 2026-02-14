// Implement a stack datastructure using a vector.

struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { items: Vec::new() }
    }

    // Push an element onto the top of the stack.
    fn push(&mut self, value: T) {
        self.items.push(value);
    }

    // Pop an element from the top of the stack.
    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    // Peek at top element of the stack without removing it.
    fn peek(&self) -> Option<&T> {
        self.items.last()
    }

}

fn main() {
    
    let mut stack: Stack<i32> = Stack::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    println!("Top of the stack: {:?}", stack.peek());
    stack.pop();
    println!("Top of the stack: {:?}", stack.peek());
}
