use dsar::linkedlist::Node;

fn main() {

    let node1 = Node{
        value: 1,
        next: None,
    };

    let new_node = Node{
        value: 2,
        next: Some(Box::new(node1)),
    };
    println!("{:?}", new_node);

    let node2 = Node::value(3);

    let node3 = Node::next(node2, Some(Box::new(new_node)));
    println!("{:?}", node3);
    node3.print_value();

}
    
