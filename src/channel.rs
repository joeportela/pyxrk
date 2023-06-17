use pyo3::prelude::*;
use xdrk::Channel;

#[pyclass(name = "Channel")]
pub struct ChannelPy {
    channel: Channel,
}

#[pymethods]
impl ChannelPy {}
impl ChannelPy {
    pub fn new(channel: Channel) -> Self {
        Self { channel }
    }
}
