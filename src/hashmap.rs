

// pub fn has_zero_sum_subarray(arr: &[i32]) -> bool {


//     let mut prefix_sum = 0;

//     let mut seen = HashMap::new();

//     seen.insert(0, false);

//     for &num in arr {
//         prefix_sum += num;

//         if seen.contains_key(&prefix_sum) {
//             return  true;
//         }
//         seen.insert(prefix_sum, false);
//     }
//     false

// }

// pub fn count_zero_sum_subarrays(arr: &[i32]) -> i32 {
//     let mut prefix_sum = 0;
//     let mut count = 0;
//     let mut freq = HashMap::new();

//     freq.insert(0,1);

//     for &num in arr {
//         prefix_sum += num;

//         if let Some(&c) = freq.get(&prefix_sum){
//             count += c;

//         }

//         *freq.entry(prefix_sum).or_insert(0) +=1;
//     }
//     count
// }

pub fn longest_zero_sum_subarray(arr: &[i32]) -> usize {
    let mut prefix_sum = 0;
    let mut max_len = 0;
    let mut map = HashMap::new();

    map.insert(0, -1);

    for (i, &num) in arr.iter().enumerate() {
        prefix_sum += num;

        if let Some(&first_index) = map.get(&prefix_sum) {

            let length = i as isize - first_index;
            if length as usize > max_len {
                max_len = length as usize;
            }

        }
        else {
            map.insert(prefix_sum, i as isize);
        }
    }
    max_len
}

// pub fn longest_consecutive(nums: &[i32]) -> usize {
//     let mut map: HashMap<i32, bool> = HashMap::new();

//     // Fill map with numbers, marked as not visited
//     for &num in nums {
//         map.insert(num, false);
//     }

//     let mut longest = 0;

//     for &num in nums {
//         // Only start from numbers that have not been visited
//         if let Some(visited) = map.get_mut(&num) {
//             if *visited {
//                 continue; // Skip if already part of a counted sequence
//             }

//             *visited = true;

//             // Expand in both directions
//             let mut length = 1;
//             let mut left = num - 1;
//             let mut right = num + 1;

//             // Expand left
//             while let Some(v) = map.get_mut(&left) {
//                 if *v {
//                     break;
//                 }
//                 *v = true;
//                 length += 1;
//                 left -= 1;
//             }

//             // Expand right
//             while let Some(v) = map.get_mut(&right) {
//                 if *v {
//                     break;
//                 }
//                 *v = true;
//                 length += 1;
//                 right += 1;
//             }

//             longest = longest.max(length);
//         }
//     }

//     longest
// }

pub fn has_zero_subarray(arr: &[i32]) -> bool {
    let mut prefered_sum = 0;

    let sum: HashMap<i32, usize> = HashMap::new();


    for (i, &num) in arr.iter().enumerate(){
        if i == arr.len()-3 {
            return false;
        }
        //[5,50,-55,32,34

        prefered_sum += num;

        if prefered_sum ==0 && i >= 2 {
            return true;
        }

        if i > 2 && sum.contains_key(&prefered_sum) {
            return true;

        }
    }

    false
}

pub fn count_zero_sum_subarrays(arr:&[i32]) -> i32{

    let mut prefer_sum = 0;
    let mut count = 0;
    let mut sum_counter = HashMap::new();

    sum_counter.insert(0, 1);

    for &num in arr.iter(){
        prefer_sum += num;

        if let Some(freq) = sum_counter.get(&prefer_sum) {
            count += freq;
            
        }

        *sum_counter.entry(prefer_sum).or_insert(0)+=1;



    }

    count


}


use std::collections::{HashMap, HashSet};

pub fn hash_insert() {
    let mut map: HashMap<String, i32> = HashMap::new();

    map.insert("apple".to_string(), 5);
    map.insert("banana".to_string(), 3);
    map.insert("apple".to_string(), 9);

    println!("{:?}", map);


    if let Some(v) = map.get("apple") {
        println!("apple count = {}", v);
    } else {
        println!(" No apple");
    }

    if let Some(v) = map.get_mut("banana") {
        *v +=2;
    }
    map.remove("apple");
    
    println!("{:?}", map);

    println!("{}", map.contains_key("apple"));

    map.entry("mango".to_string()).or_insert(7);
    
    for (k,v) in &map {
        println!("{} => {}", k, v);
    }
    

    
    println!("Length: {}", map.len());
println!("Empty? {}", map.is_empty());
map.clear();

let fruits = vec![("apple",3), ("banana",5)];
let maps: HashMap<_, _> = fruits.into_iter().collect();
}

//lookup problem in hashmap/ hashset (3)--------------------------------

pub fn two_sums(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();

    for (i, &num) in nums.iter().enumerate(){
        let complement = target - num;
         if let Some(&j) = map.get(&complement) {
            return vec![i as i32, j as i32];
         }
         map.insert(complement, i);
    }
    return vec![];
}

pub fn length_of_longest_substring(s:String) -> i32 {

    let mut map = HashMap::new();

    let mut start = 0;
    let mut max_len = 0;

    for (i,c) in s.chars().enumerate() {
        if let Some(&last_index) = map.get(&c) {
            if last_index >= start {
                start = last_index+1;
            }
        }
        map.insert(c, i);
        max_len = max_len.max(i - start + 1);
    }

    max_len as i32
}



pub fn longest_consecutive(nums: Vec<i32>) -> i32 {

    let set: HashSet<i32> = nums.iter().cloned().collect();

    let mut longest =0;

    for &num in &set {

        //check the first value 
        if !set.contains(&(num -1) ){
            let mut current_num = num;
        let mut length = 1;

        //count the legth of consective number
        while set.contains(&(current_num+1)) {
            current_num +=1;
            length +=1;

        }

        longest = longest.max(length);


        }
    }
    longest
}

//------------------------------------------------

//frequnecy count problem -------------------------------


pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {

    let mut map = HashMap::new();


    map.insert(0, 1);

    let mut count = 0;
    let mut prefix_sum = 0;

    for num in nums {
        prefix_sum += num;

        if let Some(&times) = map.get(&(prefix_sum -k)) {
            count += times;
        }

        *map.entry(prefix_sum).or_insert(0) += 1;
    }

    count
}

pub fn first_unique_char(s: String) -> i32 {

    let mut freq = HashMap::new();

    //step 1: Count frequency of each charater
    for ch in s.chars() {

        *freq.entry(ch).or_insert(0) +=1;
    }

    //step 2: Find first chsr with frequency 1
    for (i, ch) in s.chars().enumerate() {
        if freq[&ch] == 1 {
            return i as i32;
        }
    }
    -1
}

use std::collections::{BinaryHeap};
use std::result;
use std::slice::Windows;

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {

    let mut freq = HashMap::new();
    
    //step 1: Count frequency of each number
    for n in nums {
        *freq.entry(n).or_insert(0) +=1;
    }

    //step 2: Create a max heap based frequcy
    let mut heap = BinaryHeap::new();
    for (&num, &count) in freq.iter() {
        //heap order by count
        heap.push((count,num));
    }

    //step3: pop top k frequent numbers
    let mut result = Vec::new();
    for _ in 0..k {
        if let Some((_,num)) = heap.pop() {
            result.push(num);
        }
    }

    result



}


pub fn find_anagrams(s: String, p: String) -> Vec<i32> {

    let s_chars: Vec<char> = s.chars().collect();
    let p_chars: Vec<char> = p.chars().collect();

    if p.len() > s.len() {
        return vec![];
    }

    let mut p_count = HashMap::new();
    let mut window_count = HashMap::new();
    let mut result = Vec::new();



    //step 1: Count frequency of pattern chars
    for &ch in &p_chars{
        *p_count.entry(ch).or_insert(0) +=1;
    }

    let window_size = p_chars.len();

    //step 2: move sliding window
    for i in 0..s_chars.len() {

        *window_count.entry(s_chars[i]).or_insert(0) +=1;


        if i > window_size {
            let left_char = s_chars[i - window_size];
            *window_count.get_mut(&left_char).unwrap() -=1;
            if window_count[&left_char] == 0{
                window_count.remove(&left_char);
            }
        }
        //step 3: Compare frequnecy maps
        if window_count == p_count{
            result.push((i + 1 - window_size) as i32);
        }
    }
    result
}

//---------------------------------------------------------------------------



pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map : HashMap<String, Vec<String>> = HashMap::new();

    for s in strs {
        let mut chars: Vec<char>= s.chars().collect();
        chars.sort_unstable();
        let key =chars.iter().collect();

        //Group by the key
        map.entry(key).or_default().push(s);
    }

    map.into_values().collect()
}


pub fn word_pattern(pattern: String, s: String) -> bool {
    let words: Vec<&str> = s.split_whitespace().collect();
    if words.len() != pattern.len() {
        return false;
    }

    let mut char_to_word = HashMap::new();
    let mut word_to_char = HashMap::new();

    for (ch, word) in pattern.chars().zip(words.iter()) {
        //check pattern -> word mapping
        if let Some(mapped_word) = char_to_word.get(&ch) {
            if mapped_word != word {
                return false;
            }
        } else {
            char_to_word.insert(ch, *word);
        }

        //check word -> pattern mapping
        if let Some(mapped_char) = word_to_char.get(word) {
            if *mapped_char != ch {
                return false;
            }
        } else {
            word_to_char.insert(*word, ch);
        }
    }

    true
}


use std::cell::RefCell;
use std::rc::Rc;

// Definition for a Node.
#[derive(Debug)]
struct Node {
    val: i32,
    next: Option<Rc<RefCell<Node>>>,
    random: Option<Rc<RefCell<Node>>>,
}

// impl Node {
//     fn new(val: i32) -> Rc<RefCell<Self>> {
//         Rc::new(RefCell::new(Node { val, next: None, random: None }))
//     }
// }

// pub fn copy_random_list(head: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
//     let mut map = HashMap::new();
//     let mut current = head.clone();

//     // Step 1: clone all nodes and store mapping old -> new
//     while let Some(node) = current {
//         let new_node = Node::new(node.borrow().val);
//         map.insert(Rc::as_ptr(&node), new_node);
//         current = node.borrow().next.clone();
//     }

//     // Step 2: connect next and random using map
//     current = head.clone();
//     while let Some(node) = current {
//         let new_node = map.get(&Rc::as_ptr(&node)).unwrap();

//         // Set new_node.next
//         if let Some(next_node) = &node.borrow().next {
//             let new_next = map.get(&Rc::as_ptr(next_node)).unwrap();
//             new_node.borrow_mut().next = Some(new_next.clone());
//         }

//         // Set new_node.random
//         if let Some(random_node) = &node.borrow().random {
//             let new_random = map.get(&Rc::as_ptr(random_node)).unwrap();
//             new_node.borrow_mut().random = Some(new_random.clone());
//         }

//         current = node.borrow().next.clone();
//     }

//     // Return head of cloned list
//     head.map(|h| map.get(&Rc::as_ptr(&h)).unwrap().clone())
// }


// use std::cell::Refcell;
// use std::rc::Rc;

// //Define for a Node.
// #[derive(Debug)]
// struct Node {
//     val: i32,
//     next: Option<Rc<RefCell<Node>>>,
//     random: Option<Rc<RefCell<Node>>>,
// }

// impl Node {
//     fn new(val: i32) -> Rc<RefCell<Self>> {
//         Rc::new(RefCell::new(Node { val, next: None, random: None}))
//     }
// }

// fn copy_randoms_list(head: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
//     let mut map = HashMap::new();
//     let mut current = head.clone();

//     //Step 1: clone all nodes and store mapping old -> new
//     while let Some(node) = current {
//         let new_node = Node::new(node.borrow().val);
//         map.insert(Rc::as_ptr(&node), new_node);
//         current = node.borrow().next.clone();
//     }

//     //Step2: connect next and random using map
//     current = head.clone();
//     while let Some(node) = current {

//         let new_node = map.get(&Rc::as_ptr(&node)).unwrap();

//         if let Some(next_node) = &node.borrow().next {
//             let new_next = map.get(&Rc::as_ptr(next_node)).unwrap();
//             new_node.borrow_mut().next = Some(new_next.clone());
//         }

//         if let Some(random_node) = &node.borrow().random {
//             let new_random = map.get(&Rc::as_ptr(random_node)).unwrap();
//             new_node.borrow_mut().random = Some(new_random.clone());
//         }

//         current = node.borrow().next.clone();

//     }

//     head.map(|h| map.get(&Rc::as_ptr(&h)).unwrap().clone())
// }