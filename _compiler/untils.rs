// --- import --- //
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

// --- defs --- //
pub fn sym_d() -> &'static [(char, char)] {
    &[
        ('|', '|'), ('&', '&'),
        ('+', '+'), ('-', '-'), ('*', '*'), ('/', '/'), ('<', '<'), ('>', '>'),
        (':', ':'), ('<', '-'), ('-', '>'),
        ('=', '='), ('!', '='), ('<', '='), ('>', '='),
        ('+', '='), ('-', '='), ('*', '='), ('/', '='), ('%', '='), ('^', '='), ('&', '='), ('|', '=')
    ]
}

pub fn sym_s() -> &'static [&'static str] {
    &[
        "{","}","[","]","(",")","<",">",
        "!","@","#","$","&","|",":",";",",",".","?",
        "*","-","+","=","%","^","/"
    ]
}

// pub fn sym_d() -> &'static [&'static str] {
//     &[
//         "||","&&",
//         "++","--","**","//","<<",">>",
//         "::","<-","->",
//         "==","!=","<=",">=",
//         "+=","-=","*=","/=","%=","^=","&=","|="
//     ]
// }

// --- position --- //
#[derive(Debug, Clone, Copy)]
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

// --- info ---
#[repr(C)]
pub struct Info {
    pub msg: *mut c_char,
}

#[macro_export]
macro_rules! show {
    ($fmt:expr) => {{
        let msg_string = format!($fmt); 
        let msg_c = std::ffi::CString::new(msg_string)
            .expect("failed to convert to CString");
        unsafe {
            push_info(msg_c.as_ptr());
        }
    }};
    
    ($fmt:expr, $($arg:tt)*) => {{
        let s = format!($fmt, $($arg)*);
        if let Ok(c_str) = CString::new(s) {
            unsafe { push_info(c_str.as_ptr()); }
        }
    }};
}

#[unsafe(no_mangle)]
pub extern "C" fn push_info(msg_ptr: *const c_char) -> Info {
    if msg_ptr.is_null() { 
        return Info { 
            msg: std::ptr::null_mut() 
        }; 
    }

    let c_str = unsafe { 
        CStr::from_ptr(msg_ptr) 
    };

    let msg = c_str.to_str().unwrap_or("Error: Invalid UTF-8");

    match CString::new(msg) {
        Ok(c_string) => Info { msg: c_string.into_raw() },
        Err(_) => Info { msg: std::ptr::null_mut() },
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn free_info(info: Info) {
    if !info.msg.is_null() {
        unsafe {
            let _ = CString::from_raw(info.msg);
        }
    }
}
