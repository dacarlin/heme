pub mod conformation;
pub mod sampling;
pub mod transforms;
pub mod io;

// Python bindings 
use std::fs; 
use std::fs::File;
use std::io::Read;
use pyo3::prelude::*;

/// Prints a message.
#[pyfunction]
fn hello() -> PyResult<String> {
    Ok("Hello from Hemes!".into())
}

/// Reads a file and returns its contents with a greeting.
#[pyfunction]
fn parse_my_pdb(file_name: &str) -> PyResult<String> {

    // read input files from Config object
    let mut f = File::open(file_name)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    // create pose object by parsing the PDB
    let atoms = io::parse_pdb(&contents);
    let mut pose = conformation::Pose::from_atoms(atoms);

    println!("read pose"); 
    // Apply a protocol to the Pose
    //let protocol = sampling::get_protocol(&io::config.protocol);
    //protocol.run(&mut pose);

    Ok("read rule".into())
}


/// A Python module implemented in Rust.
#[pymodule]
fn _lowlevel(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_my_pdb, m)?)?;
    m.add_function(wrap_pyfunction!(hello, m)?)?;
    Ok(())
}
