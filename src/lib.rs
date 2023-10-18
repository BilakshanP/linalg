#![allow(non_snake_case)]

mod prelude;
mod macros;

use prelude::*;
// use pyo3::wrap_pymodule;

#[pyfunction]
fn func_1_1() {
    println!("1-1")
}

#[pyfunction]
fn func_1_2() {
    println!("1-2")
}

// #[pymodule]
// fn module_1(_py: Python, m: &PyModule) -> PyResult<()> {
//     insert_functions!(m, func_1_1, func_1_2);
//     Ok(())
// }

#[pyfunction]
fn func_2_1() {
    println!("2-1")
}

#[pyfunction]
fn func_2_2() {
    println!("2-2")
}

#[pymodule]
#[pyo3(name="mod_2")]
fn module_2(_py: Python, m: &PyModule) -> PyResult<()> {
    insert_functions!(m, func_2_1, func_2_2);
    Ok(())
}

/// A Python module implemented in Rust.
#[pymodule]
fn LinAlg(_py: Python, m: &PyModule) -> PyResult<()> {
    // m.add_submodule(wrap_pymodule!(module_1));
    let module_1 = PyModule::new(_py, "mod_1")?;
    module_1.add_function(wrap_pyfunction!(func_1_1, module_1)?)?;
    module_1.add_function(wrap_pyfunction!(func_1_2, module_1)?)?;

    // let module_2 = PyModule::new(_py, "mod_2")?;
    // module_2.add_function(wrap_pyfunction!(func_2_1, module_2)?)?;
    // module_2.add_function(wrap_pyfunction!(func_2_2, module_2)?)?;

    m.add_submodule(module_1)?;
    // m.add_submodule(module_2)?;

    // m.add_wrapped(wrap_pymodule!(module_2))?;

    insert_submodule!(m, module_2);

    m.add("PI", std::f64::consts::PI)?;
    m.add("alphabets", (b'a'..=b'z').map(|ord: u8| ord as char).collect::<String>())?;

    Ok(())
}