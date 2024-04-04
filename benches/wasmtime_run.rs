use criterion::{criterion_group, criterion_main, Criterion};
use wasmtime::{OptLevel, Val};
use wasmtime_float::{generate_module, setup};

pub fn criterion_benchmark(c: &mut Criterion) {
    let binary = generate_module(20 * 1024 * 1024, false);
    let (mut store, instance) = setup(binary.as_slice(), None, None);
    let func = instance.get_func(&mut store, "go").unwrap();
    let mut results = vec![Val::I32(0)];

    c.bench_function("optimized/without_nan_canonicalization", |b| {
        b.iter(|| func.call(&mut store, &[], &mut results).unwrap())
    });

    let (mut store, instance) = setup(binary.as_slice(), Some(OptLevel::None), None);
    let func = instance.get_func(&mut store, "go").unwrap();
    let mut results = vec![Val::I32(0)];

    c.bench_function("not_optimized/without_nan_canonicalization", |b| {
        b.iter(|| func.call(&mut store, &[], &mut results).unwrap())
    });

    let (mut store, instance) = setup(binary.as_slice(), None, Some(true));
    let func = instance.get_func(&mut store, "go").unwrap();
    let mut results = vec![Val::I32(0)];

    c.bench_function("optimized/with_nan_canonicalization", |b| {
        b.iter(|| func.call(&mut store, &[], &mut results).unwrap())
    });

    let (mut store, instance) = setup(binary.as_slice(), Some(OptLevel::None), Some(true));
    let func = instance.get_func(&mut store, "go").unwrap();
    let mut results = vec![Val::I32(0)];

    c.bench_function("not_optimized/with_nan_canonicalization", |b| {
        b.iter(|| func.call(&mut store, &[], &mut results).unwrap())
    });

    let wat = r#"
    (module
        (global $x_f64 (mut f64) (f64.const 1000000007))
        (global $y_f64 (mut f64) (f64.const 1337))
        (table $table 10 funcref)
        (elem func 0)
        (memory $mem 1)
        (func $test (export "go")
		    (result i32)
            (local $i i32)
            (local $x_f64 f64)
            (local $y_f64 f64)
            (local.set $y_f64 (global.get $y_f64))
            (local.set $x_f64 (global.get $x_f64))
            (local.set $i (i32.const 2_000_000)) (loop $loop
                (global.set $x_f64 (f64.add (local.get $x_f64) (local.get $y_f64)))
                (global.set $x_f64 (f64.add (local.get $x_f64) (local.get $y_f64)))
                (global.set $x_f64 (f64.add (local.get $x_f64) (local.get $y_f64)))
                (global.set $x_f64 (f64.add (local.get $x_f64) (local.get $y_f64)))
                (global.set $x_f64 (f64.add (local.get $x_f64) (local.get $y_f64)))
                (global.set $x_f64 (f64.add (local.get $x_f64) (local.get $y_f64)))
                (global.set $x_f64 (f64.add (local.get $x_f64) (local.get $y_f64)))
                (global.set $x_f64 (f64.add (local.get $x_f64) (local.get $y_f64)))
                (global.set $x_f64 (f64.add (local.get $x_f64) (local.get $y_f64)))
                (global.set $x_f64 (f64.add (local.get $x_f64) (local.get $y_f64)))
                (global.set $x_f64 (f64.add (local.get $x_f64) (local.get $y_f64)))
                (global.set $x_f64 (f64.add (local.get $x_f64) (local.get $y_f64)))
                (global.set $x_f64 (f64.add (local.get $x_f64) (local.get $y_f64)))
                (global.set $x_f64 (f64.add (local.get $x_f64) (local.get $y_f64)))
                (global.set $x_f64 (f64.add (local.get $x_f64) (local.get $y_f64)))
                (global.set $x_f64 (f64.add (local.get $x_f64) (local.get $y_f64)))
                (global.set $x_f64 (f64.add (local.get $x_f64) (local.get $y_f64)))
                (global.set $x_f64 (f64.add (local.get $x_f64) (local.get $y_f64)))
                (global.set $x_f64 (f64.add (local.get $x_f64) (local.get $y_f64)))
                (global.set $x_f64 (f64.add (local.get $x_f64) (local.get $y_f64)))
                (global.set $x_f64 (f64.add (local.get $x_f64) (local.get $y_f64)))
                (global.set $x_f64 (f64.add (local.get $x_f64) (local.get $y_f64)))
                (global.set $x_f64 (f64.add (local.get $x_f64) (local.get $y_f64)))
                (global.set $x_f64 (f64.add (local.get $x_f64) (local.get $y_f64)))
                (global.set $x_f64 (f64.add (local.get $x_f64) (local.get $y_f64)))
                (global.set $x_f64 (f64.add (local.get $x_f64) (local.get $y_f64)))
                (global.set $x_f64 (f64.add (local.get $x_f64) (local.get $y_f64)))
                (global.set $x_f64 (f64.add (local.get $x_f64) (local.get $y_f64)))
                (global.set $x_f64 (f64.add (local.get $x_f64) (local.get $y_f64)))
                (global.set $x_f64 (f64.add (local.get $x_f64) (local.get $y_f64)))
                (global.set $x_f64 (f64.add (local.get $x_f64) (local.get $y_f64)))
                (global.set $x_f64 (f64.add (local.get $x_f64) (local.get $y_f64)))
                (global.set $x_f64 (f64.add (local.get $x_f64) (local.get $y_f64)))
                (global.set $x_f64 (f64.add (local.get $x_f64) (local.get $y_f64)))
                (global.set $x_f64 (f64.add (local.get $x_f64) (local.get $y_f64)))
                (global.set $x_f64 (f64.add (local.get $x_f64) (local.get $y_f64)))
                (global.set $x_f64 (f64.add (local.get $x_f64) (local.get $y_f64)))
                (global.set $x_f64 (f64.add (local.get $x_f64) (local.get $y_f64)))
                (global.set $x_f64 (f64.add (local.get $x_f64) (local.get $y_f64)))
                (global.set $x_f64 (f64.add (local.get $x_f64) (local.get $y_f64)))
            (br_if $loop (local.tee $i (i32.sub (local.get $i) (i32.const 1)))))
            (global.set $x_f64 (local.get $x_f64))
            (global.set $y_f64 (local.get $y_f64))
			(local.get $i)
        )
    )"#;

    let binary = wat::parse_str(wat).unwrap();
    let (mut store, instance) = setup(&binary, Some(OptLevel::None), Some(true));
    let func = instance.get_func(&mut store, "go").unwrap();
    let mut results = vec![Val::I32(0)];

    c.bench_function("ulan_example", |b| {
        b.iter(|| func.call(&mut store, &[], &mut results).unwrap())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
