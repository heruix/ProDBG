use std::io;
use std::fs::File;
use std::io::Write;

use ::data::*;

pub fn generate_ffi_bindings(filename: &str, structs: &Vec<Struct>) -> io::Result<()> {
    let mut f = File::create(filename)?;

    f.write_all(b"use std::os::raw::{c_void, c_char};\n\n")?;

    for struct_ in structs {
        f.write_all(b"#[repr(C)]\n")?;
        f.write_fmt(format_args!("pub struct {} {{\n", struct_.name))?;

        for entry in &struct_.entries {
            match entry {
                &StructEntry::Var(ref var) => {
                    f.write_fmt(format_args!("    pub {}: {},\n", var.name, var.rust_type))?;
                }

                &StructEntry::FunctionPtr(ref func_ptr) => {
                    f.write_fmt(format_args!("    pub {}: extern \"C\" fn(", func_ptr.name))?;

                    let arg_count = func_ptr.function_args.len();

                    for (i, arg) in func_ptr.function_args.iter().enumerate() {
                        f.write_fmt(format_args!("{}: {}", arg.name, arg.rust_type))?;

                        if i != arg_count - 1 {
                            f.write_all(b", ")?;
                        }
                    }

                    f.write_all(b")")?;

                    if let Some(ref ret_var) = func_ptr.return_val {
                        f.write_fmt(format_args!(" -> {}", ret_var.rust_type))?;
                    }

                    f.write_all(b",\n")?;
                }
            }
        }

        f.write_all(b"}\n\n")?;
    }

    f.write_all(b"extern \"C\" {\n")?;
    f.write_all(b"    pub fn wrui_get() -> *mut Wrui;\n")?;
    f.write_all(b"}\n")?;

    Ok(())
}
