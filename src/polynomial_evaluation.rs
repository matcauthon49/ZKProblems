pub fn chi(w:String, x:&Vec<i32>, p:i32) -> i32 {
    let mut counter: i32 = 1;
    for i in 0..x.len() {
        counter *= (x[i]*((w.as_bytes())[i] as i32 - 0x30)) + (1-x[i])*(1-(((w.as_bytes())[i]) as i32 -0x30));
        counter = ((counter % p) + p) % p;
    }
    return ((counter % p) + p) % p;
}

pub fn f_tilde(fun_list:Vec<i32>, x:Vec<i32>, p:i32) -> i32 {
    let mut counter: i32 = 0;
    for i in 0..fun_list.len() {
        let j = format!("{:0>width$}", format!("{:b}", i), width = x.len());
        counter += fun_list[i]*chi(j, &x, p);
    }
    return counter % p;
}