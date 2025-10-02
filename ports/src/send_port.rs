use derive_more::Deref;
use crate::inner_port::InnerPort;
use crate::port_data::PortData;

/// A port that can send data to a connected port.
#[derive(Deref)]
pub struct SendPort<T> {
    inner_port: InnerPort<T>,
}

impl<T> SendPort<T> {
    pub fn new(data: T) -> Self {
        Self {
            inner_port: InnerPort::with_default_data(PortData::new(data)),
        }
    }

    /// Sends data to the connected port.
    pub fn send(&mut self, data: T) {
        self.inner_port.write(&PortData::new(data));
    }

    /// Read the last data from the internal buffer.
    pub fn get_last_data(&self) -> &T {
        self.inner_port.read_from_buffer().get_data()
    }

    /// Get the timestamp of the last data from the internal buffer.
    pub fn get_last_timestamp(&self) -> std::time::Instant {
        self.inner_port.read_from_buffer().get_timestamp()
    }
}

impl<T: Default> Clone for SendPort<T> {
    fn clone(&self) -> Self {
        Self {
            inner_port: self.inner_port.clone(),
        }
    }
}

impl<T: Default> Default for SendPort<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}
