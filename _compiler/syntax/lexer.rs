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

    // --- Keywords (10 - 29) ---
    Block    = 10,                          // if, elif, else 
    Loop     = 11,                          // for, while 
    Struct   = 12,                          // struct
    Def      = 13,                          // def
    Class    = 14,                          // class
    Temp     = 15,                          // template
    Lib      = 16, 
    Pick     = 17, 
    Call     = 18, 
    Exec     = 19, 
    Tvar     = 20,
    Var      = 21,
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
    FloDiv   = 79,                          // // 
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
    pos     :   usize,
}

pub struct Lexer {
    pub source  : String,
    pub chars   : Vec<char>,
    pub tokens  : Vec<Token>,
    pub raw_toks: Vec<String>,

    pos     : Position,
    ops     : Opcodes,
}

impl Lexer {
    pub fn new(source: String) -> Lexer {
        Lexer {
            source  : source,
            chars   : Vec::new(),
            tokens  : Vec::new(),
            raw_toks: Vec::new(),

            pos     : Position::new(),
            ops     : Opcodes::new(),
        }
    }

    // ----- helper functions -----
    fn curr(&self) -> char {
        let offset = self.pos.idx;
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
            "**" => Tk::Pow,       "//" => Tk::FloDiv,
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

    pub fn check_syms(&mut self, idx: usize) -> bool {
        let char1 = self.chars[idx];

        if idx + 1 < self.chars.len() {
            let char2 = self.chars[idx + 1];
            let mut dou_sym = String::with_capacity(2);
            dou_sym.push(char1);
            dou_sym.push(char2);
            if sym_d().contains(&dou_sym.as_str()) {
                self.raw_toks.push(dou_sym);
                self.pos.upd(char1);
                self.pos.upd(char2);
                return true;
            }
        }

        if idx < self.chars.len() {
            let single_sym = char1.to_string();
            if sym_s().contains(&single_sym.as_str()) {
                self.raw_toks.push(single_sym);
                self.pos.upd(char1);
                return true;
            }
        }

        return false;
    }

    pub fn keyword(&mut self, mut idx: usize) -> bool {
        let mut keyword = String::new();

        while idx < self.chars.len() {
            let char1 = self.chars[idx];

            if char1.is_alphanumeric() || char1 == '_' {
                keyword.push(char1);
                self.pos.upd(char1);
                idx += 1;
            } else {break;}
        }

        if !keyword.is_empty() {
            self.raw_toks.push(keyword);
            return true;
        }

        false
    }

    pub fn make_str(&mut self, mut idx: usize, quo: char) {
        let mut string = String::new();
        
        self.raw_toks.push(quo.to_string());
        self.pos.upd(quo); idx += 1;

        while idx < self.chars.len() {
            let char1 = self.chars[idx];
            if char1 == quo {break;}
            string.push(char1);
            self.pos.upd(char1);
            idx += 1;
        }

        self.raw_toks.push(string);
        self.raw_toks.push(quo.to_string());
        self.pos.upd(quo);
    }

    pub fn make_num(&mut self) {
        let mut num = String::new();
        
        if self.peek(0) != '0' || !self.peek(1).is_alphabetic() {
            self.dec(&mut num);
            if !num.is_empty() {self.raw_toks.push(num);}
            return;
        }

        let prefix = self.peek(1).to_ascii_lowercase();
        
        if !['x', 'o', 'q', 'b'].contains(&prefix) {
            self.dec(&mut num);
            if !num.is_empty() {self.raw_toks.push(num);}
            return;
        }

        num.push(self.curr()); self.pos.upd(self.curr());
        num.push(self.curr()); self.pos.upd(self.curr());

        while self.pos.idx < self.chars.len() {
            let c = self.curr();
            let is_valid = match prefix {
                'x' => c.is_ascii_hexdigit(),
                'o' => ('0'..='7').contains(&c),
                'q' => ('0'..='3').contains(&c),
                'b' => ('0'..='1').contains(&c),
                _ => false,
            };
            
            if !is_valid { break; }
            num.push(c);
            self.pos.upd(c);
        }

        if !num.is_empty() {self.raw_toks.push(num);}
    }

    fn dec(&mut self, s: &mut String) {
        let mut is_float = false;

        while self.pos.idx < self.chars.len() {
            let c = self.curr();

            if c.is_ascii_digit() {
                s.push(c);
                self.pos.upd(c);
            } else if c == '.' && !is_float {
                if self.peek(1) == '.' {break;}

                is_float = true;
                s.push(c);
                self.pos.upd(c);
            } else {break;}
        }
    }

    pub fn raw_tokens(&mut self) {
        self.chars = self.source.chars().collect();

        let mut idx = 0;

        while idx < self.chars.len() {
            idx = self.pos.idx;
            //-println!("[{}]|[{}]", idx, self.chars.len());
            if idx >= self.chars.len() {break;}
            let _char = self.chars[idx];

            if _char.is_whitespace() && _char != '\n' {
                self.pos.upd(_char);
                continue;
            }

            if _char == '\n' {
                self.raw_toks.push("\\n".to_string());
                self.pos.upd(_char);continue;
            }

            if _char.is_ascii_digit() {self.make_num();continue;}

            if _char == '\"' || _char == '\'' {self.make_str(idx, _char); continue;}

            if self.check_syms(idx) {continue;}

            if self.keyword(idx) {continue;}

            self.pos.upd(_char);
        }

			// for tok in self.raw_toks.clone() {
			//     if tok == "\\n" {
			//         println!();
			//         continue;
			//     }
			//     print!(" {} ", tok);
			// }
    }

    pub fn make_token(&mut self) {
        self.raw_tokens();

        let raw_tokens = std::mem::take(&mut self.raw_toks);

        for tok in raw_tokens {
            let tag = self.token_tag(&tok); 
            
            let token = Token {
                kind    : tag as u8,
                value   : tok, 
                pos     : self.pos.idx,
            };
            self.tokens.push(token);

            // for token in &self.tokens {
            //     println!("{:?}", token);
            // }
        }
    }
}
