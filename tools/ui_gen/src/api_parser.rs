use std::fs::File;
use std::io::prelude::*;
use pest::prelude::*;

impl_rdp! {
    grammar! {
        chunk = { block ~ eoi }
        block = { stat* }

        stat = {
            whitespace |
            comment |
            tableconstructor
        }

        tableconstructor =  { name ~ ["{"] ~ fieldlist? ~ ["}"] }
        fieldlist        =  { field ~ (fieldsep ~ field)* ~ fieldsep* }
        field            =  { var | function }
        fieldsep         = _{ [","] }

        ftype            = { name }
        rettype          = { name }
        callback         = { ["[callback]"] }
        retexp           = { ["->"] ~ name }
        var              = { ftype ~ name }
        varlist          = { var ~ ([","] ~ var)* }
        function         = { callback? ~ name ~ ["("] ~ varlist? ~ [")"] ~ retexp? }

        name = @{
            (['a'..'z'] | ['A'..'Z'] | ["_"]) ~ (['a'..'z'] | ['A'..'Z'] | ["_"] | ['0'..'9'])*
        }

       comment = _{
            ["//"] ~ (!(["\r"] | ["\n"]) ~ any)* ~ (["\n"] | ["\r\n"] | ["\r"] | eoi)
        }

        whitespace = _{ [" "] | ["\t"] | ["\u{000C}"] | ["\r"] | ["\n"] }
    }
}

#[derive(Debug)]
pub struct Variable<'a> {
    pub name: &'a str,
    pub vtype: &'a str,
}

pub struct Function<'a> {
    pub name: &'a str,
    pub function_args: Vec<Variable<'a>>,
    pub return_val: Option<Variable<'a>>,
}

#[derive(Debug)]
pub enum StructEntry {
    Var(Variable),
    Function(Function),
}

#[derive(Debug)]
pub struct Struct<'a> {
    name: &'a str, 
    inharit: Option<&'a str>,
    entries: Vec<StructEntry<'a>>,
}

#[derive(Debug)]
pub struct ApiDef<'a> {
    pub entries: Vec<Struct<'a>>,
}

impl ApiDef {
    pub parse_file<P: AsRef<Path>>(path: P) -> Result<()>
        let mut file = File::open(path)?;
        file.read_to_string(&mut self.text)?;

        let mut parser = Rdp::new(StringInput::new(&self.text));

        assert!(parser.block());
        assert!(parser.end());

        for token in parser.queue() {
            println!("{:?}", token);
        }

        // build up tokens here
    }
}


/*
fn main() {
    let mut parser = Rdp::new(StringInput::new(
        "
        // This is a comment!
        Rect {
           f32 x,
           f32 y,
           f32 width,
           f32 height,
       }

       // This struct has some functions and callbacks
       Foo {
            test2(i32 test, u32 foo),
            // Another comment in the \"struct\"
            [callback] test1(Rect test) -> void,
            i32 foo,
        }

        // Empty struct
        Table {

        }"));

    assert!(parser.block());
    assert!(parser.end());

    for token in parser.queue() {
        println!("{:?}", token);
    }
}
*/
