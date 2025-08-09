pub fn recursion(num:i32) -> i32{
    
    if num == 1{
        return num;
    }
    let res = recursion(num-1);

    let my_ans = res*num;

    return my_ans;



}