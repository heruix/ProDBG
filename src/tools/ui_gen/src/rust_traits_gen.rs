use std::io;
use std::fs::File;
use std::io::Write;

use ::data::*;

struct MatchName {
	c_name: &'static str,
	rust_name: &'static str,
}

pub fn generate_traits(filename: &str, structs: &Vec<Struct>) -> io::Result<()> {
	let trait_names = [
		MatchName {
			c_name: "GUWidgetFuncs",
			rust_name: "Widget",
		}
	];

	let mut f = File::create(filename)?;

	for struct_ in structs {
		for trait_name in &trait_names {
			if struct_.name != trait_name.c_name {
				continue;
			}

        	f.write_fmt(format_args!("pub trait {} {{\n", trait_name.rust_name))?;

        	for entry in &struct_.entries {
        		match entry {
					&StructEntry::FunctionPtr(ref func_ptr) => {
                        f.write_fmt(format_args!("    fn {}(", func_ptr.name))?;

                        func_ptr.write_func_def(&mut f, |index, arg| {
                            if index == 0 {
                                ("&self".to_owned(), "".to_owned())
                            } else {
                                (arg.name.to_owned(), arg.rust_ffi_type.to_owned())
                            }
                        })?;

						f.write_all(b",\n")?;
					}

                	&StructEntry::Var(ref _var) => (),
				}
			}

        	f.write_all(b"}\n\n")?;
		}
	}

	Ok(())
}

