use std::ops::Deref;
use crate::inner_port::InnerPort;
use crate::port_data::PortData;

pub struct SendPort<T> {
    inner_port: InnerPort<T>,
}

impl<T> SendPort<T> {
    pub fn new(data: T) -> Self {
        let port_data = PortData::new(data);
        let inner_port = InnerPort::with_default_data(port_data.clone());
        Self {
            inner_port,
        }
    }

    pub fn connect_to_source(&self, source: &InnerPort<T>) {
        self.inner_port.connect_to_source(source);
    }

    pub fn send(&mut self, data: T) {
        self.inner_port.write(&PortData::new(data));
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

impl<T> Deref for SendPort<T> {
    type Target = InnerPort<T>;

    fn deref(&self) -> &Self::Target {
        &self.inner_port
    }
}

