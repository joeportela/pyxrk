use std::{iter::Zip, vec};

use pyo3::{exceptions::PyValueError, prelude::*};
use xdrk::Channel;

#[pyclass(name = "Channel")]
pub struct ChannelPy {
    channel: Channel,
    #[pyo3(get)]
    frequency: f64,
}

#[pymethods]
impl ChannelPy {
    #[getter]
    pub fn name(&self) -> &str {
        self.channel.name()
    }

    #[getter]
    pub fn unit(&self) -> &str {
        self.channel.unit()
    }

    pub fn sample_count(&self) -> usize {
        self.channel.len()
    }

    pub fn is_empty(&self) -> bool {
        self.channel.is_empty()
    }

    pub fn sync_with(&self, other: &Self) -> PyResult<Self> {
        match self.channel.synchronize_with(&other.channel) {
            Ok(new_channel) => Ok(Self::new(new_channel)),
            Err(error) => Err(PyValueError::new_err(format!("{}", error))),
        }
    }

    pub fn samples(&self) -> Vec<f64> {
        self.channel.data().samples().to_vec()
    }

    pub fn timestamps(&self) -> Vec<f64> {
        self.channel.data().timestamps().to_vec()
    }

    pub fn data(&self) -> ChannelDataIterator {
        let data = self.channel.data();
        // Sadly using clone here https://github.com/PyO3/pyo3/issues/1085
        ChannelDataIterator {
            iter: data.clone().into_iter(),
        }
    }
}

impl ChannelPy {
    pub fn new(channel: Channel) -> Self {
        // frquency is static, so only compute it once
        let frequency = channel.frequency();
        Self { channel, frequency }
    }
}

#[pyclass]
pub struct ChannelDataIterator {
    iter: Zip<vec::IntoIter<f64>, vec::IntoIter<f64>>,
}

#[pymethods]
impl ChannelDataIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }
    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<(f64, f64)> {
        slf.iter.next()
    }
}
