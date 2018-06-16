// Simple example of how to use parity-wasm builder api.
// Builder api introduced as a method for fast generation of
// different small wasm modules.

extern crate parity_wasm;
extern crate wabt;

use parity_wasm::builder::{module, SignatureBuilder};
use parity_wasm::elements::{
    ExportEntry, External, ImportEntry, Instruction, Instructions, Internal, Local, ValueType,
};
use std::env;
use std::io::Write;

fn main() {
    // Example binary accepts one parameter which is the output file
    // where generated wasm module will be written at the end of execution
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        println!("Usage: {} output_file.wasm", args[0]);
        return;
    }

    // Main entry for the builder api is the module function
    // It returns empty module builder structure which can be further
    // appended with various wasm artefacts
    let module = module()
        .with_signatures(
            vec![SignatureBuilder::new().with_return_type(Some(ValueType::I32)).build_sig()]
        )
        .with_import(ImportEntry::new("host".into(), "rand".into(), External::Function(0)))
        .memory().build()
		// Here we append function to the builder
		// function() function returns a function builder attached
		// to the module builder.
		.function()
			// We describe signature for the function via signature()
			// function. In our simple example it's just one input
			// argument of type 'i32' without return value
			.signature().with_param(ValueType::I32).with_return_type(Some(ValueType::I32)).build()
			// body() without any further arguments means that the body
			// of the function will be empty
			.body().with_locals(vec![Local::new(1, ValueType::I32)])
                   .with_instructions(Instructions::new(vec![
                Instruction::I32Const(0),
                Instruction::GetLocal(0),
                Instruction::Call(0),
                Instruction::I32Add,
                Instruction::TeeLocal(1),
                Instruction::I32Store(0, 0),
                Instruction::GetLocal(1),
                Instruction::End,
            ])).build()
			// This is the end of the function builder. When `build()` is
			// invoked, function builder returns original module builder
			// from which it was invoked
			.build()
            .with_export(ExportEntry::new("_call".into(), Internal::Function(1)))
            .with_export(ExportEntry::new("mem".into(), Internal::Memory(0)))
		// And finally we finish our module builder to produce actual
		// wasm module.
		.build();

    // Module structure can be serialzed to produce a valid wasm file
    parity_wasm::serialize_to_file(&args[1], module.clone()).unwrap();
    let module = parity_wasm::serialize(module).unwrap();
    let wat = wabt::wasm2wat(module).unwrap();
    let mut file = std::fs::File::create("out.wat").unwrap();
    write!(file, "{}", wat).unwrap();
}
