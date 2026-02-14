use std::collections::VecDeque;

struct Queue<T> {
    items: VecDeque<T>,
}

impl<T> Queue<T> {
    fn new() -> Self {
        Queue { 
            items: VecDeque::new() 
        }
    }

    fn push(&mut self, value: T) {
        self.items.push_back(value);
    }

    fn pop(&mut self) -> Option<T> {
        self.items.pop_front()
    }

    fn peek(&self) -> Option<&T> {
        self.items.front()
    }
}

fn main() {
    let mut queue: Queue<i32> = Queue::new();

    queue.push(1);
    queue.push(2);
    queue.push(3);

    println!("Front of the queue: {:?}", queue.peek());
    queue.pop();
    println!("Front of the queue: {:?}", queue.peek());
}
