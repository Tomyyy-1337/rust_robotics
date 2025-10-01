use std::ops::Deref;
use crate::inner_port::InnerPort;
use crate::port_data::PortData;

pub struct ReceivePort<T: Send> {
    inner_port: InnerPort<T>,
    buffer: PortData<T>,
}

impl<T: Send> ReceivePort<T> {
    pub fn new(data: T) -> Self {
        let buffer = PortData::new(data);
        let inner_port = InnerPort::with_default_data(buffer.clone());
        Self { inner_port, buffer }
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

impl<T: Default + Send> Clone for ReceivePort<T> {
    fn clone(&self) -> Self {
        Self {
            inner_port: self.inner_port.clone(),
            buffer: self.buffer.clone(),
        }
    }
}

impl<T: Default + Send> Default for ReceivePort<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T: Send> Deref for ReceivePort<T> {
    type Target = InnerPort<T>;

    fn deref(&self) -> &Self::Target {
        &self.inner_port
    }
}