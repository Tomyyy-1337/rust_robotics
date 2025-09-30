use std::thread::park;
use std::time::Duration;
use ib2c::meta_signals::MetaSignal;
use ib2c::modules;
use ib2c::modules::basic_module::BasicModuleTrait;
use ib2c::modules::behavior_module::BehaviorModule;
use ib2c::modules::maximum_fusion::MaximumFusion;
use module::ThreadContainer;
use modules::behavior_module::BehaviorModuleTrait;
use ports::prelude::*;

fn main() {
    let mut container = ThreadContainer::new();
    let mut container_2 = ThreadContainer::new();

    let module_1 = TestModule::new(Duration::from_millis(100));
    let module_2 = TestModule::new(Duration::from_millis(500));
    module_1.in_data.connect_to_source(&module_2.out_data);
    module_2.in_data.connect_to_source(&module_1.out_data);

    let mut maximum_fusion = MaximumFusion::new(Duration::from_millis(100));
    maximum_fusion.add_module(&module_1.out_data, &module_1.activity);
    maximum_fusion.add_module(&module_2.out_data, &module_2.activity);

    module_1.add_to_container(&mut container);
    module_2.add_to_container(&mut container);
    maximum_fusion.add_to_container(&mut container_2);

    container.run();
    container_2.run();

    park()
}

#[ports] #[derive(Default)]
struct TestModule {
    count: usize,
    in_data: ReceivePort<usize>,
    out_data: SendPort<usize>,
}

impl BehaviorModuleTrait for TestModule {
     fn transfer(module: &mut BehaviorModule<Self>) {
         let count = module.count + 1;
        module.count = count;
        module.out_data.send(count);
        let timestamp = module.in_data.get_timestamp();
        let time_since_sent = std::time::Instant::now().duration_since(timestamp);
        println!("Module {}: received {} with delay {:?} at stimulation {} and inhibition {}",
            count,
            *module.in_data.get_data(),
            time_since_sent,
            *module.stimulation.get_data(),
            *module.inhibition.get_data()
        );
    }

    fn target_rating(module: &BehaviorModule<Self>) -> MetaSignal {
        MetaSignal::HIGH
    }
}
