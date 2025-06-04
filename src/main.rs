use dsar::linkedlist::Node;

fn main() {


    let node_a = Node::new(5);
    let node_b = Node::new(10);
    let node_c = Node::new(15);
    let node_d = Node::new(20);

    node_c.next(Some(node_d));
    node_b.next(Some(node_c.clone()));

    let node1 = Node{
        value: 1,
        next: None,
    };

    let new_node = Node{
        value: 2,
        next: Some(Box::new(node1)),
    };
    //println!("{:?}", new_node);

    let node2 = Node::value(3);

    let node3 = Node::next(node2, Some(Box::new(new_node)));
    println!("{:?}", node3);
    node3.print_value();

}
    
