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
use crate::twosumsorted::{two_sum, two_sum_sorted};
mod maxvoliume;
use crate::maxvoliume::max_volume;
mod sildingwindow;
use crate::sildingwindow:: sliding_window_subarray_4;
mod recursion;
use crate::recursion::{recursion,recursion_power_logn};
mod backtracking;
use crate::backtracking::n_queen;
mod binarysearch;
use crate::binarysearch::{binary_search, rsa_min_value,insert_bst,TreeNode};
mod mergesort;
use crate::mergesort::merge_sort;
mod quicksort;
use crate::quicksort::quick_sort;
mod insertionsort;
use crate::insertionsort::insertion_sort;
mod linkedlist;
//use crate::linkedlist::{Node,reverse_linked_list,find_middle};
mod stack;
use crate::stack::stacked;
mod binarytree;
use crate::binarytree::{size, find_min_max, find_path};
mod hashmap;
use crate::hashmap::{has_zero_subarray, count_zero_sum_subarrays,longest_zero_sum_subarray, longest_consecutive, hash_insert};
mod graph;
use crate::graph::{num_of_island,oranges_rotting};
mod pipeline;
use crate::pipeline::pipe_line;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error>{
    //panic!("fji idi");
    // pipe_line();
    // // let url = "https://api.github.com/users/rust-lang";
    // // let response = reqwest::get(url).await?.json::<serde_json::Value>().await?;
    // // println!("GitHub user: {:?}", response["login"]);
    // let grid = vec![
    //     vec![2, 1, 1],
    //     vec![1, 1, 0],
    //     vec![0, 1, 1],
    // ];
    // println!("Minutes to rot all: {}", oranges_rotting(grid)); 


    // let grid = vec![
    //     vec!['1','1','0','0','0'],
    //     vec!['1','1','0','0','0'],
    //     vec!['0','0','1','0','0'],
    //     vec!['0','0','0','1','1'],
    // ];
    // println!("Number of islands: {}", num_of_island(grid));

     let mut arr = [4, 5, 6, 7, 8];
     hash_insert();

     println!("max value of 4 subarray is : {}", sliding_window_subarray_4(&arr));

     println!("max volume : {}", max_volume(&arr));
     println!("target index is {:?}", two_sum_sorted(&arr, 7));
     quick_sort(&mut arr);
     println!("quick sort: {:?}",arr);
//     println!("Has zero-sum subarray? {}", has_zero_sum_subarray(&arr));
//     println!("Count of zero-sum subarrays: {}", count_zero_sum_subarrays(&arr));
//     let arr2 = [1, 2, 3];
//     println!("Has zero-sum subarray? {}", has_zero_sum_subarray(&arr2));
//      println!(
//         "Length of longest zero-sum subarray: {}",
//         longest_zero_sum_subarray(&arr)
//     );

//     println!(
//         "Longest consecutive sequence length: {}",
//         longest_consecutive(&arr)
//     );


//     let mut root: Option<Box<TreeNode>> = None;

//     insert_bst(&mut root, 10);
//     insert_bst(&mut root, 5);
//     insert_bst(&mut root, 15);
//     insert_bst(&mut root, 3);
//     insert_bst(&mut root, 7);

//     println!("{:#?}", root);

//     let val = 6;
//     insert_bst(&mut root, val);
//      println!("{:#?}", root);

//     // let mut root = Some(Box::new(TreeNode::new(1)));
//     // root.as_mut().unwrap().left = Some(Box::new(TreeNode::new(2)));
//     // root.as_mut().unwrap().right = Some(Box::new(TreeNode::new(3)));
//     // root.as_mut().unwrap().left.as_mut().unwrap().left = Some(Box::new(TreeNode::new(4)));


//     // println!("Tree size: {}", size(&root));

//     // if let Some((min_val, max_val)) = find_min_max(&root) {
//     //     println!("Min: {}, Max: {}", min_val, max_val);
//     // }
    
//     // let target = 4;
//     // let mut path= Vec::new();
//     // // if find_path(&root, target, &mut path) {
//     //     println!("Path to {}: {:?}", target, path); // Path to 7: [10, 5, 7]
//     // } else {
//     //     println!("Node {} not found.", target);
//     // }

//     // stacked();
//     // let n = 7;
//     // let solutions = n_queen(n);

//     // println!("Total solutions for {}-Queens: {}", n, solutions.len());
//     // for sol in solutions {
//     //     for row in sol {
//     //         println!("{}", row);
//     //     }
//     //     println!();
//     // }

//     // println!("{}",recursion(2));

//     //  println!("{}",recursion_power_logn(5,2));

     let   arr= [1, -1, 2, -2, 3, -3, 1];
     println!("count of subarray of zero: {}",count_zero_sum_subarrays(&arr));

//     // let arr1 = vec![38,83,7,7,3,8];

//     // println!("binary search : {:?}", rsa_min_value(&arr));
      
//     // println!("merge sort : {:?}", merge_sort(arr1));
//     // quick_sort(&mut arr);
//     // println!("quick sort : {:?}",arr);
//     // insertion_sort(&mut arr);
     
//     // println!("insertion sort : {:?}",arr);
    

  
//     // let len = 3;

//     // //println!("max volume is {}", max_volume(&arr));
//     //  println!("max sub array of length {} is {:?}",len, max_sum_subarray(&arr, len));
//     // let target = 15;

//     // //let sorted_array = selection_sort(& mut arr);


//     // let output = two_sum_sorted(&arr, target);


//     // println!("{:?}",output);



    

//     let mut head = Some(Node::new(1));
//     head.as_mut().unwrap().next = Some(Node::new(2));
//     head.as_mut().unwrap().next.as_mut().unwrap().next = Some(Node::new(3));

//     println!("Original: {:?}", head);
//     ///let reversed = reverse_linked_list(head);
//    // println!("Reversed: {:?}", reversed);
    
//     if let Some(middle) = find_middle(&head){
//         println!("middle node: {}",middle.value);

//     }
    Ok(())

    
}


