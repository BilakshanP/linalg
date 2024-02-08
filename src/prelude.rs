pub use pyo3::prelude::*;
pub use pyo3::wrap_pymodule;


pub type Mat = Vec<Vec<f64>>;


#[derive(Debug, Clone)]
pub enum Info<T> {
    Value(Option<T>),
    NotCalculated
}

pub use Info::{Value, NotCalculated as NC};