extern crate clang;

mod rust_ffi_gen;

use clang::*;

static INPUT_HEADER: &'static str = "../../native/external/wrui/include/wrui.h";
static RUST_FFI_FILE: &'static str = "../../../src/prodbg/wrui_rust/src/ffi.rs";

fn main() {
    // Acquire an instance of `Clang`
    let clang = Clang::new().unwrap();

    // Create a new `Index`
    let index = Index::new(&clang, false, false);

    // Parse a source file into a translation unit
    let tu = index.parser(INPUT_HEADER).parse().unwrap();

    // Get the structs in this translation unit also remove some stuff we don't need
    let structs = tu.get_entity().get_children().into_iter().filter(|e| {
    	let name = e.get_type().unwrap().get_display_name();
        e.get_kind() == EntityKind::StructDecl &&
        name.find("__").is_none() &&
        name.find("_opaque").is_none()
    }).collect::<Vec<_>>();

	if let Err(err) = rust_ffi_gen::generate_ffi_bindings(RUST_FFI_FILE, &structs) {
		panic!("Unable to generate {} err {:?}", RUST_FFI_FILE, err);
	}
}
