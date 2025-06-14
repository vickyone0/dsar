
#[derive(Debug, Clone)]
pub struct Node {
    pub value: i32,
    pub next: Option<Box<Node>>,
}

impl Node{

    pub fn new(value: i32) -> Box<Self>{
        Box::new(Node {
            value,
            next: None,
        })
    }

    pub fn with_next(value: i32, next: Option<Box<Node>>) -> Self {
        Node {
            value,
            next,
        }
    }

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
        // while temp.value {
        //     println!("Value: {}", temp.value);
        //     // if let Some(next_node) = &self.next {
        //     //     self.value = next_node.value;
        //     // }
        //     temp = temp.next.as_ref().unwrap();
        // }
        println!("Value: {}", temp.value);
    }

}