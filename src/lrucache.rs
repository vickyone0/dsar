use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

type Link<K, V> = Option<Rc<RefCell<Node<K, V>>>>;


#[derive(Debug)]
struct Node<K, V> {
    key: K,
    val: V,
    prev: Option<Weak<RefCell<Node<K,  V>>>>,
    next: Link<K, V>,
}

#[derive(Debug)]
struct LRUCache<K, V> {
    map: HashMap<K, Rc<RefCell<Node<K, V>>>>,
    head: Link<K, V>,
    tail: Link<K, V>,
    capacity: usize,
}

impl<K: std::cmp::Eq + std::hash::Hash + Copy, V: Copy> LRUCache<K, V> {
    fn new(capacity: usize) -> Self {
        LRUCache {
            map: HashMap::new(),
            head: None,
            tail: None,
            capacity,
        }
    }

    fn put(&mut self, key: K, value: V) {
        if let Some(node_rc) = self.map.get(&key).cloned() {
            node_rc.borrow_mut().val = value;
            self.move_to_head(node_rc);

        }
        else {
            let new_node = Rc::new(RefCell::new(Node {
                key,
                val: value,
                prev: None,
            }));

            if self.map.len() == self.capacity {
                self.remove_lru();
            }

            self.add_to_head(new_node.clone());
            self.map.insert(key, new_node);

        }
    }

    fn move_to_head(&mut self, node_rc: Rc<RefCell<Node<K, V>>>) {
        self.remove_node(node_rc.clone());
        self.add_to_head(node_rc);
    }

    fn add_to_head(&mut self, node_rc: Rc<RefCell<Node<K, V>>>) {

        node_rc.borrow_mut().next = self.head.clone();
        node_rc.borrow_mut().prev = None;

        if let Some(old_head) = self.head.take() {
            old_head.borrow_mut().prev = Some(Rc::downgrade(&node_rc));
            self.head = Some(node_rc.clone());
            node_rc.borrow_mut().next = Some(old_head);
        } else {
            self.tail = some(node_rc.clone());
            self.head = Some(node_rc);
        }
    }

    fn remove_node(&mut self,node_rc: Rc<RefCell<NOde<K, V>>>) {
        let prev_opt = node_rc.borrow().prev.clone();
        let next_opt = node_rc.borrow().next.clone();

        if let Some(prev_weak) = prevv_opt {
            if let Some(prev_rc) = prev_weak.upgrade() {
                prev_rc.borrow_mut().next = next_opt.clone();
            }
        } else {
            self.head = next_opt.clone();
        }

        if let Some(next_rc) = next_opt {
            if let Some(prev_weak) = prev_opt {
                next_rc.borrow_mut().prev = Some(prev_weak);

            } else {
                next_rc.borrow_mut().prev = None;
            } 
        } else {
            self.tail = prev_opt.and_then(|w| w.upgrade());
        }
    }

    fn remove_lru(&mut self) {
        if let Some(tail_node) = self.tail.take() {
            let key = tail_node.borrow().key;
            self.remove_node(tail_node.clone());
            self.map.remove(&key);
        }
    }
}