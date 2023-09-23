#![feature(arbitrary_self_types)]
#![feature(slice_as_chunks)]

use std::path::PathBuf;

use pyo3::prelude::*;

mod primitives;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn rs_hexpyt(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(translate_file, m)?)?;
    primitives::primitives_submodule(py, m);

    Ok(())
}

#[pyfunction]
#[pyo3(signature = (
    input_file_path,
    output_file_path,
    indentation="    ",
    extra_paths=Vec::new()
))]
fn translate_file(input_file_path: PathBuf, output_file_path: PathBuf, indentation: &str, extra_paths: Vec<&str>) -> PyResult<String> {
    let res = std::fs::read_to_string(input_file_path)?;
    Ok(res)
}
