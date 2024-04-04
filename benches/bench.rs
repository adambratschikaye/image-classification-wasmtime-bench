use criterion::{criterion_group, criterion_main, Criterion};
use wasi_common::{sync::WasiCtxBuilder, WasiCtx};
use wasmtime::{Config, Engine, Instance, Linker, OptLevel, Store};

pub fn setup(
    binary: &[u8],
    opt_level: Option<OptLevel>,
    nan_canonicalization: Option<bool>,
) -> (Store<WasiCtx>, Instance) {
    let mut config = Config::default();
    if let Some(opt_level) = opt_level {
        config.cranelift_opt_level(opt_level);
    }
    if let Some(nan_canonicalization) = nan_canonicalization {
        config.cranelift_nan_canonicalization(nan_canonicalization);
    }
    let engine = Engine::new(&config).unwrap();
    let mut linker = Linker::new(&engine);
    wasi_common::sync::add_to_linker(&mut linker, |s: &mut WasiCtx| s).unwrap();
    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_args()
        .unwrap()
        .build();
    let mut store = Store::new(&engine, wasi);
    let module = wasmtime::Module::from_binary(&engine, binary).unwrap();
    let instance = linker.instantiate(&mut store, &module).unwrap();
    (store, instance)
}

pub fn image_bench(c: &mut Criterion) {
    let binary =
        include_bytes!("../target/wasm32-wasi/release/image_classification_wasmtime_bench.wasm");
    let (mut store, instance) = setup(binary, Some(OptLevel::None), Some(true));
    instance
        .get_func(&mut store, "init")
        .unwrap()
        .call(&mut store, &[], &mut [])
        .unwrap();

    let func = instance.get_func(&mut store, "classify_example").unwrap();

    c.bench_function("image_classification", |b| {
        b.iter(|| func.call(&mut store, &[], &mut []).unwrap())
    });
}

criterion_group!(benches, image_bench);
criterion_main!(benches);
