use derive_more::Deref;
use crate::inner_port::InnerPort;
use crate::port_data::PortData;

/// A port that can receive data from a connected port.
#[derive(Deref)]
pub struct ReceivePort<T> {
    inner_port: InnerPort<T>,
}

impl<T> ReceivePort<T> {
    pub fn new(data: T) -> Self {
        Self { inner_port: InnerPort::with_default_data(PortData::new(data)) }
    }

    /// Updates the internal buffer with the latest data from the connected port.
    pub fn update(&mut self) {
        self.inner_port.update();
    }

    /// Reads the last data from the internal buffer.
    pub fn get_data(&self) -> &T {
        self.inner_port.read_from_buffer().get_data()
    }

    /// Get the timestamp of the last data from the internal buffer.
    pub fn get_timestamp(&self) -> std::time::Instant {
        self.inner_port.read_from_buffer().get_timestamp()
    }
}

impl<T: Default> Clone for ReceivePort<T> {
    fn clone(&self) -> Self {
        Self {
            inner_port: self.inner_port.clone(),
        }
    }
}

impl<T: Default> Default for ReceivePort<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}