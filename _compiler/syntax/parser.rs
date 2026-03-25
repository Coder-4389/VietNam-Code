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
    pub fn is_impl(&mut self) {

    }

    pub fn is_def(&mut self) {
        self.consume(Tk::Def)

        let mut name = String::new();

        if self.curr().kind == Tk::Id {
            name = self.curr().valu.clone();
            self.consume(Tk::Id);
        }
        if self.curr().kind == Tk::L1 {self.consume(Tk::L1);}

        let kind = self.curr().kind;

        while kind != Tk::R1 {
            self.adv();
            let kind = self.curr().kind;

            match kind {
                Tk::Id => {
                    self.id_checker();
                    self.consume(Tk::Id);
                },
                Tk::Comma => {
                    self.consume(Tk::Comma);
                },
                _ => break,
            }
        }

        self.consume(Tk::R1);

        self.kw_def.insert(name);
    }

    pub fn is_class(&mut self) {

    }

    pub fn is_asg(&mut self) {

    }

    // *********************************************************************
    // --- order functions ---
    // *********************************************************************
    pub fn is_call(&mut self) {

    }

    pub fn is_var(&mut self) {

    }
    
    // *********************************************************************
    // --- checker functions ---
    // *********************************************************************
    pub fn id_checker(&mut self) -> bool {
        if self.is_end() { return false; }
        let kind = self.peek().kind;
        let val = &self.curr().value;

        match kind {
            Tk::L1 if self.ops.exec.contains_key(&val)      => {
                self.kind_upd(self.idx, Tk::Exec);
                true
            },
            Tk::Scop if self.kw_lib.contains(&val)          => {
                self.kind_upd(self.idx, Tk::Lib);
                true
            },
            Tk::Dot if self.kw_class.contains(&val)         => {
                self.kind_upd(self.idx, Tk::Pick);
                true
            },
            Tk::L1 if self.kw_def.contains(&val)            => {
                self.kind_upd(self.idx, Tk::Call);
                true
            },
            Tk::Comma if self.kw_var.contains(&val)         => {
                self.kind_upd(self.idx, Tk::Var);
                true
            },

            _ => {false},
        }
    }

    pub fn _checker(&mut self) {

    }
}