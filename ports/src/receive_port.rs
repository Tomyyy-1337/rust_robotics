use std::ops::Deref;
use crate::inner_port::InnerPort;
use crate::port_data::PortData;

pub struct ReceivePort<T> {
    inner_port: InnerPort<T>,
    buffer: PortData<T>,
}

impl<T> ReceivePort<T> {
    pub fn new(data: T) -> Self {
        let port_data = PortData::new(data);
        let inner_port = InnerPort::with_default_data(port_data.clone());
        Self {
            inner_port: inner_port,
            buffer: port_data,
        }
    }

    pub fn update(&mut self) {
        let data= self.inner_port.read();
        self.buffer = data;
    }

    pub fn get_data(&self) -> &T {
        self.buffer.get_data()
    }

    pub fn get_timestamp(&self) -> std::time::Instant {
        self.buffer.get_timestamp()
    }
}

impl<T: Default> Default for ReceivePort<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T> Deref for ReceivePort<T> {
    type Target = InnerPort<T>;

    fn deref(&self) -> &Self::Target {
        &self.inner_port
    }
}