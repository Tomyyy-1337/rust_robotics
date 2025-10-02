use std::fmt::Debug;
use std::thread::park;
use std::time::{Duration, Instant};
use meta_signals::MetaSignal;
use ib2c::modules;
use ib2c::modules::basic_group::BasicGroupTrait;
use ib2c::modules::basic_module::{BasicModule, BasicModuleTrait};
use ib2c::modules::behavior_module::BehaviorModule;
use ib2c::modules::general_fusion::GeneralFusionTrait;
use ib2c::modules::maximum_fusion::MaximumFusion;
use scheduling::GroupBuilder;
use modules::behavior_module::BehaviorModuleTrait;
use ports::prelude::*;

fn main() {
    let group = TestGroup::new(true);
    group.spawn();

    park()
}

#[derive(Default)]
struct TestGroup {}

impl BasicGroupTrait for TestGroup {
    fn init(&mut self, builder: &mut GroupBuilder) {
        let module_1 = TestModule::new(Duration::from_millis(5), true);
        let module_2 = Oscillator::new(Duration::from_millis(500), true);

        let mut maximum_fusion = MaximumFusion::new(Duration::from_millis(10), true);
        maximum_fusion.add_module(&module_2.out_data, &module_2.activity);
        maximum_fusion.add_module(&module_1.out_data, &module_1.activity);

        let print_module = PrintModule::new(Duration::from_millis(300), true);
        print_module.in_data.connect_to_source(&maximum_fusion.output_port);

        builder.add_module_builder(module_1);
        builder.add_module_builder(module_2);
        builder.add_module_builder(maximum_fusion);
        builder.add_module_builder(print_module);
    }
}

#[derive(PortMethods, Default)]
struct TestModule {
    pub out_data: SendPort<i32>,
    count: i32,
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
    }

    fn target_rating(_module: &BehaviorModule<Self>) -> MetaSignal {
        MetaSignal::HIGH
    }
}

#[derive(PortMethods, Default)]
struct Oscillator {
    pub out_data: SendPort<i32>,
    state: bool,
}

impl BehaviorModuleTrait for Oscillator {
    fn init() -> Self {
        Oscillator {
            out_data: SendPort::new(-1),
            state: false,
        }
    }

    fn transfer(module: &mut BehaviorModule<Self>) {
        module.state = !module.state;
    }

    fn target_rating(module: &BehaviorModule<Self>) -> MetaSignal {
        if module.state {
            MetaSignal::HIGH
        } else {
            MetaSignal::LOW
        }
    }
}

#[derive(PortMethods, Default)]
struct PrintModule<T> {
    pub in_data: ReceivePort<T>
}

impl<T: Debug + Default> BasicModuleTrait for PrintModule<T> {
    fn update(module: &mut BasicModule<Self>) {
        let data = module.in_data.get_data();
        let timestamp = module.in_data.get_timestamp();
        let delay = Instant::now() - timestamp;
        println!("Received: {:?}, Delay: {:?}", data, delay);
    }
}

#[derive(PortMethods, Default)]
struct BasicTestModule {
    pub out_result: SendPort<u64>,
    param: u64,
}

impl BasicModuleTrait for BasicTestModule {
    fn init() -> Self {
        BasicTestModule {
            param: 0,
            ..Self::default()
        }
    }

    fn update(module: &mut BasicModule<Self>) {
        module.param += 1;
        let fib = BasicTestModule::fib(module.param);
        module.out_result.send(fib);
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