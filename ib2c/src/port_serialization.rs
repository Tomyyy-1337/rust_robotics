
/// Required by [`ports::send_port::SendPort`] and [`ports::receive_port::ReceivePort`] ports
pub trait PortSerialize
where
    Self: Sized,
{
    /// Serialize the data to a String
    fn serialize(&self) -> String;
}

/// Required by [`ports::send_port::SendPort`], [`ports::receive_port::ReceivePort`] and Parameter-ports
pub trait PortDeserialize
where
    Self: Sized,
{
    /// Deserialize the data from a String
    /// Returns None if deserialization fails
    fn deserialize(data: &str) -> Option<Self>;
}

macro_rules! port_serialize_primitives {
    (
        $($t:ty),*
    ) => {
        $(
            impl PortSerialize for $t {
                fn serialize(&self) -> String {
                    self.to_string()
                }
            }

            impl PortDeserialize for $t {
                fn deserialize(data: &str) -> Option<Self> {
                    data.parse::<$t>().ok()
                }
            }
        )*
    };
}

port_serialize_primitives!(u8, u16, u32, u64, u128, usize);
port_serialize_primitives!(i8, i16, i32, i64, i128, isize);
port_serialize_primitives!(f32, f64);
port_serialize_primitives!(bool);
port_serialize_primitives!(char);
port_serialize_primitives!(String);

impl<T: PortSerialize> PortSerialize for Option<T> {
    fn serialize(&self) -> String {
        match self {
            Some(value) => value.serialize(),
            None => String::from("None"),
        }
    }
}

impl<R,E> PortSerialize for Result<R,E>
where
    R: PortSerialize,
    E: PortSerialize,
{
    fn serialize(&self) -> String {
        match self {
            Ok(value) => value.serialize(),
            Err(err) => format!("Err({})", err.serialize()),
        }
    }
}

impl<T: PortSerialize> PortSerialize for Vec<T> {
    fn serialize(&self) -> String {
        let serialized_elements: Vec<String> = self.iter().map(|elem| elem.serialize()).collect();
        format!("[{}]", serialized_elements.join(", "))
    }
}

macro_rules! port_serialize_tuples {
    (
        $(
            $($name:ident, $index:tt),+ => $len:tt
        );* $(;)?
    ) => {
        $(
            impl<$($name: PortSerialize),+> PortSerialize for ($($name,)+) {
                fn serialize(&self) -> String {
                    let mut parts = Vec::new();
                    $(
                        parts.push(self.$index.serialize());
                    )+
                    format!("({})", parts.join(", "))
                }
            }

            impl<$($name: PortDeserialize),+> PortDeserialize for ($($name,)+) {
                fn deserialize(data: &str) -> Option<Self> {
                    let data = data.trim();
                    if !data.starts_with('(') || !data.ends_with(')') {
                        return None;
                    }
                    let inner = &data[1..data.len()-1];
                    let parts: Vec<&str> = inner.split(',').map(|s| s.trim()).collect();
                    if parts.len() != $len {
                        return None;
                    }
                    Some((
                        $(
                            match <$name as PortDeserialize>::deserialize(parts[$index]) {
                                Some(value) => value,
                                None => return None,
                            },
                        )+
                    ))
                }
            }
        )*
    };
}

port_serialize_tuples!(
    A, 0, B, 1 => 2;
    A, 0, B, 1, C, 2 => 3;
    A, 0, B, 1, C, 2, D, 3 => 4;
    A, 0, B, 1, C, 2, D, 3, E, 4 => 5;
    A, 0, B, 1, C, 2, D, 3, E, 4, F, 5 => 6;
    A, 0, B, 1, C, 2, D, 3, E, 4, F, 5, G, 6 => 7;
    A, 0, B, 1, C, 2, D, 3, E, 4, F, 5, G, 6, H, 7 => 8
);