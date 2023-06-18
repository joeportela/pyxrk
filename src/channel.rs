use pyo3::prelude::*;
use xdrk::Channel;

#[pyclass(name = "Channel")]
pub struct ChannelPy {
    channel: Channel,
}

#[pymethods]
impl ChannelPy {
    pub fn data(&mut self) -> Vec<f64> {
        self.channel.data().timestamps().to_vec()
    }
}

impl ChannelPy {
    pub fn new(channel: Channel) -> Self {
        Self { channel }
    }
}
