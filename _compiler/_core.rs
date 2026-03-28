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

    fn load_cdx(&mut self) {
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
        self.load_cdx();
    }
}

fn main() {
    let mut _core = Core::new(String::new());
    let mut lexer = Lexer::new(_core.source.clone());
	
	let args: Vec<String> = env::args().collect();
	
	if args[0] != "vnc" {return;}
	if args[1] != "run" {return;}
	if args.len() < 3 {return;}
	
	_core.path = args[2].clone();
    _core.run();
	
    lexer.make_token();
}
