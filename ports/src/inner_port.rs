use std::sync::{Arc, RwLock};
use crate::port_data::PortData;
use crate::port_type::PortType;

pub struct InnerPort<T> {
    port_buffer: Arc<RwLock<PortType<T>>>,
}

impl<T> InnerPort<T> {
    pub(crate) fn with_default_data(data: PortData<T>) -> Self {
        Self{
            port_buffer: Arc::new(RwLock::new(PortType::Endpoint(data))),
        }
    }

    pub(crate) fn read(&self) -> PortData<T> {
        let port = self.port_buffer.read().unwrap();
        match &*port {
            PortType::Endpoint(port_data) => port_data.clone(),
            PortType::PassThrough(inner_port) => inner_port.read(),
        }
    }

    pub(crate) fn write(&self, data: &PortData<T>) {
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

    pub fn connect_to_source(&self, source: &InnerPort<T>) {
        let mut port = self.port_buffer.write().unwrap();
        *port = PortType::PassThrough(source.clone());
    }
}

impl<T> Clone for InnerPort<T> {
    fn clone(&self) -> Self {
        Self {
            port_buffer: Arc::clone(&self.port_buffer),
        }
    }
}