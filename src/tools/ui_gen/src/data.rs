extern crate clang;

#[derive(Debug)]
pub struct Variable {
	pub c_type: String,
	pub rust_type: String,
}

#[derive(Debug)]
pub struct Parameter {
	pub name: String,
	pub var: Variable,
}

#[derive(Debug)]
pub struct FuncPtr {
	pub function_args: Vec<Parameter>,
	pub return_val: Option<Variable>,
}

#[derive(Debug)]
pub enum StructEntry {
	Var(Variable),
	FunctionPtr(FuncPtr),
}

#[derive(Debug)]
pub struct Struct {
	pub name: String,
	pub entries: Vec<StructEntry>,
}

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
                format!("*const {}", &s[..s.len() - 1])
            } else {
                panic!("Unable to decode type {}", name);
            }
        }
    }
}

///
/// Generates a regular variable in a struct
///
fn generate_var(var: &clang::Entity) -> StructEntry {
    let t = var.get_type().unwrap();
	let display_name = t.get_display_name();

	let var = Variable {
		c_type: display_name.to_owned(),
		rust_type: map_c_type_to_rust(&display_name),
	};

	StructEntry::Var(var)
}

///
/// Generates a function pointer
///
fn generate_function_ptr(var: &clang::Entity) -> StructEntry {
    let t = var.get_type().unwrap();
    let display_name = t.get_display_name();

	let mut func_ptr = FuncPtr {
		function_args: Vec::new(),
		return_val: None,
	};

    // handle the return type

    let type_end = display_name.find("(*)").unwrap();	// will always be ok inside this function as we found this before
    let ret_type = &display_name[..type_end];

    if ret_type != "void " {
    	func_ptr.return_val = Some(Variable {
			c_type: ret_type.to_owned(),
			rust_type: map_c_type_to_rust(&ret_type.to_owned()),
		});
	}

    for c in var.get_children().iter() {
        let t = c.get_type().unwrap();

        if c.get_kind() == clang::EntityKind::TypeRef {
        	continue;
        }

        let param = Parameter {
			name: c.get_display_name().unwrap().to_owned(),
			var: Variable {
				c_type: t.get_display_name(),
				rust_type: map_c_type_to_rust(&t.get_display_name()),
			}
        };

        func_ptr.function_args.push(param);
    }

	StructEntry::FunctionPtr(func_ptr)
}

pub fn build_data(structs: &Vec<clang::Entity>) -> Vec<Struct> {
	let mut struct_entries = Vec::new();

    for struct_ in structs {
        if struct_.get_children().len() == 0 {
            continue;
        }

        let mut struct_data = Struct {
        	name: struct_.get_name().unwrap().to_owned(),
        	entries: Vec::new(),
        };

        for field in struct_.get_children() {
            //let name = field.get_name().unwrap();
            let t = field.get_type().unwrap();

            if is_type_function_ptr(&t) {
                struct_data.entries.push(generate_function_ptr(&field));
            } else {
                struct_data.entries.push(generate_var(&field));
            }
        }

		struct_entries.push(struct_data);
    }

	struct_entries
}

