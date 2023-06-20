mod channel;
mod run;
use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
fn pyxrk(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(run::load_run, m)?)?;
    Ok(())
}
