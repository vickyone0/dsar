// pub fn binary_search(arr: &[i32], tar: i32) -> Option<usize>{

//     let mut left = 0;
//     let mut right = arr.len();

//     while left < right{

//         let mid = left + (right - left)/2;

//         if arr[mid] == tar {
//             return Some(mid);
//         }
//         else if arr[mid] < tar {
//             left = mid+ 1;
            
//         }else{
//             right = mid;
//         }
//     }
//     None
// }

// pub fn rsa_min_value(arr: &[i32]) -> i32{

//     let mut left = 0;
//     let mut right = arr.len()-1;

//     while left < right {
//         let mid = left + (right - left)/2;

//         if arr[mid] > arr[right] {
//             left = mid + 1;
//         }
//         else {
//             right = mid;
//         }
//     }

//     arr[left]

// }

// #[derive(Debug)]
// pub struct TreeNode {

//     pub val: i32,
//     pub left: Option<Box<TreeNode>>,
//     pub right: Option<Box<TreeNode>>,
 
// }

// impl TreeNode {
//     pub fn new(val: i32) -> Self {
//         TreeNode{
//             val, left: None, right: None
//         }
//     }
// }

// pub fn insert_bst(root: &mut Option<Box<TreeNode>>, val: i32) {
//     match root {
//         Some(node) => {
//             if val < node.val {
//                 insert_bst(&mut node.left, val);
//             } else if val > node.val {
//                 insert_bst(&mut node.right, val);
//             }
//             // Do nothing if val == node.val (no duplicates)
//         }
//         None => {
//             *root = Some(Box::new(TreeNode::new(val)));
//         }
//     }
// }


#[derive(Debug)]
pub struct BinarySearchTree{
    val: i32,
    left: Option<Box<BinarySearchTree>>,
    right: Option<Box<BinarySearchTree>>
}

impl BinarySearchTree{
    
    pub fn new(val: i32) -> Self{
        BinarySearchTree{
            val : val,
            left : None,
            right: None,
        }
    }
}

pub fn sum_of_bst(root: &Option<Box<BinarySearchTree>>)-> i32 {
    
    match root{
        Some(node) => {
        sum_of_bst(node.left) + sum_of_bst(node.right) + node.val
    },
    None => 0,
}
}

pub fn size_of_bst(root: &Option<Box<BinarySearchTree>>) -> i32 {

    match root{
        Some(node) => {
            size_of_bst(node.left) + size_of_bst(node.right) + 1
        },
        None => 0,
    }
}

pub fn max_val_of_bst(root: &Option<Box<BinarySearchTree>>) -> Option<i32> {

    match root{
        Some(node) => {
            match &node.right{
                Some(_) => max_val_of_bst(&node.right),
                None => Some(node.val),
            }
        },
        None => None,
    }
}

pub fn min_val_of_bst (root: &Option<Box<BinarySearchTree>>) -> Option<i32> {

    match root{
        Some(node) => {
            match(&node.left){
                Some(_) => min_val_of_bst(&node.left),
                None => Some(node.val),
            }
        },
        None => None,
    }
}

pub fn is_present(root: &Option<Box<BinarySearchTree>>, key: i32) -> bool{


    match root{
        Some(node) => {
            if node.val == key {
                true
            }else if node.val > key {
                is_present(&node.left,key)
            }else {
                is_present(&node.right,key)
            }
        },
        None => false,
    }

}


pub fn lca_of_bst(root: &Option<Box<BinarySearchTree>>, n1: i32, n2: i32) -> Option<i32> {

    match root {
        Some(node) => {
            if node.val > n1 && node.val > n2 {
                lca_of_bst(&node.left, n1,n2)
            }else if node.val < n1 && node.val < n2 {
                lca_of_bst(&node.right, n1, n2)
            }else {
                Some(node.val)
            }
        },
        None => None,
    }
}

pub fn is_bst(root: &Option<Box<BinarySearchTree>>) ->bool {

        is_bst_helper(root,i32::MIN, i32::MAX)
}

pub fn is_bst_helper(root: &Option<Box<BinarySearchTree>>, lower_limit: i32, upper_limit: i32) -> bool{

    match root{
        Some(node) => {
            if node.val > upper_limit || node.val < lower_limit {
                false
            }
            is_bst_helper(&node.left,lower_limit,node.val) && is_bst_helper(&node.right,node.val ,upper_limit)
        },
        None => true,
    }
}

pub fn insert_on_bst(root: Option<Box<BinarySearchTree>>, key: i32) -> Option<Box<BinarySearchTree>> {

    match root {
        Some(node) => {
            if node.val > key {
                node.left = insert_on_bst(node.left);
            }else {
                node.right = insert_on_bst(node.right);
            }
            Some(node)
        },
        None => Some(Box::new(BinarySearchTree::new(key))),
    }
}

pub fn remove_on_bst(root: Option<Box<BinarySearchTree>>, key: i32) -> Option<Box<BinarySearchTree>> {

    match root {
        Some(mut node) => {
            if node.val > key {
                node.left = remove_on_bst(node.left,key);
                return Some(node);
            }else if node.val < key {
                node.right = remove_on_bst(node.right, key);
                return Some(node);
            }

            if node.left.is_none() && node.right.is_none() {
                return None;
            }
            if node.left.is_none() {
                return node.right;
            }

            if node.right.is_none() {
                return node.left;
            }

            let max_val = max_val_of_bst(&node.left);

            node.val = max_val;

            node.left = remove_on_bst(node.left,max_val);

            Some(node)
        },
        None => None,
    }
}