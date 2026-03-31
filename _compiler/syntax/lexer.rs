// --- import --- //
use crate::_init::*;

// --- struct --- //
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Tk {
    // --- Data (0 - 9) ---
    Id       = 0, 
    Number   = 1, 
    String   = 2, 
    True     = 3,
    False    = 4,
    None     = 5,

    // --- Keywords (10 - 29) ---
    Block    = 10,                          // if, elif, else 
    Loop     = 11,                          // for, while 
    Def      = 12,                          // def
    Class    = 13,                          // class
    Temp     = 14,                          // template
    Struct   = 15,
    Lib      = 16, 
    Pick     = 17, 
    Call     = 18, 
    Exec     = 19, 
    Tvar     = 20,
    Name     = 21,
    Incl     = 22,

    // --- Symbols Single (30 - 69) ---
    L4       = 30, R4 = 31,                 // < > 
    L3       = 32, R3 = 33,                 // { } 
    L2       = 34, R2 = 35,                 // [ ]  
    L1       = 36, R1 = 37,                 // ( ) 
    Comma    = 38,                          // , 
    Dot      = 39,                          // . 
    Semi     = 40,                          // ; 
    Colon    = 41,                          // : 
    Quest    = 42,                          // ? 
    At       = 43,                          // @ 
    Hash     = 44,                          // #
    Dol      = 45,                          // $ 
    Amp      = 46,                          // & 
    Pipe     = 47,                          // | 
    
    Add      = 48, Sub = 49, Mul = 50, Div = 51, Mod = 52, // + - * / % 
    Asg      = 53,                          // = 
    Not      = 54,                          // ! 
    SQuote   = 55,                          // ' 
    DQuote   = 56,                          // " 

    // --- Symbols Double (70 - 110) ---
    Eq       = 70, Neq = 71,                // == != 
    Lte      = 72, Gte = 73,                // <= >= 
    And      = 74, Or  = 75,                // && || 
    Inc      = 76, Dec = 77,                // ++ -- 
    Pow      = 78,                          // ** 
    IDiv     = 79,                          // // 
    LShf     = 80, RShf = 81,               // << >> 
    Scop     = 82,                          // :: 
    ArwL     = 83, ArwR = 84,               // <- -> 
    
    // --- Assignment Operators (120 - 150) ---
    Ag_Ad    = 120, Ag_Sb = 121,            // += -= 
    Ag_Ml    = 122, Ag_Dv = 123,            // *= /=
    Ag_Md    = 124, Ag_Pw = 125,            // %= **= 
    Ag_Fd    = 126, Ag_Xor = 127,           // //= ^= 
    Ag_And   = 128, Ag_Or = 129,            // &= |= 

    Unknown = 255, 
}

#[derive(Debug, Clone)]
pub struct Token {
    kind    :   u8,
    value   :   String,
    pos     :   Position,
}

pub struct Lexer {
    pub source  : String,
    pub chars   : Vec<char>,
    pub tokens  : Vec<Token>,

    pos     : Position,
    ops     : Opcodes,
}

impl Lexer {
    pub fn new(source: String) -> Lexer {
        Lexer {
            source  : source,
            chars   : Vec::new(),
            tokens  : Vec::new(),

            pos     : Position::new(),
            ops     : Opcodes::new(),
        }
    }

    // ----- helper functions -----
    fn curr(&self) -> char {
        let offset = self.pos.clone().idx;
        if offset < self.chars.len() {self.chars[offset]} 
        else {return '\0';}
    }
    fn peek(&self, mut offset: usize) -> char {
        offset += self.pos.idx;
        if offset < self.chars.len() {self.chars[offset]}
        else {return '\0';}
    }
    // ----------------------------

    pub fn token_tag(&mut self, tok: &str) -> Tk {
        match tok {
            "true"                  => Tk::True,
            "false"                 => Tk::False,
            "none"                  => Tk::None,
            // --- Keywords ---
            "if"                    => Tk::Block,
            "elif"                  => Tk::Block,
            "else"                  => Tk::Block,
            "for"                   => Tk::Loop,
            "while"                 => Tk::Loop,
            "struct"                => Tk::Struct,
            "def"                   => Tk::Def,
            "class"                 => Tk::Class,
            "template"              => Tk::Temp,
            "include"               => Tk::Incl,

            // --- Symbols Single ---
            "<"  => Tk::L4,        ">"  => Tk::R4,
            "{"  => Tk::L3,        "}"  => Tk::R3,
            "["  => Tk::L2,        "]"  => Tk::R2,
            "("  => Tk::L1,        ")"  => Tk::R1,
            ","  => Tk::Comma,     "."  => Tk::Dot,   
            ";"  => Tk::Semi,      ":"  => Tk::Colon,  
            "?"  => Tk::Quest,     "@"  => Tk::At ,    
            "#"  => Tk::Hash,      "$"  => Tk::Dol,
            "&"  => Tk::Amp,       "|"  => Tk::Pipe,
            "+"  => Tk::Add,       "-"  => Tk::Sub,   
            "*"  => Tk::Mul,       "/"  => Tk::Div,    
            "%"  => Tk::Mod,       "="  => Tk::Asg,
            "!"  => Tk::Not,    
            "\'" => Tk::SQuote,    "\"" => Tk::DQuote,

            // --- Symbols Double ---
            "==" => Tk::Eq,        "!=" => Tk::Neq,
            "<=" => Tk::Lte,       ">=" => Tk::Gte,
            "&&" => Tk::And,       "||" => Tk::Or,
            "++" => Tk::Inc,       "--" => Tk::Dec,
            "**" => Tk::Pow,       "//" => Tk::IDiv,
            "<<" => Tk::LShf,      ">>" => Tk::RShf,
            "::" => Tk::Scop,   
            "<-" => Tk::ArwL,      "->" => Tk::ArwR,

            // --- Assignment Operators ---
            "+=" => Tk::Ag_Ad,     "-=" => Tk::Ag_Sb,
            "*=" => Tk::Ag_Ml,     "/=" => Tk::Ag_Dv,
            "%=" => Tk::Ag_Md,     "**="=> Tk::Ag_Pw,
            "//="=> Tk::Ag_Fd,     "^=" => Tk::Ag_Xor,
            "&=" => Tk::Ag_And,    "|=" => Tk::Ag_Or,

            // --- Dynamic Checking ---
            _ if self.ops.exec.contains_key(tok) => Tk::Exec,
            _ if self.ops.tvar.contains_key(tok) => Tk::Tvar,

            // --- Data Recognition ---
            _ if tok.chars().next().map_or(false, |c| c.is_ascii_digit()) => Tk::Number,
            _ if tok.chars().next().map_or(false, |c| c.is_alphabetic() || c == '_') => Tk::Id,

            _ => Tk::Unknown,
        }
    }

    pub fn is_sym(&mut self) {
        let c1 = self.curr();
        if c1 == '\0' { return; }

        let sym_pos = self.pos.clone();

        if let Some(&c2) = self.chars.get(self.pos.idx + 1) {
            if sym_d().contains(&(c1, c2)) { 
                let mut content = String::with_capacity(2);
                content.push(c1);
                content.push(c2);

                self.tok_push(&content, sym_pos);
                self.pos.upd(c1);
                self.pos.upd(c2);
                return;
            }
        }

        let c1_str = c1.to_string(); 
        if sym_s().contains(&c1_str.as_str()) {
            self.tok_push(&c1_str, sym_pos);
            self.pos.upd(c1);
            return;
        }

        self.pos.upd(c1);
    }

    pub fn make_ident(&mut self) {
        let mut keyword = String::new();

        let keyw_pos = self.pos.clone();

        while self.pos.idx < self.chars.len() {
            let c = self.curr();
            if c.is_alphanumeric() || c == '_' {
                keyword.push(c);
                self.pos.upd(c); 
                continue;
            } break;
        }

        if !keyword.is_empty() {
            self.tok_push(&keyword, keyw_pos);
        }
    }

    pub fn make_str(&mut self, quo: char) {
        let quo_pos = self.pos.clone();
        let _quo = quo.to_string();
        self.tok_push(&_quo, quo_pos);
        self.pos.upd(quo);

        let str_pos = self.pos.clone();
        let mut string = String::new();

        while self.pos.idx < self.chars.len() {
            let c = self.curr();
            if c == quo { 
                break; 
            }
            string.push(c);
            self.pos.upd(c);
        } self.tok_push(&string, str_pos);

        if self.pos.idx < self.chars.len() && self.curr() == quo {
            let quo_pos = self.pos.clone();
            let _quo = quo.to_string();
            self.tok_push(&_quo, quo_pos);
            self.pos.upd(quo);
        }
    }

    pub fn make_num(&mut self) {
        let mut num = String::new();

        let num_pos = self.pos.clone();

        let c1 = self.peek(0);
        let c2 = self.peek(1);

        if c1 == '0' && ['x','o','q','b'].contains(&c2) {
            num.push(c1);
            self.pos.upd(c1);
            num.push(c2);
            self.pos.upd(c2);
            match c2  {
                'x'     => self.hex(&mut num),
                'o'     => self.oct(&mut num),
                'q'     => self.qua(&mut num),
                'b'     => self.bin(&mut num),
                _       => unreachable!(),
            }
        } else {
            self.dec(&mut num);
        }

        if !num.is_empty() {
            self.tok_push(&num, num_pos);
        }
    }

    pub fn hex(&mut self, num: &mut String) {
        while let Some(&c) = self.chars.get(self.pos.idx) {
            if c.is_ascii_hexdigit() {
                num.push(c);
                self.pos.upd(c);
            } else {
                break;
            }
        }
    }
    
    pub fn dec(&mut self, num: &mut String) {
        let mut is_float = false;

        while let Some(&c) = self.chars.get(self.pos.idx) {
            if c.is_ascii_digit() {
                num.push(c);
            } else if c == '.' && !is_float && self.peek(1) != '.' {
                is_float = true;
                num.push(c);
            } else {
                break;
            } self.pos.upd(c);
        }
    }
    
    pub fn oct(&mut self, num: &mut String) {
        while let Some(&c) = self.chars.get(self.pos.idx) {
            if ('0'..='7').contains(&c) {
                num.push(c);
                self.pos.upd(c);
            } else {
                break;
            }
        }
    }

    pub fn qua(&mut self, num: &mut String) {
        while let Some(&c) = self.chars.get(self.pos.idx) {
            if ('0'..='3').contains(&c) {
                num.push(c);
                self.pos.upd(c);
            } else {
                break;
            }
        }
    }

    pub fn bin(&mut self, num: &mut String) {
        while let Some(&c) = self.chars.get(self.pos.idx) {
            if ('0'..='1').contains(&c) {
                num.push(c);
                self.pos.upd(c);
            } else {
                break;
            }
        }
    }

    pub fn tok_push(&mut self, val: &str, _pos: Position) {
        let tag = self.token_tag(val); 
        self.tokens.push(Token{
            kind: tag as u8,
            value: val.to_string(),
            pos: _pos
        });
    }

    pub fn make_token(&mut self) {
        // Debug: Kiểm tra xem có chữ nào để đọc không
        if self.chars.is_empty() {
            println!("file is empty");
            return;
        }

        while self.pos.idx < self.chars.len() {
            let c = self.curr();
            let be_idx = self.pos.idx;

            match c {
                _ if c.is_whitespace() => {
                    self.pos.upd(c); 
                }
                _ if c.is_ascii_digit() => {
                    self.make_num();
                }
                _ if c.is_alphabetic() || c == '_' => {
                    self.make_ident();
                }
                '"' | '\'' => {
                    self.make_str(c);
                }
                _ => {
                    self.is_sym();
                }
            }

            if self.pos.idx == be_idx {
                self.pos.upd(c); 
            }
        }
        
        // In kết quả sau khi quét xong
        println!("--- Token list ({}) ---", self.tokens.len());
        for token in &self.tokens {
            println!("{:?}", token);
        }
    }
}
