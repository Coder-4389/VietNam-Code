// --- library --- //
use std::fs;
use std::io::{self, Write};
use std::ffi::c_char;

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

// --- until --- //
fn input(msg: &str) -> String {
    println!("{}", msg);
    let mut input = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input);
    input.trim().to_string()
}

impl Core {
    fn new(path: String) -> Core {
        Core {
            path    :   path,
            source  :   String::new(),
        }
    }

    fn load_cdx(&mut self) {
        if !self.path.ends_with(".vnc") {
            println!("This file is not vnc format. You want to run this file. \n");
            let des = input(" Y|N : Yes|No").to_lowercase();
            if des == "n" || des == "no" { 
                return;
            }
        }

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
    _core.run();
    let mut lexer = Lexer::new(_core.source.clone());
    lexer.make_token();
}
