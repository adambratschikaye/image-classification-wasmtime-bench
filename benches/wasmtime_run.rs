use criterion::{criterion_group, criterion_main, Criterion};
use wasmtime::{Config, Engine, Instance, Module, OptLevel, Store, Val};
use wasmtime_float::generate_module;

fn setup(
    binary: &[u8],
    opt_level: Option<OptLevel>,
    nan_canonicalization: Option<bool>,
) -> (Store<()>, Instance) {
    let mut config = Config::default();
    if let Some(opt_level) = opt_level {
        config.cranelift_opt_level(opt_level);
    }
    if let Some(nan_canonicalization) = nan_canonicalization {
        config.cranelift_nan_canonicalization(nan_canonicalization);
    }
    let engine = Engine::new(&config).unwrap();
    let mut store = Store::new(&engine, ());
    let module = Module::from_binary(&engine, binary).unwrap();
    let instance = Instance::new(&mut store, &module, &[]).unwrap();
    (store, instance)
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let binary = generate_module(20 * 1024 * 1024);
    let (mut store, instance) = setup(binary.as_slice(), None, None);
    let func = instance.get_func(&mut store, "go").unwrap();
    let mut results = vec![Val::I32(0)];

    c.bench_function("optimized/no_nan_canonicalization", |b| {
        b.iter(|| func.call(&mut store, &[], &mut results).unwrap())
    });

    let (mut store, instance) = setup(binary.as_slice(), Some(OptLevel::None), None);
    let func = instance.get_func(&mut store, "go").unwrap();
    let mut results = vec![Val::I32(0)];

    c.bench_function("not_optimized/no_nan_canonicalization", |b| {
        b.iter(|| func.call(&mut store, &[], &mut results).unwrap())
    });

    let (mut store, instance) = setup(binary.as_slice(), None, Some(true));
    let func = instance.get_func(&mut store, "go").unwrap();
    let mut results = vec![Val::I32(0)];

    c.bench_function("optimized/nan_canonicalization", |b| {
        b.iter(|| func.call(&mut store, &[], &mut results).unwrap())
    });

    let (mut store, instance) = setup(binary.as_slice(), Some(OptLevel::None), Some(true));
    let func = instance.get_func(&mut store, "go").unwrap();
    let mut results = vec![Val::I32(0)];

    c.bench_function("not_optimized/nan_canonicalization", |b| {
        b.iter(|| func.call(&mut store, &[], &mut results).unwrap())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
