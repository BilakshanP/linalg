#![allow(non_snake_case)]

mod prelude;
mod macros;

use prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
fn linalg(_py: Python, m: &PyModule) -> PyResult<()> {
    Ok(())
}