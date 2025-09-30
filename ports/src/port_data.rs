use std::sync::Arc;
use std::time::Instant;

pub(crate) struct PortData<T> {
    data: Arc<T>,
    timestamp: Instant,
}

impl<T> PortData<T> {
    pub(crate) fn new(data: T) -> Self {
        Self {
            data: Arc::new(data),
            timestamp: Instant::now(),
        }
    }

    pub(crate) fn get_data(&self) -> &T {
        &self.data
    }

    pub(crate) fn get_timestamp(&self) -> Instant {
        self.timestamp
    }
}

impl<T: Default> Default for PortData<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T> Clone for PortData<T> {
    fn clone(&self) -> Self {
        Self {
            data: Arc::clone(&self.data),
            timestamp: self.timestamp,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_port_data() {
        let data = PortData::new(42);
        assert_eq!(*data.get_data(), 42);
        let timestamp = data.get_timestamp();
        assert!(timestamp <= Instant::now());

        let default_data: PortData<i32> = PortData::default();
        assert_eq!(*default_data.get_data(), 0);

        let cloned_data = data.clone();
        assert_eq!(*cloned_data.get_data(), 42);
        assert_eq!(cloned_data.get_timestamp(), data.get_timestamp());
        assert_eq!(Arc::strong_count(&data.data), 2);
        assert_eq!(Arc::strong_count(&cloned_data.data), 2);
    }
}