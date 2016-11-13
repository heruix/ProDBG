extern crate clang;

use clang::*;

fn main() {
    // Acquire an instance of `Clang`
    let clang = Clang::new().unwrap();

    // Create a new `Index`
    let index = Index::new(&clang, false, false);

    // Parse a source file into a translation unit
    let tu = index.parser("../../native/external/wrui/include/wrui.h").parse().unwrap();

    // Get the structs in this translation unit
    let structs = tu.get_entity().get_children().into_iter().filter(|e| {
        e.get_kind() == EntityKind::StructDecl
    }).collect::<Vec<_>>();

    println!("use std::os::raw::{{c_void, c_int, c_char}}\n");

    // Print information about the structs
    for struct_ in structs {
        let type_ =  struct_.get_type().unwrap();
        let size = type_.get_sizeof().unwrap();
        //println!("struct: {:?} (size: {} bytes)", struct_.get_name().unwrap(), size);
    
        if struct_.get_children().len() == 0 {
            continue;
        }
        
        println!("#[repr(C)]");
        println!("pub struct {} {{", struct_.get_name().unwrap());

        for field in struct_.get_children() {
            println!("{:?}", field);
            let name = field.get_name().unwrap();

            println!("--------------------------------------------");

            for c in field.get_children() {
                println!("c      {:?}", c);
            }




            //let offset = type_.get_offsetof(&name).unwrap();
            println!("    pub {}", name);
        }

        println!("}}\n");
    }
}
