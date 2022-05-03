mod polynomial_evaluation;

fn main() {
    let function_def:Vec<i32> = vec!(1, 12, 3, 4, 5, 6, 9, 10);
    let prime:i32 = 17;
    let to_evaluate:Vec<i32> = vec!(1,1,1);

    println!("{}", polynomial_evaluation::f_tilde(function_def, to_evaluate, prime));
}