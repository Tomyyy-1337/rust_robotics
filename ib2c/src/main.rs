use std::thread::park;
use std::time::Duration;
use ib2c::behavior_module::BehaviorModuleTrait;
use ib2c::meta_signals::MetaSignal;
use module::ThreadContainer;
use ports::prelude::*;

fn main() {
    let mut container = ThreadContainer::new();

    let module_1 = TestModule::new(Duration::from_millis(100));
    let module_2 = TestModule::new(Duration::from_millis(500));

    module_1.in_data.connect_to_source(&module_2.out_data);
    module_2.in_data.connect_to_source(&module_1.out_data);

    module_1.add_to_container(&mut container);
    module_2.add_to_container(&mut container);
    container.run();

    park()
}

#[ports]
#[derive(Default)]
struct TestModule {
    count: usize,
    in_data: ReceivePort<usize>,
    out_data: SendPort<usize>
}

impl BehaviorModuleTrait for TestModule {
     fn transfer(&mut self) {
        self.count += 1;
        self.out_data.send(self.count);
        let timestamp = self.in_data.get_timestamp();
        let time_since_sent = std::time::Instant::now().duration_since(timestamp);
        println!("Module {}: received {} with delay {:?}", self.count, *self.in_data.get_data(), time_since_sent);
    }

    fn target_rating(&self) -> MetaSignal {
        MetaSignal::HIGH
    }
}
