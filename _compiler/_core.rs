// --- library --- //
use std::fs;
use std::io::{self, Write};
use std::ffi::c_char;
use std::env;
use std::path::Path;

pub mod untils;
pub mod opcode;
pub mod syntax;
pub mod compiler;

mod _init;
use _init::*;
// --- struct --- //
struct Core {
    path    :    String,
    source  :    String,
}

impl Core {
    fn new(path: String) -> Core {
        Core {
            path    :   path,
            source  :   String::new(),
        }
    }

    fn load_cdx(&mut self, path: String) {
        match fs::read_to_string(&self.path) {
            Ok(source) => {
                self.source = source;
                show("Successfully read file.");
            } Err(e) => {
                show("Failed to read file.");
                let info = &format!("Error: {}", e);
                show(info);
            }
        }
    }

    fn run(&mut self) {
        let path = input("Enter the file path: ");
        self.path = path;
        self.load_cdx();
    }
}

fn main() {
    let mut _core = Core::new(String::new());
    let mut lexer = Lexer::new(_core.source.clone());
	
	let arg: Vec<String> = env::arg().collect();
	
	if arg[0] != "vnc" {return;}
	if arg[1] != "run" {return;}
	
	if len(arg) < 3 {return;}
	
	let path = arg[2];
    _core.run(path);
	
    lexer.make_token();
}
