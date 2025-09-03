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
    pub const ZZ_ROW: [usize; 79] = [0, 42, 84, 126, 168, 210, 252, 294, 336, 378, 420, 462, 504, 546, 588, 630, 672, 714, 588, 588, 588, 588, 588, 588, 588, 588, 756, 798, 840, 588, 882, 924, 966, 1008, 210, 1050, 1092, 1134, 1176, 210, 1218, 1260, 1302, 1344, 1386, 1428, 588, 588, 588, 588, 588, 588, 1470, 1512, 210, 210, 1554, 1596, 1638, 210, 1680, 1722, 1764, 1806, 1806, 1848, 210, 210, 1890, 1932, 1974, 2016, 210, 1470, 210, 210, 2058, 210, 210];
    pub const ZZ_TRANS: [i32; 2100] = [-1, 1, 2, 3, 4, 5, 6, 7, 8, 5, 5, 9, 5, 10, 5, 5, 11, 12, 13, 5, 14, 5, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 29, 32, -1, -1, 5, 33, 5, 34, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, -1, 5, -1, 5, -1, 5, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 5, 5, 5, 5, 5, 35, 5, 5, 5, 5, 5, 5, 5, 5, 5, -1, 5, -1, 5, -1, 5, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 5, 5, 5, 5, 5, 5, 5, 5, 36, 5, 5, 5, 5, 5, 5, -1, 5, -1, 5, -1, 5, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 5, 5, 5, 5, 37, 5, 38, 5, 5, 5, 5, 5, 5, 5, 5, -1, 5, -1, 5, -1, 5, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, -1, 5, -1, 5, -1, 5, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 5, 5, 5, 5, 5, 5, 5, 5, 39, 5, 5, 5, 5, 5, 5, -1, 5, -1, 5, -1, 5, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 5, 40, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, -1, 5, -1, 5, -1, 5, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 5, 5, 41, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, -1, 5, -1, 5, -1, 5, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 42, 5, 5, 5, -1, 5, -1, 5, -1, 5, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 5, 5, 5, 5, 43, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, -1, 5, -1, 5, -1, 5, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 12, 13, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 44, -1, -1, -1, 12, 45, 44, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 45, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 15, 15, -1, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 46, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 47, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 48, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 49, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 50, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 51, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 29, 29, 29, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 29, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 52, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 53, 5, 54, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, -1, 5, -1, 5, -1, 5, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 5, 5, 55, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, -1, 5, -1, 5, -1, 5, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 56, 5, -1, 5, -1, 5, -1, 5, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 5, 5, 5, 5, 5, 57, 5, 5, 5, 5, 5, 5, 5, 5, 5, -1, 5, -1, 5, -1, 5, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 5, 5, 5, 5, 58, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, -1, 5, -1, 5, -1, 5, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 59, -1, 5, -1, 5, -1, 5, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 5, 5, 5, 5, 5, 5, 5, 5, 60, 5, 5, 5, 5, 5, 5, -1, 5, -1, 5, -1, 5, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 61, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, -1, 5, -1, 5, -1, 5, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 5, 5, 5, 5, 5, 5, 5, 62, 5, 5, 5, 5, 5, 5, 5, -1, 5, -1, 5, -1, 5, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 63, 64, -1, -1, 63, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 44, -1, -1, -1, 45, -1, 44, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 52, 52, -1, 52, 52, 52, 52, 52, 52, 52, 52, 52, 52, 52, 52, 52, 52, 52, 52, 52, 65, 52, 52, 52, 52, 52, 52, 52, 52, 52, 52, 52, 52, 52, 52, 52, 52, 52, 52, 52, 52, -1, -1, 5, 5, 66, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, -1, 5, -1, 5, -1, 5, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 67, 5, 5, -1, 5, -1, 5, -1, 5, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 5, 5, 5, 5, 5, 5, 68, 5, 5, 5, 5, 5, 5, 5, 5, -1, 5, -1, 5, -1, 5, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 5, 5, 5, 5, 5, 5, 5, 69, 5, 5, 5, 5, 5, 5, 5, -1, 5, -1, 5, -1, 5, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 70, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, -1, 5, -1, 5, -1, 5, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 5, 5, 5, 5, 71, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, -1, 5, -1, 5, -1, 5, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 72, 5, 5, -1, 5, -1, 5, -1, 5, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 64, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 52, 52, -1, 52, 52, 52, 52, 52, 52, 52, 52, 52, 52, 52, 52, 52, 52, 52, 52, 52, 65, 52, 52, 52, 52, 52, 52, 52, 52, 52, 52, 52, 52, 52, 52, 52, 52, 52, 52, 52, 73, -1, -1, 5, 5, 74, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, -1, 5, -1, 5, -1, 5, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 75, 5, 5, -1, 5, -1, 5, -1, 5, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 5, 76, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, -1, 5, -1, 5, -1, 5, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 77, 5, 5, -1, 5, -1, 5, -1, 5, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 5, 5, 5, 5, 5, 5, 5, 5, 5, 78, 5, 5, 5, 5, 5, -1, 5, -1, 5, -1, 5, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1];
    pub const ZZ_ATTR: [i32; 79] = [0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 9, 0, 1, 0, 9, 9, 9, 9, 9, 9, 9, 9, 0, 1, 1, 9, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 9, 9, 9, 9, 9, 9, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
    pub const ZZ_ACTION: [i32; 79] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 0, 13, 0, 14, 0, 15, 16, 17, 18, 19, 20, 21, 22, 0, 23, 24, 25, 26, 27, 0, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 0, 39, 40, 41, 42, 43, 44, 45, 0, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 0, 56, 0, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69];
    pub const ZZ_LEXSTATE: [i32; 2] = [0, 0];
    pub const YYINITIAL: usize = 0;


    pub const YYEOF: i32 = -1;

    pub fn new(input: &'a str) -> Lexer<'a> {
        let max_len = input.chars().clone().count();
        let mut cmap: Vec<usize> = Vec::with_capacity(256);
        cmap.resize(256, 0);
        let mut cmap2: HashMap<usize, usize> = HashMap::new();
        cmap[9] = 39;
        cmap[10] = 38;
        cmap[11] = 38;
        cmap[12] = 38;
        cmap[13] = 37;
        cmap[32] = 36;
        cmap[33] = 33;
        cmap[34] = 22;
        cmap[35] = 40;
        cmap[40] = 27;
        cmap[41] = 28;
        cmap[42] = 25;
        cmap[43] = 20;
        cmap[44] = 32;
        cmap[45] = 16;
        cmap[46] = 18;
        cmap[47] = 26;
        cmap[48] = 17;
        cmap[49] = 17;
        cmap[50] = 17;
        cmap[51] = 17;
        cmap[52] = 17;
        cmap[53] = 17;
        cmap[54] = 17;
        cmap[55] = 17;
        cmap[56] = 17;
        cmap[57] = 17;
        cmap[58] = 23;
        cmap[59] = 31;
        cmap[60] = 34;
        cmap[61] = 24;
        cmap[62] = 35;
        cmap[65] = 21;
        cmap[66] = 21;
        cmap[67] = 21;
        cmap[68] = 21;
        cmap[69] = 19;
        cmap[70] = 21;
        cmap[71] = 21;
        cmap[72] = 21;
        cmap[73] = 21;
        cmap[74] = 21;
        cmap[75] = 21;
        cmap[76] = 21;
        cmap[77] = 21;
        cmap[78] = 21;
        cmap[79] = 21;
        cmap[80] = 21;
        cmap[81] = 21;
        cmap[82] = 21;
        cmap[83] = 21;
        cmap[84] = 21;
        cmap[85] = 21;
        cmap[86] = 21;
        cmap[87] = 21;
        cmap[88] = 21;
        cmap[89] = 21;
        cmap[90] = 21;
        cmap[92] = 41;
        cmap[97] = 7;
        cmap[98] = 21;
        cmap[99] = 21;
        cmap[100] = 15;
        cmap[101] = 13;
        cmap[102] = 4;
        cmap[103] = 10;
        cmap[104] = 12;
        cmap[105] = 1;
        cmap[106] = 21;
        cmap[107] = 21;
        cmap[108] = 5;
        cmap[109] = 21;
        cmap[110] = 2;
        cmap[111] = 6;
        cmap[112] = 21;
        cmap[113] = 21;
        cmap[114] = 9;
        cmap[115] = 8;
        cmap[116] = 3;
        cmap[117] = 14;
        cmap[118] = 21;
        cmap[119] = 11;
        cmap[120] = 21;
        cmap[121] = 21;
        cmap[122] = 21;
        cmap[123] = 29;
        cmap[125] = 30;
        cmap[133] = 38;
        cmap2.insert(8232, 38);
        cmap2.insert(8233, 38);


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
                    70 => { /* nothing */ }
                    2 => { return Ok(TokenKind::TokenId); }
                    71 => { /* nothing */ }
                    3 => { return Ok(TokenKind::TokenId); }
                    72 => { /* nothing */ }
                    4 => { return Ok(TokenKind::TokenId); }
                    73 => { /* nothing */ }
                    5 => { return Ok(TokenKind::TokenId); }
                    74 => { /* nothing */ }
                    6 => { return Ok(TokenKind::TokenId); }
                    75 => { /* nothing */ }
                    7 => { return Ok(TokenKind::TokenId); }
                    76 => { /* nothing */ }
                    8 => { return Ok(TokenKind::TokenId); }
                    77 => { /* nothing */ }
                    9 => { return Ok(TokenKind::TokenId); }
                    78 => { /* nothing */ }
                    10 => { return Ok(TokenKind::TokenId); }
                    79 => { /* nothing */ }
                    11 => { return Ok(TokenKind::TokenSub); }
                    80 => { /* nothing */ }
                    12 => { return Ok(TokenKind::TokenIntLiteral); }
                    81 => { /* nothing */ }
                    13 => { return Ok(TokenKind::TokenSum); }
                    82 => { /* nothing */ }
                    14 => { return Ok(TokenKind::TokenColon); }
                    83 => { /* nothing */ }
                    15 => { return Ok(TokenKind::TokenMul); }
                    84 => { /* nothing */ }
                    16 => { return Ok(TokenKind::TokenDiv); }
                    85 => { /* nothing */ }
                    17 => { return Ok(TokenKind::TokenParOpen); }
                    86 => { /* nothing */ }
                    18 => { return Ok(TokenKind::TokenParClose); }
                    87 => { /* nothing */ }
                    19 => { return Ok(TokenKind::TokenCBOpen); }
                    88 => { /* nothing */ }
                    20 => { return Ok(TokenKind::TokenCBClose); }
                    89 => { /* nothing */ }
                    21 => { return Ok(TokenKind::TokenSemicolon); }
                    90 => { /* nothing */ }
                    22 => { return Ok(TokenKind::TokenComma); }
                    91 => { /* nothing */ }
                    23 => { return Ok(TokenKind::TokenLess); }
                    92 => { /* nothing */ }
                    24 => { return Ok(TokenKind::TokenGreater); }
                    93 => { /* nothing */ }
                    25 => {  }
                    94 => { /* nothing */ }
                    26 => {  }
                    95 => { /* nothing */ }
                    27 => {  }
                    96 => { /* nothing */ }
                    28 => { return Ok(TokenKind::TokenId); }
                    97 => { /* nothing */ }
                    29 => { return Ok(TokenKind::TokenIf); }
                    98 => { /* nothing */ }
                    30 => { return Ok(TokenKind::TokenId); }
                    99 => { /* nothing */ }
                    31 => { return Ok(TokenKind::TokenId); }
                    100 => { /* nothing */ }
                    32 => { return Ok(TokenKind::TokenId); }
                    101 => { /* nothing */ }
                    33 => { return Ok(TokenKind::TokenId); }
                    102 => { /* nothing */ }
                    34 => { return Ok(TokenKind::TokenOr); }
                    103 => { /* nothing */ }
                    35 => { return Ok(TokenKind::TokenId); }
                    104 => { /* nothing */ }
                    36 => { return Ok(TokenKind::TokenId); }
                    105 => { /* nothing */ }
                    37 => { return Ok(TokenKind::TokenId); }
                    106 => { /* nothing */ }
                    38 => { return Ok(TokenKind::TokenId); }
                    107 => { /* nothing */ }
                    39 => { return Ok(TokenKind::TokenFloatLiteral); }
                    108 => { /* nothing */ }
                    40 => { return Ok(TokenKind::TokenStringLiteral); }
                    109 => { /* nothing */ }
                    41 => { return Ok(TokenKind::TokenAssign); }
                    110 => { /* nothing */ }
                    42 => { return Ok(TokenKind::TokenEqual); }
                    111 => { /* nothing */ }
                    43 => { return Ok(TokenKind::TokenNotEqual); }
                    112 => { /* nothing */ }
                    44 => { return Ok(TokenKind::TokenLessEqual); }
                    113 => { /* nothing */ }
                    45 => { return Ok(TokenKind::TokenGreaterEqual); }
                    114 => { /* nothing */ }
                    46 => { return Ok(TokenKind::TokenId); }
                    115 => { /* nothing */ }
                    47 => { return Ok(TokenKind::TokenInt); }
                    116 => { /* nothing */ }
                    48 => { return Ok(TokenKind::TokenNot); }
                    117 => { /* nothing */ }
                    49 => { return Ok(TokenKind::TokenId); }
                    118 => { /* nothing */ }
                    50 => { return Ok(TokenKind::TokenId); }
                    119 => { /* nothing */ }
                    51 => { return Ok(TokenKind::TokenId); }
                    120 => { /* nothing */ }
                    52 => { return Ok(TokenKind::TokenAnd); }
                    121 => { /* nothing */ }
                    53 => { return Ok(TokenKind::TokenId); }
                    122 => { /* nothing */ }
                    54 => { return Ok(TokenKind::TokenId); }
                    123 => { /* nothing */ }
                    55 => { return Ok(TokenKind::TokenId); }
                    124 => { /* nothing */ }
                    56 => { return Ok(TokenKind::TokenFloatLiteral); }
                    125 => { /* nothing */ }
                    57 => { return Ok(TokenKind::TokenInit); }
                    126 => { /* nothing */ }
                    58 => { return Ok(TokenKind::TokenTrue); }
                    127 => { /* nothing */ }
                    59 => { return Ok(TokenKind::TokenId); }
                    128 => { /* nothing */ }
                    60 => { return Ok(TokenKind::TokenId); }
                    129 => { /* nothing */ }
                    61 => { return Ok(TokenKind::TokenId); }
                    130 => { /* nothing */ }
                    62 => { return Ok(TokenKind::TokenId); }
                    131 => { /* nothing */ }
                    63 => { return Ok(TokenKind::TokenElse); }
                    132 => { /* nothing */ }
                    64 => {  }
                    133 => { /* nothing */ }
                    65 => { return Ok(TokenKind::TokenFloat); }
                    134 => { /* nothing */ }
                    66 => { return Ok(TokenKind::TokenFalse); }
                    135 => { /* nothing */ }
                    67 => { return Ok(TokenKind::TokenId); }
                    136 => { /* nothing */ }
                    68 => { return Ok(TokenKind::TokenWhile); }
                    137 => { /* nothing */ }
                    69 => { return Ok(TokenKind::TokenString); }
                    138 => { /* nothing */ }

                    _ => {
                        return Err(Error::Unmatch);
                    }
                }
            }
        }   // loop
        // never reach end of function
    }

}
