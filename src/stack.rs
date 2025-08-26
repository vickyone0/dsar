pub fn stacked() {

    let mut stack: Vec<i32> = Vec::new();

    stack.push(10);
    stack.push(20);
    stack.push(30);

    println!("Top element: {:?}", stack.last());

    println!("popped: {:?}", stack.pop());
    


}