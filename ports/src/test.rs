#[cfg(test)]
mod tests {
    use std::thread;
    use std::thread::sleep;
    use std::time::Duration;
    use crate::prelude::*;

    #[test]
    fn basic_test() {
        let mut port_1: SendPort<Vec<i32>> = SendPort::new(vec![0]);
        let mut port_2: ReceivePort<Vec<i32>> = ReceivePort::default();

        port_2.connect_to_source(&port_1);

        thread::spawn(move || {
            sleep(Duration::from_millis(50));
            port_1.send(vec![1]);
            sleep(Duration::from_millis(100));
            port_1.send(vec![1,2]);
        });

        assert_eq!(*port_2.get_data(), Vec::default());
        port_2.update();
        assert_eq!(*port_2.get_data(), vec![0]);
        sleep(Duration::from_millis(100));
        port_2.update();
        let first_timestamp = port_2.get_timestamp();
        assert_eq!(*port_2.get_data(), vec![1]);
        sleep(Duration::from_millis(100));
        port_2.update();
        let second_timestamp = port_2.get_timestamp();
        assert_eq!(*port_2.get_data(), vec![1,2]);
        assert!(second_timestamp > first_timestamp);


        let port_3: SendPort<Option<i32>> = SendPort::default();
    }

}