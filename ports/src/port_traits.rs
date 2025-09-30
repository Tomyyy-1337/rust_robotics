use crate::inner_port::InnerPort;
use crate::prelude::ReceivePort;

pub trait PortMethods {
    fn update_ports(&mut self);
}