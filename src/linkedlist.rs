
type Link = Option<Box<Node>>;

#[derive(Debug,Clone)]
pub struct Node {
   pub elem: i32,
   pub next: Link,
}

impl Node {
    pub fn new(value: i32) ->Self {
        Node{
            elem: value,
            next: None,
        }
    }
}

pub fn print_value(head: &Node){

    let mut current = Some(head);

    while let Some(node) = current {


        println!("{}",node.elem);
        
        current = match &node.next {

            Some(next_node) => Some(next_node.as_ref()),
            None => None,
        };
    }
}

pub fn size_of_linkedlist(head: &Node){

    let mut current = Some(head);

    let mut size = 0;

    while let Some(node) = current {

        size +=1;

        current = match &node.next {
            Some(next_node) => Some(next_node.as_ref()),
            None => None
        }
        
    }
    println!("{}",size);
}


pub fn insert_at_begin (head: Node,value:i32) -> Link{

    let mut new_node = Node::new(value);

    new_node.next = Some(Box::new(head));

    print_value(&new_node);

    Some(Box::new(new_node))

    
}

pub fn insert_at_end(head: &mut Node, value:i32){
    
     let new_node = Node::new(value);

    let mut current = head;

    while current.next.is_some(){
        current =  current.next.as_mut().unwrap();
    }
   

    current.next =Some(Box::new(new_node));

}

pub fn insert_at_given_node(head: &mut Node, value: i32, insert_node: &Node){

   let mut current = head;
   let insert_value = insert_node.elem;

   loop {
      if current.elem == insert_value {
        let mut new_node = Node::new(value);
        new_node.next = current.next.take();

        current.next = Some(Box::new(new_node));
        break;
    }

    match current.next.as_mut() {
        Some(next) => {
            current = next},
        None => break,
    }
   }
}

pub fn revere_linkedList(head:  Node) ->Box<Node> {

    let mut prev = None;
    let mut current = Some(Box::new(head));

    while let Some(mut node) = current {
        let next = node.next;
        node.next = prev;
        prev = Some(node);
        current = next;
    }

    prev.unwrap()
}

pub fn middle_of_linkedList(head: &Node) -> &Node{

   let mut slow: &Node = head;
   let mut fast: Option<&Node> = Some(head);

   while let Some(f) = fast {
    if let Some(next_fast) = f.next.as_deref() {
        if let Some(next_next_fast) = next_fast.next.as_deref(){
            slow = slow.next.as_deref().unwrap();
            fast = Some(next_next_fast);
        }
        else {
        break;
    }
   } else {
    break;
   }
}
   
   slow
   }

   pub fn remove_nth_node_from_end(head: Node, nth_node: usize)-> Option<Box<Node>>{


    let mut dummy = Box::new(Node {
        elem: 0,
        next: Some(Box::new(head)),
    });

    let mut fast: *mut Box<Node> = &mut dummy;
    let mut slow: *mut Box<Node> = &mut dummy;

    for _ in 0..nth_node {
        unsafe {
            fast = (*fast).next.as_mut().unwrap();
        }
    }

    unsafe {
        while (*fast).next.is_some() {
            fast = (*fast).next.as_mut().unwrap();
            slow = (*slow).next.as_mut().unwrap();
        }

        let to_remove = (*slow).next.take();
        (*slow).next = to_remove.unwrap().next;
    }

    dummy.next.take()
   }

//    pub fn is_palindrome(head: Node) -> bool{
//          let middle = middle_of_linkedList(&head);
//          let reverse_half = revere_linkedList(head.clone());

         
             
         
//    }


// #[derive(Debug)]
// pub struct LinkedList {
//    pub head: Link,
// }

// impl LinkedList {
//     // 1. Create an empty list
//     pub fn new() -> Self {
//         LinkedList { head: None }
//     }

//     // 2. Push: Add an element to the front of the list
//     pub fn push(&mut self, elem: i32) {
//         let new_node = Box::new(Node {
//             elem: elem,
//             // We take ownership of the old head and set it as the next of the new node
//             next: self.head.take(), 
//         });

//         self.head = Some(new_node);
//     }
// }

//   pub fn print_value(mut head: &Link){
        
//         while let Some(node) = head {

//             println!("{}", node.elem);
//             head = &node.next;
            
//         }

//     }


//     pub fn size_of_linkedlist(mut head: &Link){

//         let mut size: i32 = 0;

//         while let Some(node) = head{

//             size +=1;

//             head = &node.next;

//         }
//         println!("{}",size);
//     }

//     pub fn insert_at_begin(mut head: Link, value:i32){
//         let cre_node = Box::new(Node{
//             elem:value,
//             next: head,
//         });
//         print_value(&Some(cre_node));
//     }

//     pub fn insert_at_end(head: &mut Link, value: i32){

//         let mut curr = head;
//         while let Some(node) = curr {

//             curr = &mut node.next;
            
//         }
//         let new_node = Box::new(Node{
//             elem:value,
//             next:None,
//         });

//         *curr = Some(new_node);

        
//     }