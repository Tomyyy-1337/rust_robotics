use std::thread::park;
use std::time::Duration;
use ib2c::meta_signals::MetaSignal;
use ib2c::modules;
use ib2c::modules::basic_module::{BasicModule, BasicModuleTrait};
use ib2c::modules::behavior_module::BehaviorModule;
use ib2c::modules::maximum_fusion::MaximumFusion;
use module::ThreadContainer;
use modules::behavior_module::BehaviorModuleTrait;
use ports::prelude::*;

fn main() {
    let module_1 = TestModule::new(Duration::from_millis(500));
    let module_2 = TestModule::new(Duration::from_millis(500));

    let basic_module = BasicTestModule::new(Duration::from_millis(500));
    module_1.in_data.connect_to_source(&basic_module.out_result);
    module_2.in_data.connect_to_source(&basic_module.out_result);

    let mut maximum_fusion = MaximumFusion::new(Duration::from_millis(100));
    maximum_fusion.add_module(&module_1.out_data, &module_1.activity);
    maximum_fusion.add_module(&module_2.out_data, &module_2.activity);

    let mut container = ThreadContainer::new();
    let mut container_2 = ThreadContainer::new();

    module_1.add_to_container(&mut container);
    module_2.add_to_container(&mut container);
    maximum_fusion.add_to_container(&mut container);
    basic_module.add_to_container(&mut container_2);

    container.run();
    container_2.run();

    park()
}

#[module]
struct TestModule {
    in_data: ReceivePort<u64>,
    out_data: SendPort<usize>,
    count: usize,
}

impl BehaviorModuleTrait for TestModule {
    fn init() -> Self{
        Self{
            count: 0,
            ..Self::default()
        }
    }

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

#[module]
struct BasicTestModule {
    pub out_result: SendPort<u64>,
    param: u64,
}

impl BasicModuleTrait for BasicTestModule {
    fn init() -> Self {
        BasicTestModule {
            param: 0,
            out_result: SendPort::new(0),
        }
    }

    fn update(module: &mut BasicModule<Self>) {
        module.param += 1;
        let fib = BasicTestModule::fib(module.param);
        module.out_result.send(fib);
        println!("Basic module: fib({}) = {}", module.param, fib);
    }
}

impl BasicTestModule {
    fn fib(n: u64) -> u64 {
        match n {
            0 | 1 => n,
            _ => BasicTestModule::fib(n - 1) + BasicTestModule::fib(n - 2),
        }
    }
}