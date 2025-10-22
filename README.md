Data structure and algorithm
-----------------------------
1 . Linked list 
----------------------
   > node { data: i32, next: Option<Box<Node>>}
   > linkedlist { head: Option<Box<Node>>}
   > reverse a liked list - prev = None , current = list.head , next = current.next ( in each iteration first change current.next to prev then prev eqaul to current, then current equal to next . contionus the iteration whrn current is eqaul to null)
   > middle of the linked list - first = list.head.next , second = list.head.next.next ( when the second reaches the none for even or second.next is none for odd , that means intex of first will be the mid or second mid    
-----------------------   
2 . Hashmap
------------------------
    * frequency counter ->map.entry(k).or_insert(0) += 1;
    * Look up -> map.get(&k);
    * measurment and relationship -> set.contains(&), collect 