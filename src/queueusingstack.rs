struct MyQueue {
    in_stack: Vec<i32>,
    out_stack: Vec<i32>
}


impl MyQueue {
    fn new() -> Self{

        MyQueue{
            in_stack: Vec::new(),
            out_stack: Vec::new(),
        }

    }

    fn push(self, val:i32) {
        self.in_stack.push(val);
    }

    fn shift_stack(&mut self){
        
        if self.out_stack.is_empty(){
            while let Some(val) = self.in_stack.pop(){
                self.out_stack.push(val);
            }
        }
    }

    fn pop(&mut self) -> Option<i32> {
        self.shift_stack();
        self.out_stack.pop()
    }

    fn peek(&mut self) -> Option<i32>{
        self.shift_stack();
        self.out_stack.last().cloned()
    }

    fn empty(&self) -> bool {
        self.in_stack.is_empty() && self.out_stack.is_empty()
    }