use crate::grammar::TokenKind;
use crate::compiler::error::{CompilerError, log_error_and_exit};
use crate::compiler::context::CompilerContext;

type CompilerCtx<'a> = &'a mut CompilerContext;


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

    offset: usize,
    ctx: CompilerCtx<'a>,
}

impl<'a> Lexer<'a> {
    pub const ZZ_ROW: [usize; 110] = [0, 44, 88, 132, 176, 220, 264, 308, 352, 396, 440, 484, 528, 572, 616, 660, 704, 748, 792, 836, 792, 880, 924, 792, 792, 792, 792, 792, 792, 792, 968, 1012, 1056, 792, 44, 1100, 1144, 1188, 396, 1232, 1276, 1320, 1364, 1408, 396, 1452, 1496, 1540, 1584, 1628, 1672, 1716, 1760, 1804, 1848, 792, 792, 792, 792, 792, 1892, 1936, 1980, 396, 2024, 396, 2068, 2112, 2156, 396, 2200, 2244, 2288, 2332, 2376, 2420, 2464, 2464, 2508, 2552, 396, 2596, 396, 2640, 2684, 2728, 396, 2772, 2816, 396, 2860, 2904, 2948, 396, 396, 2992, 396, 396, 3036, 3080, 396, 396, 3124, 3168, 3212, 3256, 396, 3300, 3344, 792];
    pub const ZZ_TRANS: [i32; 3388] = [1, 2, -1, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 9, 14, 9, 15, 9, 9, 9, 16, 9, 9, 17, 18, 19, 9, 20, 9, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 33, 34, -1, 35, 1, -1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 36, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 33, 33, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 33, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 33, -1, -1, -1, -1, -1, 9, 37, 9, 38, 9, 9, 9, 39, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 40, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 9, 9, 9, 41, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 42, 9, 43, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 9, 9, 9, 44, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 45, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 46, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 47, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 9, 9, 9, 48, 9, 9, 49, 9, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 50, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 51, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 52, -1, -1, -1, -1, -1, -1, 53, -1, 54, 52, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 54, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 55, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 56, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 57, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 58, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 59, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 1, 1, 1, -1, -1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 36, -1, 60, 36, -1, 36, 36, 36, 36, 36, 36, 36, 36, 36, 36, 36, 36, 36, 36, 36, 36, 36, 36, 36, 36, 36, 36, 36, 61, 36, 36, 36, 36, 36, 36, 36, 36, 36, 36, 36, 36, 36, 36, 36, -1, -1, -1, -1, -1, 62, 9, 63, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 64, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 65, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 66, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 67, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 68, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 69, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 9, 9, 9, 70, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 9, 71, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 72, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 73, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 9, 9, 74, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 75, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 76, 77, -1, -1, 77, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 52, -1, -1, -1, -1, -1, -1, 78, 79, 54, 52, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 52, -1, -1, -1, -1, -1, -1, 54, -1, -1, 52, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 36, 36, 36, -1, -1, 36, 36, 36, 36, 36, 36, 36, 36, 36, 36, 36, 36, 36, 36, 36, 36, 36, 36, 36, 36, 36, 36, 36, 36, 36, 36, 36, 36, 36, 36, 36, 36, 36, 36, 36, 36, 36, 36, 36, 36, 33, 60, 36, -1, 36, 36, 36, 36, 36, 36, 36, 36, 36, 36, 36, 36, 36, 36, 36, 36, 36, 36, 36, 36, 36, 36, 36, 61, 36, 36, 36, 36, 36, 36, 36, 36, 36, 36, 36, 36, 36, 36, 36, -1, -1, -1, -1, -1, 9, 9, 80, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 81, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 82, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 9, 83, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 9, 9, 84, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 85, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 86, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 87, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 88, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 89, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 90, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 76, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 52, -1, -1, -1, -1, -1, -1, 78, -1, 54, 52, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 91, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 9, 9, 9, 92, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 93, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 94, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 95, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 96, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 97, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 98, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 99, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 100, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 9, 9, 9, 9, 101, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 9, 102, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 103, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 104, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 105, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 106, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 107, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 108, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 109, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1];
    pub const ZZ_ATTR: [i32; 110] = [0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9, 0, 9, 1, 0, 9, 9, 9, 9, 9, 9, 9, 0, 1, 1, 9, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 9, 9, 9, 9, 9, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 0, 1, 0, 0, 9];
    pub const ZZ_ACTION: [i32; 110] = [0, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 0, 17, 18, 0, 19, 20, 21, 22, 23, 24, 25, 0, 26, 27, 28, 29, 0, 0, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 0, 45, 46, 47, 48, 49, 50, 51, 0, 0, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 0, 67, 0, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 0, 79, 80, 81, 82, 83, 84, 85, 0, 86, 87, 88, 0, 89, 0, 90, 0, 0, 91];
    pub const ZZ_LEXSTATE: [i32; 2] = [0, 0];
    pub const YYINITIAL: usize = 0;


    pub const YYEOF: i32 = -1;

    pub fn new(input: &'a str, offset: usize, ctx: CompilerCtx<'a>) -> Lexer<'a> {
        let max_len = input.chars().clone().count();
        let mut cmap: Vec<usize> = Vec::with_capacity(256);
        cmap.resize(256, 0);
        let mut cmap2: HashMap<usize, usize> = HashMap::new();
        cmap[9] = 43;
        cmap[10] = 4;
        cmap[11] = 4;
        cmap[12] = 4;
        cmap[13] = 3;
        cmap[32] = 42;
        cmap[33] = 39;
        cmap[35] = 1;
        cmap[40] = 34;
        cmap[41] = 35;
        cmap[42] = 32;
        cmap[43] = 28;
        cmap[44] = 38;
        cmap[45] = 25;
        cmap[46] = 26;
        cmap[47] = 33;
        cmap[48] = 24;
        cmap[49] = 24;
        cmap[50] = 24;
        cmap[51] = 24;
        cmap[52] = 24;
        cmap[53] = 24;
        cmap[54] = 24;
        cmap[55] = 24;
        cmap[56] = 24;
        cmap[57] = 24;
        cmap[58] = 30;
        cmap[60] = 40;
        cmap[61] = 31;
        cmap[62] = 41;
        cmap[65] = 29;
        cmap[66] = 29;
        cmap[67] = 29;
        cmap[68] = 23;
        cmap[69] = 27;
        cmap[70] = 29;
        cmap[71] = 29;
        cmap[72] = 29;
        cmap[73] = 29;
        cmap[74] = 29;
        cmap[75] = 29;
        cmap[76] = 29;
        cmap[77] = 29;
        cmap[78] = 29;
        cmap[79] = 29;
        cmap[80] = 29;
        cmap[81] = 29;
        cmap[82] = 29;
        cmap[83] = 29;
        cmap[84] = 29;
        cmap[85] = 29;
        cmap[86] = 29;
        cmap[87] = 29;
        cmap[88] = 29;
        cmap[89] = 29;
        cmap[90] = 20;
        cmap[92] = 2;
        cmap[97] = 11;
        cmap[98] = 29;
        cmap[99] = 21;
        cmap[100] = 19;
        cmap[101] = 17;
        cmap[102] = 8;
        cmap[103] = 14;
        cmap[104] = 16;
        cmap[105] = 5;
        cmap[106] = 29;
        cmap[107] = 29;
        cmap[108] = 9;
        cmap[109] = 29;
        cmap[110] = 6;
        cmap[111] = 10;
        cmap[112] = 29;
        cmap[113] = 29;
        cmap[114] = 13;
        cmap[115] = 12;
        cmap[116] = 7;
        cmap[117] = 18;
        cmap[118] = 22;
        cmap[119] = 15;
        cmap[120] = 29;
        cmap[121] = 29;
        cmap[122] = 29;
        cmap[123] = 36;
        cmap[125] = 37;
        cmap[133] = 4;
        cmap2.insert(8232, 4);
        cmap2.insert(8233, 4);


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

            offset,
            ctx,
        }
    }


    #[allow(dead_code)]
    pub fn get_offset(&mut self) -> &mut usize { &mut self.offset }

    #[allow(dead_code)]
    pub fn get_ctx(&mut self) -> &mut CompilerCtx<'a> { &mut self.ctx }

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
                    1 => {  }
                    92 => { /* nothing */ }
                    2 => {  }
                    93 => { /* nothing */ }
                    3 => { return Ok(TokenKind::TokenId); }
                    94 => { /* nothing */ }
                    4 => { return Ok(TokenKind::TokenId); }
                    95 => { /* nothing */ }
                    5 => { return Ok(TokenKind::TokenId); }
                    96 => { /* nothing */ }
                    6 => { return Ok(TokenKind::TokenId); }
                    97 => { /* nothing */ }
                    7 => { return Ok(TokenKind::TokenId); }
                    98 => { /* nothing */ }
                    8 => { return Ok(TokenKind::TokenId); }
                    99 => { /* nothing */ }
                    9 => { return Ok(TokenKind::TokenId); }
                    100 => { /* nothing */ }
                    10 => { return Ok(TokenKind::TokenId); }
                    101 => { /* nothing */ }
                    11 => { return Ok(TokenKind::TokenId); }
                    102 => { /* nothing */ }
                    12 => { return Ok(TokenKind::TokenId); }
                    103 => { /* nothing */ }
                    13 => { return Ok(TokenKind::TokenId); }
                    104 => { /* nothing */ }
                    14 => { return Ok(TokenKind::TokenId); }
                    105 => { /* nothing */ }
                    15 => { {
                                                                    if let Err(e) = self.yytext().parse::<i64>() {
                                                                        log_error_and_exit(
                                                                            self.yytextpos(),
                                                                            CompilerError::Lexer(format!("Invalid integer literal {e}")),
                                                                            self.offset,
                                                                            true,
                                                                            self.ctx
                                                                        )
                                                                    }
                                                                    return Ok(TokenKind::TokenIntLiteral);
                                                                } }
                    106 => { /* nothing */ }
                    16 => { return Ok(TokenKind::TokenSub); }
                    107 => { /* nothing */ }
                    17 => { return Ok(TokenKind::TokenSum); }
                    108 => { /* nothing */ }
                    18 => { return Ok(TokenKind::TokenColon); }
                    109 => { /* nothing */ }
                    19 => { return Ok(TokenKind::TokenMul); }
                    110 => { /* nothing */ }
                    20 => { return Ok(TokenKind::TokenDiv); }
                    111 => { /* nothing */ }
                    21 => { return Ok(TokenKind::TokenParOpen); }
                    112 => { /* nothing */ }
                    22 => { return Ok(TokenKind::TokenParClose); }
                    113 => { /* nothing */ }
                    23 => { return Ok(TokenKind::TokenCBOpen); }
                    114 => { /* nothing */ }
                    24 => { return Ok(TokenKind::TokenCBClose); }
                    115 => { /* nothing */ }
                    25 => { return Ok(TokenKind::TokenComma); }
                    116 => { /* nothing */ }
                    26 => { return Ok(TokenKind::TokenLess); }
                    117 => { /* nothing */ }
                    27 => { return Ok(TokenKind::TokenGreater); }
                    118 => { /* nothing */ }
                    28 => {  }
                    119 => { /* nothing */ }
                    29 => { {
                                                                    if self.yytext().len() > 256 {
                                                                        log_error_and_exit(
                                                                            self.yytextpos(),
                                                                            CompilerError::Lexer(format!("Invalid string length {}", self.yytext().len())),
                                                                            self.offset,
                                                                            true,
                                                                            self.ctx
                                                                        )
                                                                    }
                                                                    return Ok(TokenKind::TokenStringLiteral);
                                                                } }
                    120 => { /* nothing */ }
                    30 => { return Ok(TokenKind::TokenId); }
                    121 => { /* nothing */ }
                    31 => { return Ok(TokenKind::TokenIf); }
                    122 => { /* nothing */ }
                    32 => { return Ok(TokenKind::TokenId); }
                    123 => { /* nothing */ }
                    33 => { return Ok(TokenKind::TokenId); }
                    124 => { /* nothing */ }
                    34 => { return Ok(TokenKind::TokenId); }
                    125 => { /* nothing */ }
                    35 => { return Ok(TokenKind::TokenId); }
                    126 => { /* nothing */ }
                    36 => { return Ok(TokenKind::TokenId); }
                    127 => { /* nothing */ }
                    37 => { return Ok(TokenKind::TokenOr); }
                    128 => { /* nothing */ }
                    38 => { return Ok(TokenKind::TokenId); }
                    129 => { /* nothing */ }
                    39 => { return Ok(TokenKind::TokenId); }
                    130 => { /* nothing */ }
                    40 => { return Ok(TokenKind::TokenId); }
                    131 => { /* nothing */ }
                    41 => { return Ok(TokenKind::TokenId); }
                    132 => { /* nothing */ }
                    42 => { return Ok(TokenKind::TokenId); }
                    133 => { /* nothing */ }
                    43 => { return Ok(TokenKind::TokenId); }
                    134 => { /* nothing */ }
                    44 => { return Ok(TokenKind::TokenId); }
                    135 => { /* nothing */ }
                    45 => { {
                                                                    if let Err(e) = self.yytext().parse::<i64>() {
                                                                        log_error_and_exit(
                                                                            self.yytextpos(),
                                                                            CompilerError::Lexer(format!("Invalid integer literal {e}")),
                                                                            self.offset,
                                                                            true,
                                                                            self.ctx
                                                                        )
                                                                    }
                                                                    return Ok(TokenKind::TokenIntLiteral);
                                                                } }
                    136 => { /* nothing */ }
                    46 => { {
                                                                    match self.yytext().parse::<f32>() {
                                                                        Err(e) => {
                                                                            log_error_and_exit(
                                                                                self.yytextpos(),
                                                                                CompilerError::Lexer(format!("Invalid float literal {e}")),
                                                                                self.offset,
                                                                                true,
                                                                                self.ctx
                                                                            );
                                                                        }
                                                                        Ok(value) => {
                                                                            if !value.is_normal() {
                                                                                log_error_and_exit(
                                                                                    self.yytextpos(),
                                                                                    CompilerError::Lexer(format!("Invalid float literal")),
                                                                                    self.offset,
                                                                                    true,
                                                                                    self.ctx
                                                                                )
                                                                            }
                                                                        }
                                                                    };
                                                                    return Ok(TokenKind::TokenFloatLiteral);
                                                                } }
                    137 => { /* nothing */ }
                    47 => { return Ok(TokenKind::TokenAssign); }
                    138 => { /* nothing */ }
                    48 => { return Ok(TokenKind::TokenEqual); }
                    139 => { /* nothing */ }
                    49 => { return Ok(TokenKind::TokenNotEqual); }
                    140 => { /* nothing */ }
                    50 => { return Ok(TokenKind::TokenLessEqual); }
                    141 => { /* nothing */ }
                    51 => { return Ok(TokenKind::TokenGreaterEqual); }
                    142 => { /* nothing */ }
                    52 => { return Ok(TokenKind::TokenId); }
                    143 => { /* nothing */ }
                    53 => { return Ok(TokenKind::TokenInt); }
                    144 => { /* nothing */ }
                    54 => { return Ok(TokenKind::TokenId); }
                    145 => { /* nothing */ }
                    55 => { return Ok(TokenKind::TokenNot); }
                    146 => { /* nothing */ }
                    56 => { return Ok(TokenKind::TokenId); }
                    147 => { /* nothing */ }
                    57 => { return Ok(TokenKind::TokenId); }
                    148 => { /* nothing */ }
                    58 => { return Ok(TokenKind::TokenId); }
                    149 => { /* nothing */ }
                    59 => { return Ok(TokenKind::TokenAnd); }
                    150 => { /* nothing */ }
                    60 => { return Ok(TokenKind::TokenId); }
                    151 => { /* nothing */ }
                    61 => { return Ok(TokenKind::TokenId); }
                    152 => { /* nothing */ }
                    62 => { return Ok(TokenKind::TokenId); }
                    153 => { /* nothing */ }
                    63 => { return Ok(TokenKind::TokenId); }
                    154 => { /* nothing */ }
                    64 => { return Ok(TokenKind::TokenId); }
                    155 => { /* nothing */ }
                    65 => { return Ok(TokenKind::TokenId); }
                    156 => { /* nothing */ }
                    66 => { {
                                                                    match self.yytext().parse::<f32>() {
                                                                        Err(e) => {
                                                                            log_error_and_exit(
                                                                                self.yytextpos(),
                                                                                CompilerError::Lexer(format!("Invalid float literal {e}")),
                                                                                self.offset,
                                                                                true,
                                                                                self.ctx
                                                                            );
                                                                        }
                                                                        Ok(value) => {
                                                                            if !value.is_normal() {
                                                                                log_error_and_exit(
                                                                                    self.yytextpos(),
                                                                                    CompilerError::Lexer(format!("Invalid float literal")),
                                                                                    self.offset,
                                                                                    true,
                                                                                    self.ctx
                                                                                )
                                                                            }
                                                                        }
                                                                    };
                                                                    return Ok(TokenKind::TokenFloatLiteral);
                                                                } }
                    157 => { /* nothing */ }
                    67 => { {
                                                                    if let Err(e) = self.yytext().parse::<i64>() {
                                                                        log_error_and_exit(
                                                                            self.yytextpos(),
                                                                            CompilerError::Lexer(format!("Invalid integer literal {e}")),
                                                                            self.offset,
                                                                            true,
                                                                            self.ctx
                                                                        )
                                                                    }
                                                                    return Ok(TokenKind::TokenIntLiteral);
                                                                } }
                    158 => { /* nothing */ }
                    68 => { return Ok(TokenKind::TokenInit); }
                    159 => { /* nothing */ }
                    69 => { return Ok(TokenKind::TokenId); }
                    160 => { /* nothing */ }
                    70 => { return Ok(TokenKind::TokenTrue); }
                    161 => { /* nothing */ }
                    71 => { return Ok(TokenKind::TokenId); }
                    162 => { /* nothing */ }
                    72 => { return Ok(TokenKind::TokenId); }
                    163 => { /* nothing */ }
                    73 => { return Ok(TokenKind::TokenId); }
                    164 => { /* nothing */ }
                    74 => { return Ok(TokenKind::TokenRead); }
                    165 => { /* nothing */ }
                    75 => { return Ok(TokenKind::TokenId); }
                    166 => { /* nothing */ }
                    76 => { return Ok(TokenKind::TokenId); }
                    167 => { /* nothing */ }
                    77 => { return Ok(TokenKind::TokenElse); }
                    168 => { /* nothing */ }
                    78 => { return Ok(TokenKind::TokenId); }
                    169 => { /* nothing */ }
                    79 => { return Ok(TokenKind::TokenId); }
                    170 => { /* nothing */ }
                    80 => { return Ok(TokenKind::TokenFloat); }
                    171 => { /* nothing */ }
                    81 => { return Ok(TokenKind::TokenFalse); }
                    172 => { /* nothing */ }
                    82 => { return Ok(TokenKind::TokenId); }
                    173 => { /* nothing */ }
                    83 => { return Ok(TokenKind::TokenWrite); }
                    174 => { /* nothing */ }
                    84 => { return Ok(TokenKind::TokenWhile); }
                    175 => { /* nothing */ }
                    85 => { return Ok(TokenKind::TokenId); }
                    176 => { /* nothing */ }
                    86 => { return Ok(TokenKind::TokenIsZero); }
                    177 => { /* nothing */ }
                    87 => { return Ok(TokenKind::TokenString); }
                    178 => { /* nothing */ }
                    88 => { return Ok(TokenKind::TokenId); }
                    179 => { /* nothing */ }
                    89 => { return Ok(TokenKind::TokenId); }
                    180 => { /* nothing */ }
                    90 => { return Ok(TokenKind::TokenConvDate); }
                    181 => { /* nothing */ }
                    91 => { return Ok(TokenKind::TokenDate); }
                    182 => { /* nothing */ }

                    _ => {
                        return Err(Error::Unmatch);
                    }
                }
            }
        }   // loop
        // never reach end of function
    }

}
