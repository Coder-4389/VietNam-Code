// --- impoert --- //
use crate::_init::*;

// --- struct --- //
pub struct Paser {
    pub tokens  :   Vec<Token>,
    pub idx     :   usize,

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
    pub fn adv(&mut self) -> &Token {
        if !self.is_end() {self.idx += 1;}
        &self.tokens[self.idx - 1]
    }

    pub fn consume(&mut self, kind: TokType) -> bool {
        if !self.is_end() && self.curr().kind == kind {
            self.adv(); 
            return true; 
        } return false; 
    }

    pub fn kind_upd(&mut self, idx: usize, _kind: Tk) {
        if idx < self.tokens.len() {self.tokens[idx].kind = _kind;}
    }

    pub fn new(tokens: Vec<Token>) -> Paser {
        Paser {
            tokens  :   tokens,
            idx     :   0,

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
                    self.consume(Tk::Id);
                },
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

    }

    pub fn is_asg(&mut self) {
		self.consume(Tk::Tvar);

        self.kind_upd(self.idx, Tk::Var);
        self.consume(Tk::Var);

        if self.curr().kind == Tk::Asg {
            self.consume(Tk::Asg);
        }

        self.asg_check()
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
        if self.is_end() { return false; }
        let kind = self.peek().kind;
        let val = &self.curr().value;

        match kind {
            Tk::L1 if self.ops.exec.contains_key(&val)      => {
                self.kind_upd(self.idx, Tk::Exec);
            },
            Tk::Scop if self.kw_lib.contains(&val)          => {
                self.kind_upd(self.idx, Tk::Lib);
            },
            Tk::Dot if self.kw_class.contains(&val)         => {
                self.kind_upd(self.idx, Tk::Call);
            },
            Tk::L1 if self.kw_def.contains(&val)            => {
                self.kind_upd(self.idx, Tk::Call);
            },
            Tk::Comma if self.kw_var.contains(&val)         => {
                self.kind_upd(self.idx, Tk::Var);
            },

            _ => false,
        }

        true
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
                self.consume(Tk::Number);
            }
            Tk::True | Tk::False => {
                self.consume(kind)
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

    }

    pub fn _checker(&mut self) {

    }
}
