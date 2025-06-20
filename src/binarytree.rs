use std::cmp::Ordering;


#[derive(Debug)]
struct Node<T: Ord> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: Ord> Node<T> {
    fn new(value: T) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }
}

#[derive(Debug)]
pub struct BinaryTree<T: Ord> {
    root: Option<Box<Node<T>>>,
}

//insertion 

impl<T: Ord + std::fmt::Debug> BinaryTree<T> {
    pub fn new() -> Self {
        BinaryTree { root: None }
    }

    pub fn insert(&mut self, value: T) {
        match self.root {
            Some(ref mut node) => {
                Self::insert_recursive(node, value);
            }
            None => {
                self.root = Some(Box::new(Node::new(value)));
            }
        }
    }


    fn insert_recursive(node: &mut Box<Node<T>>, value: T) {

        match value.cmp(&node.value) {
            Ordering::Less => {
                if let Some(left) = &mut node.left {
                    Self::insert_recursive(left, value);
                } else {
                    node.left = Some(Box::new(Node::new(value)));
                }
            }
            Ordering::Greater => {
                if let Some(right) = &mut node.right {
                    Self::insert_recursive(right, value);
                } else {
                    node.right = Some(Box::new(Node::new(value)));
                }
            }
            Ordering::Equal => (),
        }
    }

    pub fn inorder_traversal(&self) {
        fn traverse<T: Ord + std::fmt::Debug>(node: &Option<Box<Node<T>>>) {
            if let Some(n) = node {
                traverse(&n.left);
                println!("{:?}", n.value);
                traverse(&n.right);
    }
}
        traverse(&self.root);
    }
}