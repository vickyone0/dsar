use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    val: i32,
    next: Option<Rc<RefCell<Node>>>,
}

fn has_loop(head: Option<Rc<RefCell<Node>>>) -> bool {
    let mut slow = head.clone();
    let mut fast = head.clone();

    while let (Some(s), Some(f)) = (slow.clone(), fast.clone()) {

        slow =s.borrow().next.clone();

        fast = f.borrow().next.clone().and_then(|n| n.borrow().next.clone());

        if let (Some(ref s_ptr), Some(ref f_ptr)) = (&slow, &fast) {
            if Rc::ptr_eq(s_ptr, f_ptr) {
                return true;
            }
        }
    }

    false
}