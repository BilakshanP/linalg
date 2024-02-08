#![allow(non_snake_case)]

mod prelude;
mod macros;

use prelude::*;

import_module!(matrices);

#[pyfunction]
fn run() {
    fn fibonacci(upto: usize) -> Vec<usize> {
        fn fibonacci_inner(fibonacci_numbers: &mut Vec<usize>, upto: usize, m: usize, n: usize) {
            if upto != 0 {
                fibonacci_numbers.push(m);
                fibonacci_inner(fibonacci_numbers, upto - 1, n, m + n);
            }
        }
    
        let mut fibonacci_numbers: Vec<usize> = Vec::with_capacity(upto);
    
        fibonacci_inner(&mut fibonacci_numbers, upto, 0, 1);
    
        fibonacci_numbers
    }

    let f = fibonacci(92);

    println!("{:?}", f);
}

#[pymodule]
fn linalg(_py: Python, m: &PyModule) -> PyResult<()> {
    insert_classes!(m, matrices::Matrix);
    insert_functions!(m, run);

    Ok(())
}