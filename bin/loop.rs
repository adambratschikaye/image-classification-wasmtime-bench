use wasmtime::Val;
use wasmtime_float::{generate_module, setup};

const LOOPS: usize = 100;

fn main() {
    let binary = generate_module(20 * 1024 * 1024, true);
    let (mut store, instance) = setup(binary.as_slice(), None, Some(true));
    let func = instance.get_func(&mut store, "go").unwrap();
    let mut results = vec![Val::I32(0)];
    let start = std::time::Instant::now();
    for _ in 0..LOOPS {
        func.call(&mut store, &[], &mut results).unwrap()
    }
    println!("Average time: {:?}", start.elapsed() / LOOPS as u32)
}
