use crate::grammar::TokenKind;


use std::collections::HashMap;
use std::str::CharIndices;

#[derive(Debug, PartialEq)]
pub enum Error {
    EOF,
    Unmatch,
}

pub struct Lexer<'a> {
    input: &'a str,
    cmap: Vec<usize>,

    cmap2: HashMap<usize, usize>,

    start: CharIndices<'a>,
    current: CharIndices<'a>,
    max_len: usize,


    zz_state: usize,
    zz_lexical_state: usize,

    // byte
    zz_marked_pos: usize,
    zz_current_pos: usize,
    zz_start_read: usize,

    // char
    zz_start_read_char: usize,
    zz_marked_char: usize,

    zz_at_eof: bool,

}

impl<'a> Lexer<'a> {
    pub const ZZ_ROW: [usize; 53] = [0, 37, 74, 111, 148, 185, 222, 259, 296, 333, 370, 407, 296, 296, 296, 296, 296, 296, 296, 444, 481, 518, 296, 296, 555, 592, 629, 666, 703, 740, 777, 814, 296, 296, 296, 296, 296, 296, 851, 888, 74, 925, 962, 999, 999, 1036, 74, 1073, 1110, 851, 74, 1147, 74];
    pub const ZZ_TRANS: [i32; 1184] = [-1, 1, 2, 2, 3, 2, 2, 2, 4, 2, 2, 5, 6, 7, 2, 8, 2, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 23, 26, -1, -1, 2, 27, 2, 2, 2, 2, 2, 2, 2, 2, -1, 2, -1, 2, -1, 2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, -1, 2, -1, 2, -1, 2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 2, 2, 2, 2, 28, 2, 2, 2, 2, 2, -1, 2, -1, 2, -1, 2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 2, 2, 29, 2, 2, 2, 2, 2, 2, 2, -1, 2, -1, 2, -1, 2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 6, 7, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 6, 30, 31, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 30, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, -1, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 32, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 33, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 34, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 35, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 36, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 37, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 23, 23, 23, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 23, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 38, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 39, 2, 40, 2, 2, 2, 2, 2, 2, 2, -1, 2, -1, 2, -1, 2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 2, 2, 2, 2, 2, 41, 2, 2, 2, 2, -1, 2, -1, 2, -1, 2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 2, 2, 2, 2, 2, 2, 2, 2, 42, 2, -1, 2, -1, 2, -1, 2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 30, -1, 31, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 43, 44, -1, -1, 43, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 38, 38, -1, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 45, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, -1, -1, 2, 2, 46, 2, 2, 2, 2, 2, 2, 2, -1, 2, -1, 2, -1, 2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 2, 2, 2, 2, 2, 2, 47, 2, 2, 2, -1, 2, -1, 2, -1, 2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 48, 2, 2, 2, 2, 2, 2, 2, 2, 2, -1, 2, -1, 2, -1, 2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 44, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 38, 38, -1, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 45, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 49, -1, -1, 2, 2, 50, 2, 2, 2, 2, 2, 2, 2, -1, 2, -1, 2, -1, 2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 2, 51, 2, 2, 2, 2, 2, 2, 2, 2, -1, 2, -1, 2, -1, 2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 52, -1, 2, -1, 2, -1, 2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1];
    pub const ZZ_ATTR: [i32; 53] = [0, 1, 1, 1, 1, 1, 1, 0, 9, 0, 1, 0, 9, 9, 9, 9, 9, 9, 9, 0, 1, 1, 9, 9, 1, 1, 0, 1, 1, 1, 1, 0, 9, 9, 9, 9, 9, 9, 0, 1, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1];
    pub const ZZ_ACTION: [i32; 53] = [0, 1, 2, 3, 4, 5, 6, 0, 7, 0, 8, 0, 9, 10, 11, 12, 13, 14, 15, 0, 16, 17, 18, 19, 20, 21, 0, 22, 23, 24, 25, 0, 26, 27, 28, 29, 30, 31, 0, 32, 33, 34, 35, 0, 36, 0, 37, 38, 39, 40, 41, 42, 43];
    pub const ZZ_LEXSTATE: [i32; 2] = [0, 0];
    pub const YYINITIAL: usize = 0;


    pub const YYEOF: i32 = -1;

    pub fn new(input: &'a str) -> Lexer<'a> {
        let max_len = input.chars().clone().count();
        let mut cmap: Vec<usize> = Vec::with_capacity(256);
        cmap.resize(256, 0);
        let mut cmap2: HashMap<usize, usize> = HashMap::new();
        cmap[9] = 34;
        cmap[10] = 33;
        cmap[11] = 33;
        cmap[12] = 33;
        cmap[13] = 32;
        cmap[32] = 31;
        cmap[33] = 27;
        cmap[34] = 17;
        cmap[35] = 35;
        cmap[40] = 22;
        cmap[41] = 23;
        cmap[42] = 20;
        cmap[43] = 15;
        cmap[44] = 30;
        cmap[45] = 11;
        cmap[46] = 13;
        cmap[47] = 21;
        cmap[48] = 12;
        cmap[49] = 12;
        cmap[50] = 12;
        cmap[51] = 12;
        cmap[52] = 12;
        cmap[53] = 12;
        cmap[54] = 12;
        cmap[55] = 12;
        cmap[56] = 12;
        cmap[57] = 12;
        cmap[58] = 18;
        cmap[59] = 26;
        cmap[60] = 28;
        cmap[61] = 19;
        cmap[62] = 29;
        cmap[65] = 16;
        cmap[66] = 16;
        cmap[67] = 16;
        cmap[68] = 16;
        cmap[69] = 14;
        cmap[70] = 16;
        cmap[71] = 16;
        cmap[72] = 16;
        cmap[73] = 16;
        cmap[74] = 16;
        cmap[75] = 16;
        cmap[76] = 16;
        cmap[77] = 16;
        cmap[78] = 16;
        cmap[79] = 16;
        cmap[80] = 16;
        cmap[81] = 16;
        cmap[82] = 16;
        cmap[83] = 16;
        cmap[84] = 16;
        cmap[85] = 16;
        cmap[86] = 16;
        cmap[87] = 16;
        cmap[88] = 16;
        cmap[89] = 16;
        cmap[90] = 16;
        cmap[92] = 36;
        cmap[97] = 7;
        cmap[98] = 16;
        cmap[99] = 16;
        cmap[100] = 16;
        cmap[101] = 14;
        cmap[102] = 4;
        cmap[103] = 10;
        cmap[104] = 16;
        cmap[105] = 1;
        cmap[106] = 16;
        cmap[107] = 16;
        cmap[108] = 5;
        cmap[109] = 16;
        cmap[110] = 2;
        cmap[111] = 6;
        cmap[112] = 16;
        cmap[113] = 16;
        cmap[114] = 9;
        cmap[115] = 8;
        cmap[116] = 3;
        cmap[117] = 16;
        cmap[118] = 16;
        cmap[119] = 16;
        cmap[120] = 16;
        cmap[121] = 16;
        cmap[122] = 16;
        cmap[123] = 24;
        cmap[125] = 25;
        cmap[133] = 33;
        cmap2.insert(8232, 33);
        cmap2.insert(8233, 33);


        Lexer {
            input,
            cmap,

            cmap2,

            start: input.char_indices(),
            current: input.char_indices(),

            max_len,
            zz_state: 0,
            zz_lexical_state: Lexer::YYINITIAL,
            zz_marked_pos: 0,
            zz_current_pos: 0,
            zz_start_read: 0,
            zz_start_read_char: 0,
            zz_marked_char: 0,

            zz_at_eof: false,

        }
    }


    #[allow(dead_code)]
    pub fn is_eof(&self) -> bool {
        self.zz_at_eof
    }

    #[allow(dead_code)]
    pub fn yybegin(&mut self, new_state: usize) {
        self.zz_lexical_state = new_state;
    }

    #[allow(dead_code)]
    pub fn yystate(&self) -> usize {
        self.zz_lexical_state
    }

    #[allow(dead_code)]
    pub fn yylength(&self) -> usize {
        self.zz_marked_char - self.zz_start_read_char
    }

    #[allow(dead_code)]
    pub fn yycharat(&self, pos: usize) -> Option<char> {
        let mut ch: Option<char> = None;
        let mut start = self.start.clone();
        for _ in 0..(pos + 1) {
            if let Some(c) = start.next() {
                ch = Some(c.1);
            } else {
                return None;
            }
        }
        ch
    }

    #[allow(dead_code)]
    pub fn yytext(&self) -> String {
        self.input[self.yybytepos()].to_string()
    }

    #[allow(dead_code)]
    pub fn yytextpos(&self) -> std::ops::Range<usize> {
        std::ops::Range {
            start: self.zz_start_read_char,
            end: self.zz_marked_char,
        }
    }

    #[allow(dead_code)]
    pub fn yybytepos(&self) -> std::ops::Range<usize> {
        std::ops::Range {
            start: self.zz_start_read,
            end: self.zz_marked_pos,
        }
    }

    #[allow(dead_code)]
    pub fn yylex(&mut self) -> Result<TokenKind, Error> {
        let mut zz_input: i32 = -1;

        // cached
        loop {
            // char unit
            let mut zz_marked_char_l = self.zz_marked_char;
            let mut zz_current_char_pos_l = self.zz_marked_char;
            self.zz_start_read_char = self.zz_marked_char;

            // byte unit
            let mut zz_marked_byte_pos_l = self.zz_marked_pos;
            let mut zz_current_byte_pos_l = self.zz_marked_pos;

            let mut zz_action = -1;
            let mut current = self.current.clone();
            

            self.zz_start_read = self.zz_marked_pos;
            self.zz_current_pos = self.zz_marked_pos;
            self.zz_start_read_char = self.zz_marked_char;
            self.start = self.current.clone();

            self.zz_state = Lexer::ZZ_LEXSTATE[self.zz_lexical_state] as usize;

            // set up zz_action for empty match case:
            let zz_attributes = Lexer::ZZ_ATTR[self.zz_state];
            if (zz_attributes & 1) == 1 {
                zz_action = self.zz_state as i32;
            }

            'zz_for_action: loop {
                if zz_current_char_pos_l < self.max_len {
                    
                if let Some(next) = current.next() {
                    zz_current_byte_pos_l += next.1.len_utf8();
                    zz_input = next.1 as i32;
                }
                    zz_current_char_pos_l += 1;
                } else if self.zz_at_eof {
                    zz_input = Lexer::YYEOF;
                    break 'zz_for_action;
                } else {
                    self.zz_current_pos = zz_current_byte_pos_l;

                    if self.max_len <= zz_current_char_pos_l {
                        zz_input = Lexer::YYEOF;
                        break 'zz_for_action;
                    } else {
                        
                if let Some(next) = current.next() {
                    zz_current_byte_pos_l += next.1.len_utf8();
                    zz_input = next.1 as i32;
                }
                        zz_current_char_pos_l += 1;
                    }
                }

                let cidx = if zz_input <= 0xFF {
                    self.cmap[zz_input as usize]
                } else {

                    *self.cmap2.get(&(zz_input as usize)).unwrap_or(&0usize)

                };
                let idx = Lexer::ZZ_ROW[self.zz_state] + cidx;
                let zz_next = Lexer::ZZ_TRANS[idx];
                if zz_next == -1 {
                    break 'zz_for_action;
                }
                self.zz_state = zz_next as usize;

                let zz_attributes = Lexer::ZZ_ATTR[self.zz_state];
                if (zz_attributes & 1) == 1 {
                    zz_action = self.zz_state as i32;
                    zz_marked_char_l = zz_current_char_pos_l;
                    zz_marked_byte_pos_l = zz_current_byte_pos_l;
                    self.current = current.clone();

                    if (zz_attributes & 8) == 8 {
                        break 'zz_for_action;
                    }
                }
            }   // loop 'zz_for_action

            // store back cached position
            self.zz_marked_char = zz_marked_char_l;
            self.zz_marked_pos = zz_marked_byte_pos_l;

            if zz_input == Lexer::YYEOF && self.zz_start_read == self.zz_current_pos {
                self.zz_at_eof = true;

                return Err(Error::EOF);
            } else {
                let action = if zz_action < 0 {
                    zz_action
                } else {
                    Lexer::ZZ_ACTION[zz_action as usize]
                };
                match action {
                    1 => { return Ok(TokenKind::TokenId); }
                    44 => { /* nothing */ }
                    2 => { return Ok(TokenKind::TokenId); }
                    45 => { /* nothing */ }
                    3 => { return Ok(TokenKind::TokenId); }
                    46 => { /* nothing */ }
                    4 => { return Ok(TokenKind::TokenId); }
                    47 => { /* nothing */ }
                    5 => { return Ok(TokenKind::TokenSub); }
                    48 => { /* nothing */ }
                    6 => { return Ok(TokenKind::TokenIntLiteral); }
                    49 => { /* nothing */ }
                    7 => { return Ok(TokenKind::TokenSum); }
                    50 => { /* nothing */ }
                    8 => { return Ok(TokenKind::TokenColon); }
                    51 => { /* nothing */ }
                    9 => { return Ok(TokenKind::TokenMul); }
                    52 => { /* nothing */ }
                    10 => { return Ok(TokenKind::TokenDiv); }
                    53 => { /* nothing */ }
                    11 => { return Ok(TokenKind::TokenParOpen); }
                    54 => { /* nothing */ }
                    12 => { return Ok(TokenKind::TokenParClose); }
                    55 => { /* nothing */ }
                    13 => { return Ok(TokenKind::TokenCBOpen); }
                    56 => { /* nothing */ }
                    14 => { return Ok(TokenKind::TokenCBClose); }
                    57 => { /* nothing */ }
                    15 => { return Ok(TokenKind::TokenSemicolon); }
                    58 => { /* nothing */ }
                    16 => { return Ok(TokenKind::TokenLess); }
                    59 => { /* nothing */ }
                    17 => { return Ok(TokenKind::TokenGreater); }
                    60 => { /* nothing */ }
                    18 => { return Ok(TokenKind::TokenComa); }
                    61 => { /* nothing */ }
                    19 => {  }
                    62 => { /* nothing */ }
                    20 => {  }
                    63 => { /* nothing */ }
                    21 => {  }
                    64 => { /* nothing */ }
                    22 => { return Ok(TokenKind::TokenId); }
                    65 => { /* nothing */ }
                    23 => { return Ok(TokenKind::TokenId); }
                    66 => { /* nothing */ }
                    24 => { return Ok(TokenKind::TokenId); }
                    67 => { /* nothing */ }
                    25 => { return Ok(TokenKind::TokenFloatLiteral); }
                    68 => { /* nothing */ }
                    26 => { return Ok(TokenKind::TokenStringLiteral); }
                    69 => { /* nothing */ }
                    27 => { return Ok(TokenKind::TokenAssign); }
                    70 => { /* nothing */ }
                    28 => { return Ok(TokenKind::TokenEqual); }
                    71 => { /* nothing */ }
                    29 => { return Ok(TokenKind::TokenNotEqual); }
                    72 => { /* nothing */ }
                    30 => { return Ok(TokenKind::TokenLessEqual); }
                    73 => { /* nothing */ }
                    31 => { return Ok(TokenKind::TokenGreaterEqual); }
                    74 => { /* nothing */ }
                    32 => { return Ok(TokenKind::TokenId); }
                    75 => { /* nothing */ }
                    33 => { return Ok(TokenKind::TokenInt); }
                    76 => { /* nothing */ }
                    34 => { return Ok(TokenKind::TokenId); }
                    77 => { /* nothing */ }
                    35 => { return Ok(TokenKind::TokenId); }
                    78 => { /* nothing */ }
                    36 => { return Ok(TokenKind::TokenFloatLiteral); }
                    79 => { /* nothing */ }
                    37 => { return Ok(TokenKind::TokenInit); }
                    80 => { /* nothing */ }
                    38 => { return Ok(TokenKind::TokenId); }
                    81 => { /* nothing */ }
                    39 => { return Ok(TokenKind::TokenId); }
                    82 => { /* nothing */ }
                    40 => {  }
                    83 => { /* nothing */ }
                    41 => { return Ok(TokenKind::TokenFloat); }
                    84 => { /* nothing */ }
                    42 => { return Ok(TokenKind::TokenId); }
                    85 => { /* nothing */ }
                    43 => { return Ok(TokenKind::TokenString); }
                    86 => { /* nothing */ }

                    _ => {
                        return Err(Error::Unmatch);
                    }
                }
            }
        }   // loop
        // never reach end of function
    }

}
