// pub fn stacked() {

//     let mut stack: Vec<i32> = Vec::new();

//     stack.push(10);
//     stack.push(20);
//     stack.push(30);

//     println!("Top element: {:?}", stack.last());

//     println!("popped: {:?}", stack.pop());
    


// }

struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { items: Vec::new()}
    }

    fn push(&mut self, item: T) {
        self.items.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    fn peek(&self) -> Option<&T> {
        self.items.last()
    }

    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    fn size(&self) -> usize {
        self.items.len()
    }
}