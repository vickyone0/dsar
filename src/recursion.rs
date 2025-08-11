pub fn recursion(num:i32) -> i32{
    
    if num == 1{
        return num;
    }
    let res = recursion(num-1);

    let my_ans = res*num;

    return my_ans;



}

pub fn recursion_power_logn(a:i32, n:i32) -> i32{
    
    if n == 0{
        return 1;
    }

    if n % 2 == 0 {
        let half = recursion_power_logn(a,n/2);
         return half * half;
    }
    else{
        let my_ans = recursion_power_logn(a, n-1);
        return my_ans;
    }

    return a;

}

