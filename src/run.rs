use super::lap::LapPy;
use chrono::NaiveDateTime;
use pyo3::{exceptions::PyValueError, prelude::*};
use std::{path::Path, sync::Arc};
use xdrk::Run;

#[pyclass(name = "Run")]
pub struct RunPy {
    run: Arc<Run>,
}

#[pymethods]
impl RunPy {
    ///
    ///
    /// Run level attributes and functions
    ///
    ///
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
    pub fn channel_names(&mut self) -> Vec<String> {
        self.run.channel_names().to_vec()
    }

    #[getter]
    pub fn channels_count(&mut self) -> usize {
        self.run.number_of_channels()
    }

    pub fn get_channel_unit(&mut self, channel_name: String) -> String {
        let idx = self
            .run
            .channel_idx(&channel_name)
            .expect("Channel not found");
        self.run
            .channel_unit(idx)
            .expect("Couldn't get channel unit")
    }

    pub fn get_channel(
        &mut self,
        channel_name: String,
        lap: Option<usize>,
    ) -> Vec<f64> {
        let idx = self
            .run
            .channel_idx(&channel_name)
            .expect("Channel not found");
        let channel =
            self.run.channel(idx, lap).expect("Failed loading channel");
        channel.data().samples().to_vec()
    }
}

#[pyfunction]
pub fn load_run(path_string: String) -> PyResult<RunPy> {
    let xdrk_file_result = Run::load(Path::new(&path_string));
    match xdrk_file_result {
        Ok(xdrk_file) => Ok(RunPy { run: xdrk_file }),
        Err(error) => Err(PyValueError::new_err(format!(
            "Failed to load .xrk file at {} - {}",
            path_string, error,
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use xdrk::LapInfo;

    const XRK_PATH: &str = "./testdata/test.xrk";
    #[test]
    fn xrkfile_test() {
        let xdrk_file = Run::load(Path::new(XRK_PATH)).unwrap();
        assert_eq!(
            LapInfo::new(2, 336.179, 134.718),
            xdrk_file.lap_info(2).unwrap()
        );
    }
}
