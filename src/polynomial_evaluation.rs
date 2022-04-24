fn chi(w:String, x:&Vec<i32>, p:i32) -> i32 {
    let mut counter: i32 = 1;
    for i in 1..x.len() {
        counter *= (x[i]*((w.as_bytes())[i] as i32)) + (1-x[i])*(1-((w.as_bytes())[i]) as i32);
    }
    return counter % p;
}

pub fn f_tilde(fun_list:Vec<i32>, x:Vec<i32>, p:i32) -> i32 {
    let mut counter: i32 = 0;
    for i in 1..fun_list.len() {
        counter += fun_list[i]*chi(format!("{:b}", i), &x, p);
    }
    return counter % p;
}