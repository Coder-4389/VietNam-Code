// --- import --- //
use std::collections::HashMap; 

// --- struct --- //
pub struct Opcodes {
    pub ops     :   HashMap<&'static str, u8>,
    pub tvar    :   HashMap<&'static str, u8>,
    pub exec    :   HashMap<&'static str, u16>,
}

impl Opcodes {
    pub fn new() -> Opcodes {
        // Khởi tạo HashMap tiêu chuẩn
        let mut ops = HashMap::new();
        let mut tvar = HashMap::new();
        let mut exec = HashMap::new();

        // opcodes 
        ops.insert("EXIT"  , 0x00);

        // type var
        tvar.insert("void" , 0x00);
        tvar.insert("bool" , 0x01);
        tvar.insert("i8"   , 0x02);
        tvar.insert("i16"  , 0x03);
        tvar.insert("i32"  , 0x04);
        tvar.insert("i64"  , 0x05);
        tvar.insert("f32"  , 0x06);
        tvar.insert("f64"  , 0x07);
        tvar.insert("u8"   , 0x08);
        tvar.insert("u16"  , 0x09);
        tvar.insert("u32"  , 0x0A);
        tvar.insert("u64"  , 0x0B);
        tvar.insert("str8" , 0x0C);
        tvar.insert("str16", 0x0D);
        tvar.insert("c8"   , 0x0E);
        tvar.insert("c16"  , 0x0F);

        tvar.insert("int"   , 0x10);
        tvar.insert("uint"  , 0x11);
        tvar.insert("float" , 0x12);
        tvar.insert("str"   , 0x13);
        tvar.insert("char"  , 0x14);

        // execute
        exec.insert("print"     , 0x00);
        exec.insert("input"     , 0x01);
        exec.insert("clear"     , 0x02);
        exec.insert("color"     , 0x03);

        Opcodes {
            ops,
            tvar,
            exec,
        }
    }
}