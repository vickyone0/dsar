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

#[derive(Debug)]
pub struct TreeNode {
    pub val: i32,
   pub  left: Option<Box<TreeNode>>,
   pub right: Option<Box<TreeNode>>,
}

impl TreeNode {
   pub fn new(val: i32) -> Self {
        TreeNode { val, left: None, right: None }
    }
}

pub fn size(root: &Option<Box<TreeNode>>) -> usize {
    match root {
        Some(node) => 1 + size(&node.left)+ size(&node.right),
        None => 0,
    }
}

pub fn find_min_max(root: &Option<Box<TreeNode>>) -> Option<(i32, i32)>{

    match root {
        Some(node) => {
            let mut min_val = node.val;
            let mut max_val = node.val;

            if let Some((lmin, lmax)) = find_min_max(&node.left) {
                min_val = min_val.min(lmin);
                max_val = max_val.max(lmax);
            }
            if let Some((rmin, rmax)) = find_min_max(&node.right) {
                min_val = min_val.min(rmin);
                max_val = max_val.max(rmax);
            }

            Some((min_val, max_val))
        }
        None => None,
    }

}

pub fn find_path(root: &Option<Box<TreeNode>>, target:i32, path: &mut Vec<i32>) -> bool{

    if let Some(node) = root {

        path.push(node.val);

        if node.val == target {
            return true;
        }

        if find_path(&node.left, target, path) || find_path(&node.right, target, path){
            return true;
        }

        path.pop();
    }

    false
}