#![allow(non_snake_case)]

mod prelude;
mod macros;

use prelude::*;

#[pymodule]
fn linalg(_py: Python, m: &PyModule) -> PyResult<()> {
    Ok(())
}