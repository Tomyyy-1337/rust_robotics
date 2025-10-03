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
use scheduling::{GroupBuilder, ModuleBuilder, SpawnMode};
use modules::behavior_module::BehaviorModuleTrait;
use ports::prelude::*;

fn main() {
    let group = TestGroup::new(SpawnMode::GroupThread);
    group.spawn();

    park()
}

#[derive(Default)]
struct TestGroup {}

impl BasicGroupTrait for TestGroup {
    fn init(&mut self, builder: &mut GroupBuilder) {
        let module_1 = ModuleBuilder::new(TestModule::new(), Duration::from_millis(700), SpawnMode::GroupThread);
        let module_2 = ModuleBuilder::new(Oscillator::new(), Duration::from_millis(500), SpawnMode::GroupThread);

        let mut maximum_fusion = ModuleBuilder::new(MaximumFusion::new(), Duration::from_millis(10), SpawnMode::GroupThread);
        maximum_fusion.add_module(&module_2.out_data, &module_2.activity);
        maximum_fusion.add_module(&module_1.out_data, &module_1.activity);

        let print_module = ModuleBuilder::new(PrintModule::new(), Duration::from_millis(300), SpawnMode::GroupThread);
        print_module.in_data.connect_to_source(&maximum_fusion.output_port);

        let expensive_modules = TenModulesGroup::new(SpawnMode::NewThread);

        builder.add_module(module_1);
        builder.add_module(module_2);
        builder.add_module(maximum_fusion);
        builder.add_module(print_module);
        builder.add_group(expensive_modules);
    }
}

#[derive(Default)]
struct TenModulesGroup {}

impl BasicGroupTrait for TenModulesGroup {
    fn init(&mut self, builder: &mut GroupBuilder) {
        for _ in 0..10 {
            let module = ModuleBuilder::new(FibModule::new(), Duration::from_millis(100), SpawnMode::GroupThread);
            builder.add_module(module);
        }
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
struct FibModule {
    pub out_result: SendPort<u64>,
    param: u64,
}

impl BasicModuleTrait for FibModule {
    fn init() -> Self {
        FibModule {
            param: 0,
            ..Self::default()
        }
    }

    fn update(module: &mut BasicModule<Self>) {
        module.param += 1;
        let fib = FibModule::fib(module.param);
        module.out_result.send(fib);
    }
}

impl FibModule {
    fn fib(n: u64) -> u64 {
        match n {
            0 | 1 => n,
            _ => FibModule::fib(n - 1) + FibModule::fib(n - 2),
        }
    }
}