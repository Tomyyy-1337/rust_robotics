use std::sync::{Arc, RwLock};
use crate::port_data::PortData;
use crate::port_type::PortType;

/// Internal representation of a port.
/// Used internally by the Ports and to connect ports together.
/// All ports Deref to this struct to allow connection of different port types together.
pub struct InnerPort<T> {
    // Data accessed by other ports.
    port_buffer: Arc<RwLock<PortType<T>>>,
    // Internal buffer to avoid locking more than necessary.
    // This buffer is updated when `update` is called or when data is written to the port.
    inner_buffer: PortData<T>
}

impl<T> InnerPort<T> {
    pub(crate) fn with_default_data(data: PortData<T>) -> Self {
        Self{
            port_buffer: Arc::new(RwLock::new(PortType::Endpoint(data.clone()))),
            inner_buffer: data,
        }
    }

    /// Reads the data from the connected port.
    /// This will lock the port for reading.
    pub(crate) fn read_from_connected_port(&self) -> PortData<T> {
        let port = self.port_buffer.read().unwrap();
        match &*port {
            PortType::Endpoint(port_data) => port_data.clone(),
            PortType::PassThrough(inner_port) => inner_port.read_from_connected_port(),
        }
    }

    /// Reads the data from the inner buffer.
    /// Used for reading data without locking the port.
    /// Only use this if you are sure the buffer is being updated.
    pub(crate) fn read_from_buffer(&self) -> &PortData<T> {
        &self.inner_buffer
    }

    /// Updates the inner buffer with the latest data from the connected port.
    /// Data written to the port is immediately available through the inner buffer.
    pub(crate) fn update(&mut self) {
        self.inner_buffer = self.read_from_connected_port();
    }

    /// Writes data to the port
    /// The data is accessible through the inner buffer without calling `update`.
    pub(crate) fn write(&mut self, data: &PortData<T>) {
        self.inner_buffer = data.clone();
        let mut port = self.port_buffer.write().unwrap();
        match &mut *port {
            PortType::Endpoint(port_data) => {
                *port_data = data.clone();
            },
            PortType::PassThrough(inner_port) => {
                inner_port.write(data);
            },
        }
    }

    /// Connects this port to a source port.
    pub fn connect_to_source(&self, source: &InnerPort<T>) {
        let mut port = self.port_buffer.write().unwrap();
        *port = PortType::PassThrough(source.clone());
    }
}

impl<T> Clone for InnerPort<T> {
    fn clone(&self) -> Self {
        Self {
            port_buffer: Arc::clone(&self.port_buffer),
            inner_buffer: self.inner_buffer.clone(),
        }
    }
}