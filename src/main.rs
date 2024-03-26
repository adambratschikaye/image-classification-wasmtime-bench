use rand::Rng;
use wasm_encoder::{
    BlockType, CodeSection, ConstExpr, DataSection, ExportSection, Function, FunctionSection,
    Instruction, MemArg, MemorySection, MemoryType, Module, TypeSection, ValType,
};

fn generate_instructions() -> Vec<Instruction<'static>> {
    use Instruction::*;
    vec![
        // Set 1 to memory size in bytes / 2 = memory size * page_size / 2
        MemorySize(0),
        I32Const(1024 * 32), // half the page size
        I32Mul,
        LocalSet(1),
        // start loop
        Loop(BlockType::Empty),
        LocalGet(0),
        F32Load(MemArg {
            offset: 0,
            align: 0,
            memory_index: 0,
        }),
        LocalGet(0),
        LocalGet(1),
        I32Add,
        F32Load(MemArg {
            offset: 0,
            align: 0,
            memory_index: 0,
        }),
        F32Add,
        LocalSet(2),
        LocalGet(0),
        LocalGet(2),
        F32Store(MemArg {
            offset: 0,
            align: 0,
            memory_index: 0,
        }),
        LocalGet(0),
        I32Const(4),
        I32Add,
        LocalSet(0),
        LocalGet(0),
        LocalGet(1),
        I32GtU,
        BrIf(0),
        End,
        LocalGet(1),
        End,
    ]
}

fn generate_data(size: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let mut data: Vec<f32> = vec![0.0; size];
    for d in &mut data {
        *d = rng.gen();
    }
    data.into_iter().flat_map(|f| f.to_le_bytes()).collect()
}

fn generate_module(size: usize) -> Module {
    let mut module = Module::new();
    let mut type_section = TypeSection::new();
    let mut function_section = FunctionSection::new();
    let mut memory_section = MemorySection::new();
    let mut code_section = CodeSection::new();
    let mut data_section = DataSection::new();
    let mut export_section = ExportSection::new();

    type_section.function([], [ValType::I32]);
    function_section.function(0);
    let mut function = Function::new([(2, ValType::I32), (1, ValType::F32)]);
    for ins in generate_instructions() {
        function.instruction(&ins);
    }
    code_section.function(&function);

    memory_section.memory(MemoryType {
        minimum: (size * 4 / (64 * 1024)) as u64,
        maximum: None,
        memory64: false,
        shared: false,
    });

    let data = generate_data(size);
    data_section.active(0, &ConstExpr::empty().with_i32_const(0), data);

    export_section.export("go", wasm_encoder::ExportKind::Func, 0);

    module.section(&type_section);
    module.section(&function_section);
    module.section(&memory_section);
    module.section(&export_section);
    module.section(&code_section);
    module.section(&data_section);
    module
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let module = generate_module(args[1].parse().unwrap());
    std::fs::write(&args[2], module.as_slice()).unwrap();
}
