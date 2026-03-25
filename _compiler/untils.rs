// --- import --- //
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

// --- defs --- //
pub fn sym_s() -> &'static [&'static str] {
    &[
        "\'","\"","\n","\t",
        "{","}","[","]","(",")","<",">",
        "!","@","#","$","&","|",":",";",",",".","?",
        "*","-","+","=","%","^","/"
    ]
}

pub fn sym_d() -> &'static [&'static str] {
    &[
        "||","&&",
        "++","--","**","//","<<",">>",
        "::","<-","->",
        "==","!=","<=",">=",
        "+=","-=","*=","/=","%=","^=","&=","|="
    ]
}

// --- position --- //
pub struct Position {
    pub ln  :   usize,
    pub col :   usize,
    pub idx :   usize,
}

impl Position {
    pub fn new() -> Position {
        Position {
            ln  :   0,
            col :   0,
            idx :   0,
        }
    }

    pub fn upd(&mut self, _char: char) {
        self.idx += 1;
        self.col += 1;
        if _char == '\n' {
            self.ln += 1;
            self.col = 0;
        }
    }
}

// --- info --- //
#[repr(C)]
pub struct Info {pub msg: *mut c_char,}

pub fn show(text: &str) {
    let c_msg = CString::new(text).expect("CString failed");
    let info = info(c_msg.as_ptr());
    free(info);
}

#[unsafe(no_mangle)]
pub extern "C" fn info(msg_ptr: *const c_char) -> Info {
    if msg_ptr.is_null() { 
        return Info { msg: std::ptr::null_mut() }; 
    }

    let c_str = unsafe { CStr::from_ptr(msg_ptr) };
    let msg_content = c_str.to_str().unwrap_or("Error: Invalid UTF-8");
    
    let display_msg = format!("VNS Log: {}", msg_content);

    Info {msg: CString::new(display_msg).expect("CString failed").into_raw(),}
}

#[unsafe(no_mangle)]
pub extern "C" fn free(info: Info) {
    unsafe {if !info.msg.is_null() {
            let _ = CString::from_raw(info.msg);
    }}
}
