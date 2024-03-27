use criterion::{criterion_group, criterion_main, Criterion};
use wasmtime::{OptLevel, Val};
use wasmtime_float::{generate_module, setup};

pub fn criterion_benchmark(c: &mut Criterion) {
    let binary = generate_module(20 * 1024 * 1024, false);
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
