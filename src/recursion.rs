pub fn recursion(num:i32){
    
    if num == 0{
        return;
    }
    println!("{}",num);
    recursion(num-1);
    println!("{}",num);

}