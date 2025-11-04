struct MinStack {
    stack: Vec<i32>,
    min_stack: Vec<i32>,
}

impl MinStack {
    fn new() -> Self {
        MinStack {
            stack: Vec::new(),
            min_stack: Vec::new(),
        }
    }

    fn push(&mut self, val: i32) {
        self.stack.push(val);

        if let Some(&min_val) = self.min_stack.last() {
            self.min_stack.push(min_val.min(val));
        } else {
            self.min_stack.push(val);
        }
    }

    fn pop(&mut self) {
        self.stack.pop();
        self.min_stack.pop();
    }

    fn top(&self) -> Option<i32> {
        self.tack.last().cloned()
    }

    fn get_min(&self) -> Option<i32> {
        self.min_stack.last().cloned()
    }
}