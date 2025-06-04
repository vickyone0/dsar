
#[derive(Debug)]
pub struct Node {
    pub value: i32,
    pub next: Option<Box<Node>>,
}

impl Node{

    pub fn value(v:i32) -> Node {
        Node {
            value: v,
            next: None,
        }
    }

    pub fn next(self,n: Option<Box<Node>>) -> Node {
        Node {
            value: self.value, // Placeholder value
            next: n,
        }
    }

    pub fn print_value(&self) {

        let mut temp = self;
        while temp.next.is_some() {
            println!("Value: {}", self.value);
            // if let Some(next_node) = &self.next {
            //     self.value = next_node.value;
            // }
            temp = temp.next.as_ref().unwrap();
        }
    }

}