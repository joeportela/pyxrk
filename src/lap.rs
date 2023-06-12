use pyo3::prelude::*;
use xdrk::Lap;

#[pyclass]
pub struct LapInfoPy {}

#[pyclass(name = "Lap")]
pub struct LapPy {
    lap: Lap,
}

impl LapPy {
    pub fn new(lap: Lap) -> Self {
        Self { lap }
    }
}

#[pymethods]
impl LapPy {
    #[getter]
    pub fn channel_count(&mut self) -> usize {
        self.lap.data().len()
    }

    #[getter]
    pub fn channel_names(&mut self) -> Vec<String> {
        self.lap.channel_names()
    }
}
