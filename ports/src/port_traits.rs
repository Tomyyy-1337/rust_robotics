/// Trait for updating all [`ReceivePorts`][crate::receive_port::ReceivePort] in a struct
/// Can be derived using the [`PortMethods`] derive macro.
pub trait PortMethods {
    fn update_ports(&mut self);
}