use wasmtime_float::generate_module;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let module = generate_module(args[1].parse().unwrap(), false);
    std::fs::write(&args[2], module.as_slice()).unwrap();
}
