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
mod bubblesort;
use crate::bubblesort::bubble_sort;
mod selectionsort;
use crate::selectionsort::selection_sort;
mod twosumsorted;
use crate::twosumsorted::two_sum_sorted;
mod maxvoliume;
use crate::maxvoliume::max_volume;
mod sildingwindow;
use crate::sildingwindow::max_sum_subarray;
mod recursion;
use crate::recursion::{recursion,recursion_power_logn};


#[tokio::main]
async fn main() -> Result<(), reqwest::Error>{
    // let url = "https://api.github.com/users/rust-lang";
    // let response = reqwest::get(url).await?.json::<serde_json::Value>().await?;
    // println!("GitHub user: {:?}", response["login"]);
    

    println!("{}",recursion(2));

     println!("{}",recursion_power_logn(5,2));

    let  arr= [5,50,10,32,34];

    let len = 3;

    //println!("max volume is {}", max_volume(&arr));
     println!("max sub array of length {} is {:?}",len, max_sum_subarray(&arr, len));
    let target = 15;

    //let sorted_array = bubble_sort(& mut arr);


    let output = two_sum_sorted(&arr, target);


    println!("{:?}",output);



    Ok(())
}
