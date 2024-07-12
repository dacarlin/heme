pub mod conformation;
pub mod sampling;
pub mod transforms;
pub mod io;

// Python bindings 
use pyo3::prelude::*;

/// Prints a message.
#[pyfunction]
fn hello() -> PyResult<String> {
    io::parse_pdb("AAAAA"); 
    Ok("Hello from Heme!".into())
}

#[pyfunction]
fn goodbye() -> PyResult<String> {
    Ok("Hello from Heme!".into())
}

/// A Python module implemented in Rust.
#[pymodule]
fn _lowlevel(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hello, m)?)?;
    m.add_function(wrap_pyfunction!(goodbye, m)?)?;
    Ok(())
}
