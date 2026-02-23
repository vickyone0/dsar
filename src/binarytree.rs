// use std::cmp::Ordering;


// #[derive(Debug)]
// struct Node<T: Ord> {
//     value: T,
//     left: Option<Box<Node<T>>>,
//     right: Option<Box<Node<T>>>,
// }

// impl<T: Ord> Node<T> {
//     fn new(value: T) -> Self {
//         Node {
//             value,
//             left: None,
//             right: None,
//         }
//     }
// }

// #[derive(Debug)]
// pub struct BinaryTree<T: Ord> {
//     root: Option<Box<Node<T>>>,
// }

// //insertion 

// impl<T: Ord + std::fmt::Debug> BinaryTree<T> {
//     pub fn new() -> Self {
//         BinaryTree { root: None }
//     }

//     pub fn insert(&mut self, value: T) {
//         match self.root {
//             Some(ref mut node) => {
//                 Self::insert_recursive(node, value);
//             }
//             None => {
//                 self.root = Some(Box::new(Node::new(value)));
//             }
//         }
//     }


//     fn insert_recursive(node: &mut Box<Node<T>>, value: T) {

//         match value.cmp(&node.value) {
//             Ordering::Less => {
//                 if let Some(left) = &mut node.left {
//                     Self::insert_recursive(left, value);
//                 } else {
//                     node.left = Some(Box::new(Node::new(value)));
//                 }
//             }
//             Ordering::Greater => {
//                 if let Some(right) = &mut node.right {
//                     Self::insert_recursive(right, value);
//                 } else {
//                     node.right = Some(Box::new(Node::new(value)));
//                 }
//             }
//             Ordering::Equal => (),
//         }
//     }

//     pub fn inorder_traversal(&self) {
//         fn traverse<T: Ord + std::fmt::Debug>(node: &Option<Box<Node<T>>>) {
//             if let Some(n) = node {
//                 traverse(&n.left);
//                 println!("{:?}", n.value);
//                 traverse(&n.right);
//     }
// }
//         traverse(&self.root);
//     }
// }

// pub struct Node{
//     left: Node,
//     right: Node,
//     value: i32
// }

// impl Node {

//     pub fn new(){
        

//     }
    
// }

// #[derive(Debug)]
// pub struct TreeNode {
//     pub val: i32,
//    pub  left: Option<Box<TreeNode>>,
//    pub right: Option<Box<TreeNode>>,
// }

// impl TreeNode {
//    pub fn new(val: i32) -> Self {
//         TreeNode { val, left: None, right: None }
//     }
// }

// pub fn size(root: &Option<Box<TreeNode>>) -> usize {
//     match root {
//         Some(node) => 1 + size(&node.left)+ size(&node.right),
//         None => 0,
//     }
// }

// pub fn find_min_max(root: &Option<Box<TreeNode>>) -> Option<(i32, i32)>{

//     match root {
//         Some(node) => {
//             let mut min_val = node.val;
//             let mut max_val = node.val;

//             if let Some((lmin, lmax)) = find_min_max(&node.left) {
//                 min_val = min_val.min(lmin);
//                 max_val = max_val.max(lmax);
//             }
//             if let Some((rmin, rmax)) = find_min_max(&node.right) {
//                 min_val = min_val.min(rmin);
//                 max_val = max_val.max(rmax);
//             }

//             Some((min_val, max_val))
//         }
//         None => None,
//     }

// }

// pub fn find_path(root: &Option<Box<TreeNode>>, target:i32, path: &mut Vec<i32>) -> bool{

//     if let Some(node) = root {

//         path.push(node.val);

//         if node.val == target {
//             return true;
//         }

//         if find_path(&node.left, target, path) || find_path(&node.right, target, path){
//             return true;
//         }

//         path.pop();
//     }

//     false
// }

use std::{collections::VecDeque, i32::{self, MIN}};



#[derive(Debug)]
pub struct BinaryTree {
     pub  val: i32,
     pub  left: Option<Box<BinaryTree>>,
     pub  right: Option<Box<BinaryTree>>,
     
}

impl BinaryTree {
   pub fn new(val:i32) -> Self{

   

        BinaryTree{
            val : val,
            left: None,
            right: None,
        }

        

    }
}

 pub fn size_of_binary_tree(root: Option<Box<BinaryTree>>) -> i32 {


    match root {
         Some(node) => 1 + size_of_binary_tree(node.left) + size_of_binary_tree(node.right),
         None => 0,
    }
}

pub fn max_in_btree(root: &Option<Box<BinaryTree>>) -> i32{

    match root {
        Some(node)=>{
            let l_max = max_in_btree(&node.left);
            let r_max = max_in_btree(&node.right);
            std::cmp::max(node.val, std::cmp::max(l_max, r_max))

            
        },
        None => i32::MIN,
    }
}

pub fn sum_of_btree(root: &Option<Box<BinaryTree>>) -> i32 {
   
    
    match root{
        Some(node) => {

            node.val + sum_of_btree(&node.left) + sum_of_btree(&node.right)
            

        },

        None => 0,
    }

}

pub fn is_present(root: &Option<Box<BinaryTree>>, value: i32 ) -> bool{
    
    match root {
        Some(node) => {


            node.val == value || is_present(&node.left, value) || is_present(&node.right, value)
        
        },

        None => false
    }

}

pub fn find_path_of_node(root: &Option<Box<BinaryTree>>, value: i32, path: &mut Vec<i32>) -> bool {

   

    match root {
        Some(node) => {
             path.push(node.val);

            // If found → done
            if node.val == value {
                return true;
            }

            // Search subtrees
            if find_path_of_node(&node.left, value, path)
                || find_path_of_node(&node.right, value, path)
            {
                return true;
            }

             path.pop();
            false

        },
        None => {

            false
        }
    
}
}

pub fn l_c_a(root: &Option<Box<BinaryTree>>, k1: i32, k2: i32) -> Option<i32> {


    let mut path1 = Vec::new();
    let mut path2 = Vec::new();
    
    if !find_path_of_node(&root, k1, &mut path1) || !find_path_of_node(&root, k2, &mut path2){
        return None;
    }

    let mut i = 0;

    let mut lca = None;

    println!("{:?}",path1);
    println!("{:?}",path2);

    while i < path1.len() && i < path2.len() {

        if path1[i]== path2[i] {
            lca = Some(path1[i]);
        } else {
            break;
            
        }

        i +=1;
    }

    lca
}

pub fn in_order(root: &Option<Box<BinaryTree>>) {


    match root {
        Some(node) => {
            in_order(&node.left);
            println!("{}",node.val);
            in_order(&node.right);
        },
        None => ()
    }

}

pub fn pre_order(root: &Option<Box<BinaryTree>>){

    match root {
        Some(node) => {
            println!("{}",node.val);
            pre_order(&node.left);
            pre_order(&node.right);
        },
        None => ()
    }
}

pub fn post_order(root: &Option<Box<BinaryTree>>){

    match root{
        Some(node) => {
            post_order(&node.left);
            post_order(&node.right);
            println!("{}",node.val);
        },
        None => ()
    }
}

pub fn level_order(root: &Option<Box<BinaryTree>>){

    let mut q = VecDeque::new();

    if let Some(node) = root {
        q.push_back(node);
    }
    while let Some(node) = q.pop_front() {

        println!("{}",node.val);

        if let Some(left) =&node.left {
            q.push_back(left);
        }

        if let Some(right) = &node.right {
            q.push_back(right);
        }
    }
}