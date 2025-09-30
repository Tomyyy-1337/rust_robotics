use crate::inner_port::InnerPort;
use crate::port_data::PortData;

pub(crate) enum PortType<T: Send> {
    Endpoint(PortData<T>),
    PassThrough(InnerPort<T>)
}



