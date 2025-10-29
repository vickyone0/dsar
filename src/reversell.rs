#[derive(Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { val, next: None}
    }
}

pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode> {
    let mut prev: Option<Box<ListNode>> = None;
    while let Some(mut current) = head {
        head = current.next.take();
        current.next = prev;
        prev = Some(current);
    }
    prev
}

pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

    let mut slow = head.as_ref();
    let mut fast = head.as_ref();

    while let (Some(f), Some(next_fast)) = (fast, fast.as_ref().next.as_ref()) {
        slow = slow.and_then(|s| s.next.as_ref());
        fast = next_fast.next.as_ref();
    }
    slow.cloned()
}

pub fn merge_two_lists(
    mut list1: Option<Box<ListNode>>,
    mut list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode::new(0));
    let mut tail = &mut dummy;

    while let (Some(mut node1), Some(mut node2)) = (list1, list2) {
        if node1.val < node2.val {
            list1 = node1.next.take();
            tail.next = Some(node1);
        } else {
            list2 = node2.next.take();
            tail.next = Some(node2);
        }
        tail = tail.next.as_mut().unwrap();
    }

    tail.next = if list1.is_some() {list1} else { list2};
    dummy.next
}

pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode {val: 0, next:head});
    let mut fast = dummmy.as_ref();
    let mut slow = dummy.as_mut();

    for _ in 0..n {
        if let Some(node) = fast.next.as_ref() {
            fast = node;
        } else {
            return dummy.next;
        }
    }

    while let Some(node) = fast.next.as_ref() {
        fast = node;
        slow = slow.next.as_mut().unwrap();
    }

    let next = slow.next.as_mut().and_then(|n| n.next.take());
    slow.next = next;

    dummmy.next
}