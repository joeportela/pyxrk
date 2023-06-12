use chrono::NaiveDateTime;
use pyo3::prelude::*;
use std::{path::Path, sync::Arc};
use xdrk::Run;

#[pyclass(name = "Run")]
pub struct RunPy {
    run: Arc<Run>,
}

#[pymethods]
impl RunPy {
    #[getter]
    pub fn lap_count(&mut self) -> usize {
        self.run.number_of_laps()
    }

    #[getter]
    pub fn championship(&mut self) -> String {
        self.run.championship().unwrap()
    }

    #[getter]
    pub fn track(&mut self) -> String {
        self.run.track().unwrap()
    }

    #[getter]
    pub fn venue_type(&mut self) -> String {
        self.run.venue_type().unwrap()
    }

    #[getter]
    pub fn vehicle(&mut self) -> String {
        self.run.vehicle().unwrap()
    }

    #[getter]
    pub fn racer(&mut self) -> String {
        self.run.racer().unwrap()
    }

    #[getter]
    pub fn datetime(&mut self) -> NaiveDateTime {
        self.run.datetime().expect("Failure getting datetime")
    }
}

#[pyfunction]
pub fn load_run(path_string: String) -> RunPy {
    let xdrk_file =
        Run::load(Path::new(&path_string)).expect("Failure loading file");
    RunPy { run: xdrk_file }
}
