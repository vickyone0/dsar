
// #[derive(Debug, Clone)]
// pub struct Node {
//     pub value: i32,
//     pub next: Option<Box<Node>>,
// }

// impl Node{

//     pub fn new(value: i32) -> Box<Self>{
//         Box::new(Node {
//             value,
//             next: None,
//         })
//     }

//     pub fn with_next(value: i32, next: Option<Box<Node>>) -> Self {
//         Node {
//             value,
//             next,
//         }
//     }

//     pub fn value(v:i32) -> Node {
//         Node {
//             value: v,
//             next: None,
//         }
//     }

//     pub fn next(self,n: Option<Box<Node>>) -> Node {
//         Node {
//             value: self.value, // Placeholder value
//             next: n,
//         }
//     }

//     pub fn print_value(&self) {

//         let mut temp = self;
//         // while temp.value {
//         //     println!("Value: {}", temp.value);
//         //     // if let Some(next_node) = &self.next {
//         //     //     self.value = next_node.value;
//         //     // }
//         //     temp = temp.next.as_ref().unwrap();
//         // }
//         println!("Value: {}", temp.value);
//     }

// }


// pub fn reverse_linked_list(mut head: Option<Box<Node>>) -> Option<Box<Node>>
// {

//     let mut prev = None;
//     while let Some(mut node) = head {
//         head = node.next.take();
//         node.next = prev;
//         prev = Some(node);

//     }
//     prev
// }


// pub fn find_middle(head: &Option<Box<Node>>) -> Option<&Box<Node>> {

//     let mut slow = head.as_ref();
//     let mut fast = head.as_ref();

//     while let (Some(f), Some(ff)) = (fast, fast.as_ref().unwrap().next.as_ref()) {
//         fast = ff.next.as_ref();
//         slow = slow.unwrap().next.as_ref();
//     }

//     slow
// }


pub struct Node {
    data: i32,
    next: Option<Box<Node>>
}

pub struct Linkedlist {
    pub head: Option<Box<Node>>
}


impl Linkedlist {

    pub fn new() -> Self {
        Linkedlist { head: None}
    }

    pub fn push_front(&mut self, data: i32) {
        let new_node = Box::new(Node{
            data: data,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }
}


pub fn reverse_linked_list(mut list: Linkedlist) -> Linkedlist {
    let mut perv = None;
    let mut current = list.head.take();
    
    while let Some(mut node) = current {
                
                let mut next = node.next.take();

                node.next = perv;
                perv = Some(node); 
                current = next;
    }
    list.head = perv;
    list


}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val : i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val}
    }
}

// pub fn reverse_list(mut head: Option<Box<ListNOde>>) -> Option<Box<ListNode>> {
//     let mut prev = None;
//     while let Some(mut current) = head {
//         head = current.next.take();
//         current.next = prev;
//         prev = Some(current);
//     }
//     prev
// }