extern crate clang;

use clang::*;

///
/// Checks if a type is a function pointer or not. I'm sure there is a better way for this
/// but it works so meh.
///
fn is_type_function_ptr(t: &clang::Type) -> bool {
	let type_name = t.get_display_name();

	if type_name.find("(*)").is_some() {
		true
	} else {
		false
	}
}


///
/// Maps a C type to Rust
///
fn map_c_type_to_rust(name: &String) -> String {
	let s = name.replace("struct ", "");
	let s = s.replace(" ", "");

	match s.as_str() {
		"_Bool" => "bool".to_owned(),
		"int" => "i32".to_owned(),
		"uint8_t" => "u8".to_owned(),
		"uint16_t" => "u16".to_owned(),
		"uint32_t" => "u32".to_owned(),
		"uint64_t" => "u64".to_owned(),
		"int8_t" => "s8".to_owned(),
		"int16_t" => "s16".to_owned(),
		"int32_t" => "s32".to_owned(),
		"int64_t" => "s64".to_owned(),
		"constchar*" => "*const c_char".to_owned(),
		"void*" => "*const c_void".to_owned(),
		_ => {
			// found pointer, return as is
			if s.find("*").is_some() {
				format!("*const {}", &s[.. s.len() - 1])
			} else {
				panic!("Unable to decode type {}", name);
			}
		}
	}
}

///
/// Generates a regular variable in a struct
///
fn generate_var(var: &clang::Entity) {
	let t = var.get_type().unwrap();
	println!("{},", map_c_type_to_rust(&t.get_display_name()));
}

///
/// Generates a function pointer
///
fn generate_function_ptr(var: &clang::Entity) {
	let mut ret_value = None;
	let child_count = var.get_children().len();
	let t = var.get_type().unwrap();
	let display_name = t.get_display_name();

	// handle the return type

	let type_end = display_name.find("(*)").unwrap();	// will always be ok inside this function as we found this before
	let ret_type = &display_name[..type_end];

	if ret_type != "void " {
		ret_value = Some(map_c_type_to_rust(&ret_type.to_owned()));
	}

	print!("extern \"C\" fn(");
	for (i, c) in var.get_children().iter().enumerate() {
		let t = c.get_type().unwrap();

		if c.get_kind() != clang::EntityKind::TypeRef {
			print!("{}: {}", c.get_display_name().unwrap(), map_c_type_to_rust(&t.get_display_name()));
			if i != child_count - 1 {
				print!(", ");
			}
		}
	}

	print!(")");

	if let Some(ret) = ret_value {
		println!(" -> {},", ret);
	} else {
		println!(",");
	}
}

fn main() {
    // Acquire an instance of `Clang`
    let clang = Clang::new().unwrap();

    // Create a new `Index`
    let index = Index::new(&clang, false, false);

    // Parse a source file into a translation unit
    let tu = index.parser("../../native/external/wrui/include/wrui.h").parse().unwrap();

    // Get the structs in this translation unit
    let structs = tu.get_entity().get_children().into_iter().filter(|e| {
    	let name = e.get_type().unwrap().get_display_name();
        e.get_kind() == EntityKind::StructDecl &&
        name.find("__").is_none() &&
        name.find("_opaque").is_none()
    }).collect::<Vec<_>>();

    println!("use std::os::raw::{{c_void, c_int, c_char}};\n");

    // Print information about the structs
    for struct_ in structs {
        let type_ =  struct_.get_type().unwrap();
        let size = type_.get_sizeof().unwrap();

        if struct_.get_children().len() == 0 {
            continue;
        }

        println!("#[repr(C)]");
        println!("pub struct {} {{", struct_.get_name().unwrap());

        for field in struct_.get_children() {
            let name = field.get_name().unwrap();
            let t = field.get_type().unwrap();

			let func_ptr = is_type_function_ptr(&t);

			print!("    pub {}: " , name);

            if is_type_function_ptr(&t) {
            	generate_function_ptr(&field)
            } else {
            	generate_var(&field)
            }
        }

        println!("}}\n");
    }

    println!("extern \"C\" {{");
    println!("    pub fn wrui_get() -> *mut Wrui;\n");
    println!("}}");
}
