use super::lap::LapPy;
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

    pub fn get_lap(&mut self, lap_idx: usize) -> LapPy {
        let lap = self.run.lap(lap_idx).expect("Could not load Lap");
        LapPy::new(lap)
    }

    #[getter]
    pub fn laps(&mut self) -> Vec<LapPy> {
        // let len = self.lap_count();
        // let mut laps = Vec::with_capacity(len);
        // for lap_idx in 0..len {
        //     laps.push(self.get_lap(lap_idx))
        // }
        // laps
        let all_laps = self.run.all_laps().expect("Laps Loaded");
        all_laps.iter().map(|&lap| LapPy::new(&lap)).collect()
        // let len = all_laps.len();
        // let mut laps = Vec::with_capacity(len);
        // for idx in 0..len {
        //     let lap = all_laps.get(idx).cloned();
        //     laps.push(LapPy::new(lap))
        // }
        // laps
    }
}

#[pyfunction]
pub fn load_run(path_string: String) -> RunPy {
    let xdrk_file =
        Run::load(Path::new(&path_string)).expect("Failure loading file");
    RunPy { run: xdrk_file }
}
