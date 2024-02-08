mod trait_definitions;

use super::prelude::*;

#[pyclass]
#[derive(Debug)]
pub struct Matrix {
    mat: Mat,
    row: usize,
    col: usize,
}

#[pymethods]
impl Matrix {
    #[new]
    fn new() -> Self {
        Matrix { mat: Vec::new(), row: 0, col: 0 }
    }

    #[getter]
    fn matv(&self) -> Mat {
        self.mat.clone()
    }

    #[getter]
    fn rc(&self) -> (usize, usize) {
        (self.row, self.col)
    }
}