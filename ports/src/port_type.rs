use crate::inner_port::InnerPort;
use crate::port_data::PortData;

pub(crate) enum PortType<T> {
    Endpoint(PortData<T>),
    PassThrough(InnerPort<T>)
}

impl<T> PortType<T> {
    pub(crate) fn new_endpoint() -> Self
    where
        T: Default,
    {
        PortType::Endpoint(PortData::default())
    }

    pub(crate) fn endpoint_with_default(data: T) -> Self {
        PortType::Endpoint(PortData::new(data))
    }
}



