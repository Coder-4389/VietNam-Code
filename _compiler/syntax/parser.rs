// --- impoert --- //
use crate::_init::*;

// --- struct --- //
pub struct Paser {
    pub tokens  :   Vec<Token>,
    pub idx     :   usize,

    pub kw_struct : HashSet<String>,
    pub kw_temp   : HashSet<String>,
    pub kw_def    : HashSet<String>,
    pub kw_class  : HashSet<String>,
    pub kw_var    : HashSet<String>,
    pub kw_lib    : HashSet<String>,

    pub ops     :   Opcodes,
}

impl Paser {
    pub fn peek(&self) -> &Token {&self.tokens[self.idx + 1]}
    pub fn curr(&self) -> &Token {&self.tokens[self.idx]}
    pub fn is_end(&self) -> bool {self.idx >= self.tokens.len()}

    pub fn consume(&mut self, kind: TokType) -> bool {
        if !self.is_end() && self.curr().kind == kind {
            self.idx += 1;
            true 
        } false 
    }

    pub fn kind_upd(&mut self, idx: usize, _kind: Tk) {
        if idx < self.tokens.len() {self.tokens[idx].kind = _kind;}
    }

    pub fn new(tokens: Vec<Token>) -> Paser {
        Paser {
            tokens  :   tokens,
            idx     :   0,

            kw_struct :   HashSet::new(),
            kw_temp   :   HashSet::new(),
            kw_def    :   HashSet::new(),
            kw_class  :   HashSet::new(),
            kw_var    :   HashSet::new(),
            kw_lib    :   HashSet::new(),

            ops     :   Opcodes::new(),
        }
    }

    // *********************************************************************
    // --- define functions ---
    // *********************************************************************
    pub fn is_incl(&mut self) {
		self.consume(Tk::incl);
		
		if !self.peek().kind == Tk::DQuote {
            return;
        } 
        self.consume(Tk::DQuote);
		
		let path = self.peek().value;
		if !Path::new(path).exists() {
            return;
        } 
        self.consume(Tk::String);
		
		if !self.peek().kind == Tk::DQuote {
            return;
        } 
        self.consume(Tk::DQuote);
    }

    pub fn is_temp(&mut self) {
        self.consume(Tk::Temp);

        let mut name = String::new();

        if self.curr().kind == Tk::Id {
            name = self.curr().value;
            self.kw_temp.insert(name);

            self.kind_upd(self.idx, Tk::Name);
            self.consume(Tk::Name);
        }

        if self.curr().kind == Tk::L4 {
            self.consume(Tk::L4)
        }

        while !self.is_end() {
            let kind = self.curr().kind;

            match kind {
                Tk::Tvar => {
                    self.consume(kind);
                    if self.curr().kind == Tk::Comma {
                        self.consume(Tk::Comma);
                    }
                }
                Tk::R4 => {
                    break;
                }

                _ => info("the template at line {} don't have ")
            } 
        }  
    }

    pub fn is_def(&mut self) {
        self.consume(Tk::Def);

        let mut name = String::new();

        if self.curr().kind == Tk::Id {
            name = self.curr().value.clone();
            self.consume(Tk::Id);
        }

        if self.curr().kind == Tk::L1 {
            self.consume(Tk::L1);
        }

        while !self.is_end() && self.curr().kind != Tk::R1 {
            match self.curr().kind {
                Tk::Id => {
                    self.iden_check();
                },
                Tk::Tvar => {
                    self.param_check();
                }
                Tk::Comma => {
                    self.consume(Tk::Comma);
                },
                _ => break,
            }
        } 
        
        self.consume(Tk::R1);

        if !name.is_empty() {
            self.kw_def.insert(name);
        }
    }

    pub fn is_class(&mut self) {
        self.consume(Tk::Class);

        let mut name = String::new();

        if self.curr().kind == Tk::Id {
            name = self.curr().value.clone();
            self.kw_class.insert(name);

            self.kind_upd(self.idx, Tk::Name);
            self.consume(Tk::Name);
        }

        if self.curr().kind == Tk::L3 {
            self.consume(Tk::L3);
        }

        while !self.is_end() && self.curr().kind != Tk::L3 {
            let kind = self.curr().kind;

            match kind {
                Tk::Tvar    => self.is_asg(),
                Tk::Def     => self.is_def(),
                Tk::Class   => self.is_class();

                _  => break;
            }
        }

        self.consume(Tk::L3);
    } 

    pub fn is_asg(&mut self) {
        self.consume(Tk::Tvar);

        let mut name = String::new();

        if self.curr().kind == Tk::Id {
            name = self.curr().value.clone();
            self.kw_var.insert(name);
            
            self.kind_upd(self.idx, Tk::Name);
            self.consume(Tk::Name);
        }

        if self.curr().kind == Tk::Asg {
            self.consume(Tk::Asg);
        }

        self.asg_check();
    }

    // *********************************************************************
    // --- order functions ---
    // *********************************************************************
    pub fn is_lib(&mut self) {

    }
    
    pub fn is_call(&mut self) {

    }

    pub fn is_var(&mut self) {

    }
    
    // *********************************************************************
    // --- checker functions ---
    // *********************************************************************
    pub fn iden_check(&mut self) -> bool {
        let val = &self.curr().value;

        self.consume(Tk::Id);
        if self.is_end() { 
            return false; 
        }
        
        let kind = self.curr().kind;

        match kind {
            Tk::L1 if self.ops.exec.contains_key(&val) => {
                self.kind_upd(self.idx, Tk::Exec);
            },
            Tk::Scop if self.kw_lib.contains(&val) => {
                self.kind_upd(self.idx, Tk::Lib);
            },
            Tk::Dot if self.kw_class.contains(&val) => {
                self.kind_upd(self.idx, Tk::Call);
            },
            Tk::L1 if self.kw_def.contains(&val) => {
                self.kind_upd(self.idx, Tk::Call);
            },
            Tk::Comma if self.kw_var.contains(&val) => {
                self.kind_upd(self.idx, Tk::Name);
            },

            _ => false,
        }
        true
    }

    pub fn param_check(&mut self) {

    }

    pub fn data_check(&mut self) -> bool {
        let kind = self.curr().kind;
        let val = self.curr().value;

        match kind {
            Tk::DQuote | Tk::SQuote => {
                self.consume(kind);
                self.consume(Tk::String);
                self.consume(kind);
            }
            Tk::Number => {
                self.consume(kind);
            }
            Tk::Sub => {
                self.consume(kind);
            }
            Tk::True | Tk::False => {
                self.consume(kind);
            }
            Tk::Id => {
                if kw_lib.contains(val) {
                    self.is_lib();
                }
                else if kw_class.contains(val) {
                    self.is_call();
                }
                else if kw_def.contains(val) {
                    self.is_call();
                }
                else if kw_var.contains(val) {
                    self.is_var();
                }
                else {
                    consume(kind)
                }
            }
            Tk::L1 => {
                self.consume(Tk::L1)
                self.asg_check();
                self.consume(Tk::R1)
            }
            Tk::None => {
                self.consume(Tk::None)
            }

            _ => false
        }
    }

    pub fn asg_check(&mut self) {
        self.data_check();

        while !self.is_end() && self.curr().kind != Tk::Semi {
            let kind = self.curr().kind;
            match kind {
                Tk::Add | Tk::Sub | Tk::Mul | Tk::Div | Tk::Mod | Tk::Pow | Tk::IDiv => {
                    self.consume(kind);
                    self.data_check();
                }
                _ => break;
            }
        }
        
        self.consume(Tk::Semi);
    }

    pub fn analyzer(&mut self) {

    }
}
