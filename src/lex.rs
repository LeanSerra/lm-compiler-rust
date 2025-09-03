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
    pub const ZZ_ROW: [usize; 89] = [0, 41, 82, 123, 164, 205, 246, 287, 328, 369, 410, 451, 492, 533, 574, 615, 656, 697, 738, 779, 820, 861, 82, 82, 82, 82, 82, 82, 82, 82, 902, 943, 984, 82, 1025, 1066, 369, 1107, 1148, 1189, 1230, 369, 1271, 1312, 1353, 1394, 1435, 1476, 1517, 1558, 82, 82, 82, 82, 82, 82, 1599, 1640, 369, 369, 1681, 1722, 1763, 369, 1804, 1845, 1886, 1927, 1968, 2009, 2009, 2050, 2091, 369, 369, 2132, 2173, 2214, 369, 2255, 2296, 369, 1599, 369, 369, 2337, 369, 369, 369];
    pub const ZZ_TRANS: [i32; 2378] = [-1, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 9, 14, 9, 15, 9, 9, 16, 17, 18, 9, 9, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 33, -1, -1, 34, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 33, -1, -1, -1, 33, 33, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 33, -1, -1, -1, -1, -1, 9, 35, 9, 36, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, 9, -1, 9, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 37, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, 9, -1, 9, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 9, 9, 9, 38, 9, 9, 9, 9, 9, 9, -1, 9, -1, 9, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 39, 9, 40, 9, 9, 9, 9, 9, 9, 9, 9, -1, 9, -1, 9, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, 9, -1, 9, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 9, 9, 9, 41, 9, 9, 9, 9, 9, 9, -1, 9, -1, 9, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 42, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, 9, -1, 9, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 43, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, 9, -1, 9, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 44, 9, 9, -1, 9, -1, 9, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 9, 9, 9, 45, 9, 9, 46, 9, 9, 9, -1, 9, -1, 9, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 47, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, 9, -1, 9, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 17, 18, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 48, -1, -1, -1, 17, 49, 48, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 49, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 19, 19, 19, 19, 19, 19, -1, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 50, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 51, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 52, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 53, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 54, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 55, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 56, 56, 56, -1, -1, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, -1, -1, -1, -1, -1, 57, 9, 58, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, 9, -1, 9, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 59, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, 9, -1, 9, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 60, 9, -1, 9, -1, 9, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 61, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, 9, -1, 9, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 62, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, 9, -1, 9, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 63, -1, 9, -1, 9, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 9, 9, 9, 64, 9, 9, 9, 9, 9, 9, -1, 9, -1, 9, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 9, 65, 9, 9, 9, 9, 9, 9, 9, 9, -1, 9, -1, 9, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 66, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, 9, -1, 9, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 67, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, 9, -1, 9, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 9, 9, 68, 9, 9, 9, 9, 9, 9, 9, -1, 9, -1, 9, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 69, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 69, 70, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 48, -1, -1, -1, 49, -1, 48, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 56, 56, 71, 72, 72, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, -1, -1, -1, -1, -1, 9, 9, 73, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, 9, -1, 9, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 74, 9, 9, -1, 9, -1, 9, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 9, 75, 9, 9, 9, 9, 9, 9, 9, 9, -1, 9, -1, 9, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 9, 9, 76, 9, 9, 9, 9, 9, 9, 9, -1, 9, -1, 9, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 77, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, 9, -1, 9, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 78, -1, 9, -1, 9, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 79, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, 9, -1, 9, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 80, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, 9, -1, 9, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 81, 9, 9, -1, 9, -1, 9, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 70, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 56, 82, 71, 72, 72, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 71, -1, -1, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, 56, -1, -1, -1, -1, -1, 9, 9, 83, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, 9, -1, 9, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 84, 9, 9, -1, 9, -1, 9, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 85, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, 9, -1, 9, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 86, 9, 9, -1, 9, -1, 9, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 87, 9, 9, -1, 9, -1, 9, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 9, 9, 9, 9, 88, 9, 9, 9, 9, 9, -1, 9, -1, 9, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1];
    pub const ZZ_ATTR: [i32; 89] = [0, 0, 9, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 0, 9, 9, 9, 9, 9, 9, 9, 9, 0, 1, 1, 9, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 9, 9, 9, 9, 9, 9, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
    pub const ZZ_ACTION: [i32; 89] = [0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 0, 0, 17, 0, 18, 19, 20, 21, 22, 23, 24, 25, 0, 26, 27, 28, 0, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 0, 42, 43, 44, 45, 46, 47, 48, 0, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 0, 61, 0, 0, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77];
    pub const ZZ_LEXSTATE: [i32; 2] = [0, 0];
    pub const YYINITIAL: usize = 0;


    pub const YYEOF: i32 = -1;

    pub fn new(input: &'a str) -> Lexer<'a> {
        let max_len = input.chars().clone().count();
        let mut cmap: Vec<usize> = Vec::with_capacity(256);
        cmap.resize(256, 0);
        let mut cmap2: HashMap<usize, usize> = HashMap::new();
        cmap[9] = 40;
        cmap[10] = 3;
        cmap[11] = 3;
        cmap[12] = 3;
        cmap[13] = 4;
        cmap[32] = 39;
        cmap[33] = 36;
        cmap[34] = 25;
        cmap[35] = 1;
        cmap[40] = 30;
        cmap[41] = 31;
        cmap[42] = 28;
        cmap[43] = 2;
        cmap[44] = 35;
        cmap[45] = 20;
        cmap[46] = 22;
        cmap[47] = 29;
        cmap[48] = 21;
        cmap[49] = 21;
        cmap[50] = 21;
        cmap[51] = 21;
        cmap[52] = 21;
        cmap[53] = 21;
        cmap[54] = 21;
        cmap[55] = 21;
        cmap[56] = 21;
        cmap[57] = 21;
        cmap[58] = 26;
        cmap[59] = 34;
        cmap[60] = 37;
        cmap[61] = 27;
        cmap[62] = 38;
        cmap[65] = 24;
        cmap[66] = 24;
        cmap[67] = 24;
        cmap[68] = 24;
        cmap[69] = 23;
        cmap[70] = 24;
        cmap[71] = 24;
        cmap[72] = 24;
        cmap[73] = 24;
        cmap[74] = 24;
        cmap[75] = 24;
        cmap[76] = 24;
        cmap[77] = 24;
        cmap[78] = 24;
        cmap[79] = 24;
        cmap[80] = 24;
        cmap[81] = 24;
        cmap[82] = 24;
        cmap[83] = 24;
        cmap[84] = 24;
        cmap[85] = 24;
        cmap[86] = 24;
        cmap[87] = 24;
        cmap[88] = 24;
        cmap[89] = 24;
        cmap[90] = 24;
        cmap[92] = 25;
        cmap[97] = 11;
        cmap[98] = 24;
        cmap[99] = 24;
        cmap[100] = 19;
        cmap[101] = 17;
        cmap[102] = 8;
        cmap[103] = 14;
        cmap[104] = 16;
        cmap[105] = 5;
        cmap[106] = 24;
        cmap[107] = 24;
        cmap[108] = 9;
        cmap[109] = 24;
        cmap[110] = 6;
        cmap[111] = 10;
        cmap[112] = 24;
        cmap[113] = 24;
        cmap[114] = 13;
        cmap[115] = 12;
        cmap[116] = 7;
        cmap[117] = 18;
        cmap[118] = 24;
        cmap[119] = 15;
        cmap[120] = 24;
        cmap[121] = 24;
        cmap[122] = 24;
        cmap[123] = 32;
        cmap[125] = 33;
        cmap[133] = 3;
        cmap2.insert(8232, 3);
        cmap2.insert(8233, 3);


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
                    1 => { return Ok(TokenKind::TokenSum); }
                    78 => { /* nothing */ }
                    2 => {  }
                    79 => { /* nothing */ }
                    3 => {  }
                    80 => { /* nothing */ }
                    4 => { return Ok(TokenKind::TokenId); }
                    81 => { /* nothing */ }
                    5 => { return Ok(TokenKind::TokenId); }
                    82 => { /* nothing */ }
                    6 => { return Ok(TokenKind::TokenId); }
                    83 => { /* nothing */ }
                    7 => { return Ok(TokenKind::TokenId); }
                    84 => { /* nothing */ }
                    8 => { return Ok(TokenKind::TokenId); }
                    85 => { /* nothing */ }
                    9 => { return Ok(TokenKind::TokenId); }
                    86 => { /* nothing */ }
                    10 => { return Ok(TokenKind::TokenId); }
                    87 => { /* nothing */ }
                    11 => { return Ok(TokenKind::TokenId); }
                    88 => { /* nothing */ }
                    12 => { return Ok(TokenKind::TokenId); }
                    89 => { /* nothing */ }
                    13 => { return Ok(TokenKind::TokenId); }
                    90 => { /* nothing */ }
                    14 => { return Ok(TokenKind::TokenId); }
                    91 => { /* nothing */ }
                    15 => { return Ok(TokenKind::TokenSub); }
                    92 => { /* nothing */ }
                    16 => { return Ok(TokenKind::TokenIntLiteral); }
                    93 => { /* nothing */ }
                    17 => { return Ok(TokenKind::TokenColon); }
                    94 => { /* nothing */ }
                    18 => { return Ok(TokenKind::TokenMul); }
                    95 => { /* nothing */ }
                    19 => { return Ok(TokenKind::TokenDiv); }
                    96 => { /* nothing */ }
                    20 => { return Ok(TokenKind::TokenParOpen); }
                    97 => { /* nothing */ }
                    21 => { return Ok(TokenKind::TokenParClose); }
                    98 => { /* nothing */ }
                    22 => { return Ok(TokenKind::TokenCBOpen); }
                    99 => { /* nothing */ }
                    23 => { return Ok(TokenKind::TokenCBClose); }
                    100 => { /* nothing */ }
                    24 => { return Ok(TokenKind::TokenSemicolon); }
                    101 => { /* nothing */ }
                    25 => { return Ok(TokenKind::TokenComma); }
                    102 => { /* nothing */ }
                    26 => { return Ok(TokenKind::TokenLess); }
                    103 => { /* nothing */ }
                    27 => { return Ok(TokenKind::TokenGreater); }
                    104 => { /* nothing */ }
                    28 => {  }
                    105 => { /* nothing */ }
                    29 => { return Ok(TokenKind::TokenId); }
                    106 => { /* nothing */ }
                    30 => { return Ok(TokenKind::TokenIf); }
                    107 => { /* nothing */ }
                    31 => { return Ok(TokenKind::TokenId); }
                    108 => { /* nothing */ }
                    32 => { return Ok(TokenKind::TokenId); }
                    109 => { /* nothing */ }
                    33 => { return Ok(TokenKind::TokenId); }
                    110 => { /* nothing */ }
                    34 => { return Ok(TokenKind::TokenId); }
                    111 => { /* nothing */ }
                    35 => { return Ok(TokenKind::TokenOr); }
                    112 => { /* nothing */ }
                    36 => { return Ok(TokenKind::TokenId); }
                    113 => { /* nothing */ }
                    37 => { return Ok(TokenKind::TokenId); }
                    114 => { /* nothing */ }
                    38 => { return Ok(TokenKind::TokenId); }
                    115 => { /* nothing */ }
                    39 => { return Ok(TokenKind::TokenId); }
                    116 => { /* nothing */ }
                    40 => { return Ok(TokenKind::TokenId); }
                    117 => { /* nothing */ }
                    41 => { return Ok(TokenKind::TokenId); }
                    118 => { /* nothing */ }
                    42 => { return Ok(TokenKind::TokenFloatLiteral); }
                    119 => { /* nothing */ }
                    43 => { return Ok(TokenKind::TokenStringLiteral); }
                    120 => { /* nothing */ }
                    44 => { return Ok(TokenKind::TokenAssign); }
                    121 => { /* nothing */ }
                    45 => { return Ok(TokenKind::TokenEqual); }
                    122 => { /* nothing */ }
                    46 => { return Ok(TokenKind::TokenNotEqual); }
                    123 => { /* nothing */ }
                    47 => { return Ok(TokenKind::TokenLessEqual); }
                    124 => { /* nothing */ }
                    48 => { return Ok(TokenKind::TokenGreaterEqual); }
                    125 => { /* nothing */ }
                    49 => { return Ok(TokenKind::TokenId); }
                    126 => { /* nothing */ }
                    50 => { return Ok(TokenKind::TokenInt); }
                    127 => { /* nothing */ }
                    51 => { return Ok(TokenKind::TokenNot); }
                    128 => { /* nothing */ }
                    52 => { return Ok(TokenKind::TokenId); }
                    129 => { /* nothing */ }
                    53 => { return Ok(TokenKind::TokenId); }
                    130 => { /* nothing */ }
                    54 => { return Ok(TokenKind::TokenId); }
                    131 => { /* nothing */ }
                    55 => { return Ok(TokenKind::TokenAnd); }
                    132 => { /* nothing */ }
                    56 => { return Ok(TokenKind::TokenId); }
                    133 => { /* nothing */ }
                    57 => { return Ok(TokenKind::TokenId); }
                    134 => { /* nothing */ }
                    58 => { return Ok(TokenKind::TokenId); }
                    135 => { /* nothing */ }
                    59 => { return Ok(TokenKind::TokenId); }
                    136 => { /* nothing */ }
                    60 => { return Ok(TokenKind::TokenId); }
                    137 => { /* nothing */ }
                    61 => { return Ok(TokenKind::TokenFloatLiteral); }
                    138 => { /* nothing */ }
                    62 => { return Ok(TokenKind::TokenInit); }
                    139 => { /* nothing */ }
                    63 => { return Ok(TokenKind::TokenTrue); }
                    140 => { /* nothing */ }
                    64 => { return Ok(TokenKind::TokenId); }
                    141 => { /* nothing */ }
                    65 => { return Ok(TokenKind::TokenId); }
                    142 => { /* nothing */ }
                    66 => { return Ok(TokenKind::TokenId); }
                    143 => { /* nothing */ }
                    67 => { return Ok(TokenKind::TokenRead); }
                    144 => { /* nothing */ }
                    68 => { return Ok(TokenKind::TokenId); }
                    145 => { /* nothing */ }
                    69 => { return Ok(TokenKind::TokenId); }
                    146 => { /* nothing */ }
                    70 => { return Ok(TokenKind::TokenElse); }
                    147 => { /* nothing */ }
                    71 => {  }
                    148 => { /* nothing */ }
                    72 => { return Ok(TokenKind::TokenFloat); }
                    149 => { /* nothing */ }
                    73 => { return Ok(TokenKind::TokenFalse); }
                    150 => { /* nothing */ }
                    74 => { return Ok(TokenKind::TokenId); }
                    151 => { /* nothing */ }
                    75 => { return Ok(TokenKind::TokenWrite); }
                    152 => { /* nothing */ }
                    76 => { return Ok(TokenKind::TokenWhile); }
                    153 => { /* nothing */ }
                    77 => { return Ok(TokenKind::TokenString); }
                    154 => { /* nothing */ }

                    _ => {
                        return Err(Error::Unmatch);
                    }
                }
            }
        }   // loop
        // never reach end of function
    }

}
