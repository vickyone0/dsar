use dsar::linkedlist::{Node, insert_at_begin, insert_at_end, insert_at_given_node, middle_of_linkedList, print_value, remove_nth_node_from_end, revere_linkedList, size_of_linkedlist};
use dsar::binarytree::{BinaryTree, find_path_of_node, in_order, is_present, l_c_a, level_order, max_in_btree, post_order, pre_order, size_of_binary_tree, sum_of_btree};
use dsar::binarysearchtree::BinarySearchTree;
fn main() {
    
    // let mut a = Node::new(1);
    // let mut b = Node::new(2);
    // let mut c = Node::new(3);
    // let mut d = Node::new(4);
    // let mut e = Node::new(5);

    // d.next = Some(Box::new(e));
    // c.next =Some(Box::new(d));
    // b.next =Some(Box::new(c));
    // a.next =Some(Box::new(b.clone()));


    let mut a = BinaryTree::new(1);
    let mut b = BinaryTree::new(2);
    let mut c = BinaryTree::new(3);
    let mut d = BinaryTree::new(7);
     
     a.left = Some(Box::new(b));
     c.left = Some(Box::new(d));
     a.right = Some(Box::new(c));
      
        
   let root = Some(Box::new(a));

    //let size = size_of_binary_tree(root);

    let max_val = max_in_btree(&root);
    let is_present = is_present(&root, 9);

    

    //let mut path = Vec::new();

    //find_path_of_node(&root, 7, &mut path);
   
  // let lca = l_c_a(&root, 7, 3);
   //in_order(&root);
   //pre_order(&root);
   //post_order(&root);

   level_order(&root);

    //println!("{:?}",lca);


    //print_value(&a);
    //size_of_linkedlist(&a);
    //insert_at_begin(a, 0);
    //insert_at_end(&mut a, 5);
    //let insert_after = b;
    //insert_at_given_node(&mut a, 3, &insert_after);
    //let mut revesed_head = revere_linkedList(a);
    //print_value(&revesed_head);
    // let middle = middle_of_linkedList(&a);
    // let removed_node = remove_nth_node_from_end(3, nth_node)
    // println!("{}",middle.elem);

    

    //size_of_linkedlist(&list.head);

    //insert_at_begin(list.head, 6);

    //insert_at_end(&mut list.head, 0);


    
}