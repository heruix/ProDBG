use std::io;
use std::fs::File;
use std::io::Write;

extern crate clang;

pub mod data;
mod rust_ffi_gen;

use data::*;

use clang::*;

static INPUT_HEADER: &'static str = "../../native/external/wrui/include/wrui.h";
static RUST_FFI_FILE: &'static str = "../../../src/prodbg/wrui_rust/src/ffi_gen.rs";
static TRAITS_FILE: &'static str = "../../../src/prodbg/wrui_rust/src/traits_gen.rs";
static WIDGETS_FILE: &'static str = "../../../src/prodbg/wrui_rust/src/widgets.rs";

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
			println!("{} - {}", struct_.name, trait_name.c_name);
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
                                ("&mut self".to_owned(), "".to_owned())
                            } else {
                                (arg.name.to_owned(), arg.rust_type.to_owned())
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

///
/// 2. Check if function is create style function
///
fn is_create_func(func: &FuncPtr) -> bool {
	if func.name.find("_create").is_none() {
		return false;
	}

	func.return_val.is_some()
}

///
///
///
fn find_funcs_struct<'a>(func: &FuncPtr, structs: &'a Vec<Struct>) -> &'a Struct {
	let index = func.name.find("_create").unwrap();
	structs.iter().find(|&e| { e.name == &func.name[..index] }).unwrap()
}

///
/// 3. Find name_funcs that maps to 2.
///
fn generate_struct(f: &mut File, func: &FuncPtr, structs: &Vec<Struct>) -> io::Result<()> {
	let type_name = func.return_val.as_ref().unwrap().name[7..].to_owned();
	let funcs_struct = find_funcs_struct(func, structs);

	f.write_fmt(format_args!("pub {} {{\n", &func.name[2..]))?;
	f.write_fmt(format_args!("    funcs: *const {}", funcs_struct.name))?;
	f.write_fmt(format_args!("    obj: *const {}", type_name))?;
	f.write_all(b"}\n\n")?;

	f.write_fmt(format_args!("impl {} {{\n", &func.name[2..]))?;

	for entry in &funcs_struct.entries {
		match entry {
			&StructEntry::FunctionPtr(ref func_ptr) => {
				func_ptr.write_func_def(f, |index, arg| {
					if index == 0 {
						("&mut self".to_owned(), "".to_owned())
					} else {
						(arg.name.to_owned(), arg.rust_type.to_owned())
					}
				})?;
			}

			_ => (),
		}
	}

	f.write_all(b"}\n\n")?;

	Ok(())
}

///
/// This functions generates "Real" Rust bindings (using the FFI wrapper)
///
/// It's done in this way:
///
/// 1. Find the the Wrui struct.
/// 2. Find a name_create function that returns GUX* inside the UI struct
/// 3. Find name_funcs that maps to 2.
/// 4. Generate struct X which uses functions in name_funcs and wraps GUX object
/// 5. If struct has GUWidget* base also generate Widget trait impl
///
fn generate_rust_binding(filename: &str, structs: &Vec<Struct>) -> io::Result<()> {
	let mut f = File::create(filename)?;

	let wrui_struct = structs.iter().find(|&e| { e.name == "Wrui" } ).unwrap();

	for entry in &wrui_struct.entries {
		match entry {
			&StructEntry::FunctionPtr(ref func_ptr) => {
				if !is_create_func(func_ptr) {
					continue;
				}

				generate_struct(&mut f, func_ptr, structs)?;
			}

			_ => (),
		}
	}

	Ok(())
}

fn main() {
    // Acquire an instance of `Clang`
    let clang = Clang::new().unwrap();

    // Create a new `Index`
    let index = Index::new(&clang, false, false);

    // Parse a source file into a translation unit
    let tu = index.parser(INPUT_HEADER).parse().unwrap();

    // Get the structs in this translation unit also remove some stuff we don't need
    let structs = tu.get_entity()
        .get_children()
        .into_iter()
        .filter(|e| {
            let name = e.get_type().unwrap().get_display_name();
            e.get_kind() == EntityKind::StructDecl && name.find("__").is_none() &&
            name.find("_opaque").is_none()
        })
        .collect::<Vec<_>>();

    let structs = data::build_data(&structs);

    if let Err(err) = rust_ffi_gen::generate_ffi_bindings(RUST_FFI_FILE, &structs) {
        panic!("Unable to generate {} err {:?}", RUST_FFI_FILE, err);
    }

    if let Err(err) = generate_traits(TRAITS_FILE, &structs) {
        panic!("Unable to generate {} err {:?}", RUST_FFI_FILE, err);
    }

    if let Err(err) = generate_rust_binding(WIDGETS_FILE, &structs) {
        panic!("Unable to generate {} err {:?}", RUST_FFI_FILE, err);
    }
}
