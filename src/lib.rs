use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use wasm_encoder::{
    BlockType, CodeSection, ConstExpr, DataSection, ExportSection, Function, FunctionSection,
    Instruction, MemArg, MemorySection, MemoryType, Module, TypeSection, ValType,
};
use wasmtime::{Config, Engine, Instance, OptLevel, Store};

#[derive(Copy, Clone, PartialEq, Debug)]
struct NanFloat(f32);

/// f32 with a 50% chance of being NaN.
impl NanFloat {
    fn to_le_bytes(self) -> [u8; 4] {
        self.0.to_le_bytes()
    }
}

impl Distribution<NanFloat> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> NanFloat {
        if rng.gen_bool(0.5) {
            NanFloat(rng.gen())
        } else {
            NanFloat(f32::NAN)
        }
    }
}

fn generate_instructions() -> Vec<Instruction<'static>> {
    use Instruction::*;
    vec![
        // Set 1 to memory size in bytes = memory size * page_size
        MemorySize(0),
        I32Const(1024 * 64),
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
        LocalGet(2),
        F32Add,
        LocalSet(2),
        LocalGet(0),
        I32Const(4),
        I32Add,
        LocalSet(0),
        LocalGet(0),
        LocalGet(1),
        I32LtU,
        BrIf(0),
        End,
        LocalGet(2),
        End,
    ]
}

fn generate_data(size: usize, include_nan: bool) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    if include_nan {
        let mut data: Vec<NanFloat> = vec![NanFloat(0.0); size];
        for d in &mut data {
            *d = rng.gen();
        }
        data.into_iter().flat_map(|f| f.to_le_bytes()).collect()
    } else {
        let mut data: Vec<f32> = vec![0.0; size];
        for d in &mut data {
            *d = rng.gen();
        }
        data.into_iter().flat_map(|f| f.to_le_bytes()).collect()
    }
}

pub fn generate_module(size: usize, include_nan: bool) -> Module {
    let mut module = Module::new();
    let mut type_section = TypeSection::new();
    let mut function_section = FunctionSection::new();
    let mut memory_section = MemorySection::new();
    let mut code_section = CodeSection::new();
    let mut data_section = DataSection::new();
    let mut export_section = ExportSection::new();

    type_section.function([], [ValType::F32]);
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

    let data = generate_data(size, include_nan);
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

pub fn setup(
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
    let module = wasmtime::Module::from_binary(&engine, binary).unwrap();
    let instance = Instance::new(&mut store, &module, &[]).unwrap();
    (store, instance)
}
