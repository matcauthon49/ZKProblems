let initial_values = vec![];
let initial_vector = vec![];
let prime;

fn chi(w:vec<i32>, x:vec<i32>) -> i32 {
    let mut counter = 1;
    for i in 1..x.len() {
        counter *= (x[i]*w[i]) + (1-x[i])*(1-w[i]);
    }
    return counter;
}

