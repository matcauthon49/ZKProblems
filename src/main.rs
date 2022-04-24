mod polynomial_evaluation;

fn main() {
    let function: Vec<i32> = vec![1, 2, 3, 4];
    let string = vec![3, 4];
    let prime = 7;

    println!("{}", polynomial_evaluation::f_tilde(function, string, prime));
}