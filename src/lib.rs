mod lap;
mod run;
use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
fn python_xrk(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(run::load_run, m)?)?;
    Ok(())
}
