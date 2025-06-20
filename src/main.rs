// use dsar::linkedlist::Node;
// use dsar::binarytree::BinaryTree;

// use std::sync::{Arc, Mutex};
// use std::thread;

// fn main() {

//     let counter = Arc::new(Mutex::new(0));
//     let mut handles = vec![];


//     for _ in 0..10 {
//         let counter = Arc::clone(&counter);

//         let handle = thread::spawn(move || {
//             let mut num = counter.lock().unwrap();
//             *num += 1;
//         });
//         handles.push(handle);
//     }

//     for handle in handles {
//         handle.join().unwrap();
//     }

//     println!("Result: {}", *counter.lock().unwrap());



//     let mut tree = BinaryTree::new();
//     tree.insert(10);
//     tree.insert(5);
//     tree.insert(15);

//     println!("In-order traversal:");
//     tree.inorder_traversal();


//     // let node_a = Node::new(5);
//     // let node_b = Node::new(10);
//     // let node_c = Node::new(15);
//     // let node_d = Node::new(20);

//     // node_c.next(Some(node_d));
//     // node_b.next(Some(node_c.clone()));

//     // let node1 = Node{
//     //     value: 1,
//     //     next: None,
//     // };

//     // let new_node = Node{
//     //     value: 2,
//     //     next: Some(Box::new(node1)),
//     // };
//     // //println!("{:?}", new_node);

//     // let node2 = Node::value(3);

//     // let node3 = Node::next(node2, Some(Box::new(new_node)));
//     // println!("{:?}", node3);
//     // node3.print_value();

// }
    

use reqwest;
use tokio;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error>{
    let url = "https://api.github.com/users/rust-lang";
    let response = reqwest::get(url).await?.json::<serde_json::Value>().await?;
    println!("GitHub user: {:?}", response["login"]);
    Ok(())
}
