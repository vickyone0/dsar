pub fn binary_search(arr: &[i32], tar: i32) -> Option<usize>{

    let mut left = 0;
    let mut right = arr.len();

    while left < right{

        let mid = left + (right - left)/2;

        if arr[mid] == tar {
            return Some(mid);
        }
        else if arr[mid] < tar {
            left = mid+ 1;
            
        }else{
            right = mid;
        }
    }
    None
}

pub fn rsa_min_value(arr: &[i32]) -> i32{

    let mut left = 0;
    let mut right = arr.len()-1;

    while left < right {
        let mid = left + (right - left)/2;

        if arr[mid] > arr[right] {
            left = mid + 1;
        }
        else {
            right = mid;
        }
    }

    arr[left]

}

#[derive(Debug)]
pub struct TreeNode {

    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
 
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        TreeNode{
            val, left: None, right: None
        }
    }
}

pub fn insert_bst(root: &mut Option<Box<TreeNode>>, val: i32) {
    match root {
        Some(node) => {
            if val < node.val {
                insert_bst(&mut node.left, val);
            } else if val > node.val {
                insert_bst(&mut node.right, val);
            }
            // Do nothing if val == node.val (no duplicates)
        }
        None => {
            *root = Some(Box::new(TreeNode::new(val)));
        }
    }
}
