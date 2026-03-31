// --- library --- //
use std::fs;
use std::io::{self, Write};
use std::ffi::c_char;
use std::env;
use std::path::Path;
use std::ffi::CString;

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
        let msg = match fs::read_to_string(&self.path) {
            Ok(source) => {
                self.source = source;
                CString::new("Successfully read file.").unwrap()
            } 
            Err(_) => {
                CString::new("Failed to open the file").unwrap()
            }
        };

        unsafe {
            info(msg.as_ptr());
        }
    }

    fn parser(&mut self, code: Vec<String>) {

    }

    fn run(&mut self) {
        self.load_cdx();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 3 { 
        return; 
    } 

    if args[1] == "run" {
        let mut _core = Core::new(args[2].clone());
        _core.run(); 
    
        let mut lexer = Lexer::new(_core.source.clone());
        lexer.make_token();
    }
}