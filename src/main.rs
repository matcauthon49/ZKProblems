mod polynomial_evaluation;

fn main() {
    let function: Vec<i32> = vec![4, 3, 1, 4];
    let string = vec![1, 1];
    let prime = 7;

    println!("{}", polynomial_evaluation::f_tilde(function, string, prime));
}