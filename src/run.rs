use super::channel::ChannelPy;
use super::lap::LapPy;
use chrono::NaiveDateTime;
use pyo3::{exceptions::PyValueError, prelude::*};
use std::{collections::HashMap, path::Path, sync::Arc};
use xdrk::Run;

#[pyclass(name = "Run")]
pub struct RunPy {
    run: Arc<Run>,
    channel_names: HashMap<String, usize>,
}

impl RunPy {
    pub fn new(run: Arc<Run>) -> Self {
        // Setup a channel name to index mapping
        // Most of the operations are based on index args,
        // so we'll be doing a lot of lookups.
        let mut channel_names: HashMap<String, usize> = HashMap::new();
        for i in 0..run.number_of_channels() {
            channel_names.insert(run.channel_name(i).unwrap(), i);
        }
        Self { run, channel_names }
    }
}

#[pymethods]
impl RunPy {
    ///
    ///
    /// Run level attributes and functions
    ///
    ///
    #[getter]
    pub fn lap_count(&self) -> usize {
        self.run.number_of_laps()
    }

    #[getter]
    pub fn championship(&self) -> String {
        self.run.championship().unwrap()
    }

    #[getter]
    pub fn track(&self) -> String {
        self.run.track().unwrap()
    }

    #[getter]
    pub fn venue_type(&self) -> String {
        self.run.venue_type().unwrap()
    }

    #[getter]
    pub fn vehicle(&self) -> String {
        self.run.vehicle().unwrap()
    }

    #[getter]
    pub fn racer(&self) -> String {
        self.run.racer().unwrap()
    }

    #[getter]
    pub fn datetime(&self) -> NaiveDateTime {
        self.run.datetime().expect("Failure getting datetime")
    }

    ///
    ///
    /// Lap functions
    ///
    ///
    pub fn get_lap(&mut self, lap_idx: usize) -> LapPy {
        let lap = self.run.lap(lap_idx).expect("Could not load Lap");
        LapPy::new(lap)
    }

    pub fn get_all_laps(&mut self) -> Vec<LapPy> {
        let len = self.lap_count();
        let mut laps = Vec::with_capacity(len);
        for lap_idx in 0..len {
            laps.push(self.get_lap(lap_idx))
        }
        laps
    }

    pub fn get_lap_test(&mut self, lap_idx: usize) {
        // - 11 to skip gps channels (temporary)
        let channel_count = self.channels_count() - 11;
        println!("Channels {}", channel_count);
        for channel_idx in 0..channel_count {
            if channel_idx == 23 || channel_idx == 39 || channel_idx == 40 {
                continue;
            }
            println!(
                "channel data for lap {} channel {} {}",
                lap_idx,
                channel_idx,
                self.run.channel_names()[channel_idx]
            );
            self.run.lap_channel_samples(lap_idx, channel_idx).unwrap();
        }
    }
    ///
    ///
    /// Channel functions
    ///
    ///
    #[getter]
    pub fn channel_names(&self) -> Vec<String> {
        self.run.channel_names().to_vec()
    }

    #[getter]
    pub fn channels_count(&self) -> usize {
        self.run.number_of_channels()
    }

    pub fn get_channel_idx(&self, channel_name: &str) -> PyResult<usize> {
        let result = self.channel_names.get(channel_name);
        match result {
            Some(idx) => Ok(*idx),
            None => Err(PyValueError::new_err(format!(
                "Channel not found: {}",
                channel_name,
            ))),
        }
    }

    pub fn get_channel_unit(&self, channel_name: &str) -> PyResult<String> {
        let idx = self.get_channel_idx(channel_name)?;
        self.get_channel_unit_by_idx(idx)
    }

    pub fn get_channel_unit_by_idx(&self, idx: usize) -> PyResult<String> {
        let unit_result = self.run.channel_unit(idx);
        match unit_result {
            Ok(unit_result) => Ok(unit_result),
            Err(error) => Err(PyValueError::new_err(format!(
                "Couldn't get channel unit for idx {} - {}",
                idx, error
            ))),
        }
    }

    pub fn get_channel(
        &self,
        channel_name: &str,
        lap: Option<usize>,
    ) -> PyResult<ChannelPy> {
        let idx = self.get_channel_idx(channel_name)?;
        self.get_channel_by_idx(idx, lap)
    }

    pub fn get_channel_by_idx(
        &self,
        idx: usize,
        lap: Option<usize>,
    ) -> PyResult<ChannelPy> {
        match self.run.channel(idx, lap) {
            Ok(channel) => Ok(ChannelPy::new(channel)),
            Err(_) => Err(PyValueError::new_err("")),
        }
    }
}

#[pyfunction]
pub fn load_run(path_string: String) -> PyResult<RunPy> {
    let xdrk_file_result = Run::load(Path::new(&path_string));
    match xdrk_file_result {
        Ok(xdrk_file) => Ok(RunPy::new(xdrk_file)),
        Err(error) => Err(PyValueError::new_err(format!(
            "Failed to load .xrk file at {} - {}",
            path_string, error,
        ))),
    }
}
