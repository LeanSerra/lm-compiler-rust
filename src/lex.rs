use crate::grammar::TokenKind;
use crate::grammar_lexer::log_error;
use crate::read_source_to_string;


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
}

impl<'a> Lexer<'a> {
    pub const ZZ_ROW: [usize; 111] = [0, 45, 90, 135, 180, 225, 270, 315, 360, 405, 450, 495, 540, 585, 630, 675, 720, 765, 810, 855, 810, 900, 945, 810, 810, 810, 810, 810, 810, 810, 810, 990, 1035, 1080, 810, 45, 1125, 1170, 1215, 405, 1260, 1305, 1350, 1395, 1440, 405, 1485, 1530, 1575, 1620, 1665, 1710, 1755, 1800, 1845, 1890, 810, 810, 810, 810, 810, 1935, 1980, 2025, 405, 2070, 405, 2115, 2160, 2205, 405, 2250, 2295, 2340, 2385, 2430, 2475, 2520, 2520, 2565, 2610, 405, 2655, 405, 2700, 2745, 2790, 405, 2835, 2880, 405, 2925, 2970, 3015, 405, 405, 3060, 405, 405, 3105, 3150, 405, 405, 3195, 3240, 3285, 3330, 405, 3375, 3420, 810];
    pub const ZZ_TRANS: [i32; 3465] = [1, 2, -1, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 9, 14, 9, 15, 9, 9, 9, 16, 9, 9, 17, 18, 19, 9, 20, 9, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 34, 35, -1, 36, 1, -1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 37, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 34, 34, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 34, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 34, -1, -1, -1, -1, -1, 9, 38, 9, 39, 9, 9, 9, 40, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 41, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 9, 9, 9, 42, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 43, 9, 44, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 9, 9, 9, 45, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 46, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 47, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 48, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 9, 9, 9, 49, 9, 9, 50, 9, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 51, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 52, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 53, -1, -1, -1, -1, -1, -1, 54, -1, 55, 53, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 55, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 56, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 57, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 58, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 59, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 60, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 1, 1, 1, -1, -1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 37, -1, 61, 37, -1, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 62, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, -1, -1, -1, -1, -1, 63, 9, 64, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 65, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 66, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 67, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 68, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 69, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 70, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 9, 9, 9, 71, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 9, 72, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 73, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 74, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 9, 9, 75, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 76, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 77, 78, -1, -1, 78, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 53, -1, -1, -1, -1, -1, -1, 79, 80, 55, 53, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 53, -1, -1, -1, -1, -1, -1, 55, -1, -1, 53, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 37, 37, 37, -1, -1, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 34, 61, 37, -1, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 62, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, -1, -1, -1, -1, -1, 9, 9, 81, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 82, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 83, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 9, 84, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 9, 9, 85, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 86, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 87, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 88, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 89, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 90, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 91, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 77, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 53, -1, -1, -1, -1, -1, -1, 79, -1, 55, 53, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 92, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 9, 9, 9, 93, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 94, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 95, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 96, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 97, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 98, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 99, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 100, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 101, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 9, 9, 9, 9, 102, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 9, 103, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 104, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 105, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 106, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 107, 9, 9, 9, 9, 9, 9, 9, -1, -1, 9, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 108, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 109, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 110, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1];
    pub const ZZ_ATTR: [i32; 111] = [0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9, 0, 9, 1, 0, 9, 9, 9, 9, 9, 9, 9, 9, 0, 1, 1, 9, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 9, 9, 9, 9, 9, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 0, 1, 0, 0, 9];
    pub const ZZ_ACTION: [i32; 111] = [0, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 0, 17, 18, 0, 19, 20, 21, 22, 23, 24, 25, 26, 0, 27, 28, 29, 30, 0, 0, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 0, 46, 47, 48, 49, 50, 51, 52, 0, 0, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 0, 68, 0, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 0, 80, 81, 82, 83, 84, 85, 86, 0, 87, 88, 89, 0, 90, 0, 91, 0, 0, 92];
    pub const ZZ_LEXSTATE: [i32; 2] = [0, 0];
    pub const YYINITIAL: usize = 0;


    pub const YYEOF: i32 = -1;

    pub fn new(input: &'a str, offset: usize) -> Lexer<'a> {
        let max_len = input.chars().clone().count();
        let mut cmap: Vec<usize> = Vec::with_capacity(256);
        cmap.resize(256, 0);
        let mut cmap2: HashMap<usize, usize> = HashMap::new();
        cmap[9] = 44;
        cmap[10] = 4;
        cmap[11] = 4;
        cmap[12] = 4;
        cmap[13] = 3;
        cmap[32] = 43;
        cmap[33] = 40;
        cmap[35] = 1;
        cmap[40] = 34;
        cmap[41] = 35;
        cmap[42] = 32;
        cmap[43] = 28;
        cmap[44] = 39;
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
        cmap[59] = 38;
        cmap[60] = 41;
        cmap[61] = 31;
        cmap[62] = 42;
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
        }
    }


    #[allow(dead_code)]
    pub fn get_offset(&mut self) -> &mut usize { &mut self.offset }

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
                    93 => { /* nothing */ }
                    2 => {  }
                    94 => { /* nothing */ }
                    3 => { return Ok(TokenKind::TokenId); }
                    95 => { /* nothing */ }
                    4 => { return Ok(TokenKind::TokenId); }
                    96 => { /* nothing */ }
                    5 => { return Ok(TokenKind::TokenId); }
                    97 => { /* nothing */ }
                    6 => { return Ok(TokenKind::TokenId); }
                    98 => { /* nothing */ }
                    7 => { return Ok(TokenKind::TokenId); }
                    99 => { /* nothing */ }
                    8 => { return Ok(TokenKind::TokenId); }
                    100 => { /* nothing */ }
                    9 => { return Ok(TokenKind::TokenId); }
                    101 => { /* nothing */ }
                    10 => { return Ok(TokenKind::TokenId); }
                    102 => { /* nothing */ }
                    11 => { return Ok(TokenKind::TokenId); }
                    103 => { /* nothing */ }
                    12 => { return Ok(TokenKind::TokenId); }
                    104 => { /* nothing */ }
                    13 => { return Ok(TokenKind::TokenId); }
                    105 => { /* nothing */ }
                    14 => { return Ok(TokenKind::TokenId); }
                    106 => { /* nothing */ }
                    15 => { {
                                                                    if let Err(e) = self.yytext().parse::<u64>() {
                                                                        log_error(
                                                                            self.yytextpos(),
                                                                            crate::CompilerError::Lexer(format!("Invalid integer literal {e}")),
                                                                            self.offset,
                                                                            &read_source_to_string().unwrap(),
                                                                        );
                                                                        std::process::exit(1)
                                                                    }
                                                                    return Ok(TokenKind::TokenIntLiteral);
                                                                } }
                    107 => { /* nothing */ }
                    16 => { return Ok(TokenKind::TokenSub); }
                    108 => { /* nothing */ }
                    17 => { return Ok(TokenKind::TokenSum); }
                    109 => { /* nothing */ }
                    18 => { return Ok(TokenKind::TokenColon); }
                    110 => { /* nothing */ }
                    19 => { return Ok(TokenKind::TokenMul); }
                    111 => { /* nothing */ }
                    20 => { return Ok(TokenKind::TokenDiv); }
                    112 => { /* nothing */ }
                    21 => { return Ok(TokenKind::TokenParOpen); }
                    113 => { /* nothing */ }
                    22 => { return Ok(TokenKind::TokenParClose); }
                    114 => { /* nothing */ }
                    23 => { return Ok(TokenKind::TokenCBOpen); }
                    115 => { /* nothing */ }
                    24 => { return Ok(TokenKind::TokenCBClose); }
                    116 => { /* nothing */ }
                    25 => { return Ok(TokenKind::TokenSemicolon); }
                    117 => { /* nothing */ }
                    26 => { return Ok(TokenKind::TokenComma); }
                    118 => { /* nothing */ }
                    27 => { return Ok(TokenKind::TokenLess); }
                    119 => { /* nothing */ }
                    28 => { return Ok(TokenKind::TokenGreater); }
                    120 => { /* nothing */ }
                    29 => {  }
                    121 => { /* nothing */ }
                    30 => { {
                                                                    if self.yytext().len() > 256 {
                                                                        log_error(
                                                                            self.yytextpos(),
                                                                            crate::CompilerError::Lexer(format!("Invalid string length {}", self.yytext().len())),
                                                                            self.offset,
                                                                            &read_source_to_string().unwrap(),
                                                                        );
                                                                        std::process::exit(1);
                                                                    }
                                                                    return Ok(TokenKind::TokenStringLiteral);
                                                                } }
                    122 => { /* nothing */ }
                    31 => { return Ok(TokenKind::TokenId); }
                    123 => { /* nothing */ }
                    32 => { return Ok(TokenKind::TokenIf); }
                    124 => { /* nothing */ }
                    33 => { return Ok(TokenKind::TokenId); }
                    125 => { /* nothing */ }
                    34 => { return Ok(TokenKind::TokenId); }
                    126 => { /* nothing */ }
                    35 => { return Ok(TokenKind::TokenId); }
                    127 => { /* nothing */ }
                    36 => { return Ok(TokenKind::TokenId); }
                    128 => { /* nothing */ }
                    37 => { return Ok(TokenKind::TokenId); }
                    129 => { /* nothing */ }
                    38 => { return Ok(TokenKind::TokenOr); }
                    130 => { /* nothing */ }
                    39 => { return Ok(TokenKind::TokenId); }
                    131 => { /* nothing */ }
                    40 => { return Ok(TokenKind::TokenId); }
                    132 => { /* nothing */ }
                    41 => { return Ok(TokenKind::TokenId); }
                    133 => { /* nothing */ }
                    42 => { return Ok(TokenKind::TokenId); }
                    134 => { /* nothing */ }
                    43 => { return Ok(TokenKind::TokenId); }
                    135 => { /* nothing */ }
                    44 => { return Ok(TokenKind::TokenId); }
                    136 => { /* nothing */ }
                    45 => { return Ok(TokenKind::TokenId); }
                    137 => { /* nothing */ }
                    46 => { {
                                                                    if let Err(e) = self.yytext().parse::<u64>() {
                                                                        log_error(
                                                                            self.yytextpos(),
                                                                            crate::CompilerError::Lexer(format!("Invalid integer literal {e}")),
                                                                            self.offset,
                                                                            &read_source_to_string().unwrap(),
                                                                        );
                                                                        std::process::exit(1)
                                                                    }
                                                                    return Ok(TokenKind::TokenIntLiteral);
                                                                } }
                    138 => { /* nothing */ }
                    47 => { {
                                                                    match self.yytext().parse::<f32>() {
                                                                        Err(e) => {
                                                                            log_error(
                                                                                self.yytextpos(),
                                                                                crate::CompilerError::Lexer(format!("Invalid float literal {e}")),
                                                                                self.offset,
                                                                                &read_source_to_string().unwrap(),
                                                                            );
                                                                            std::process::exit(1)
                                                                        }
                                                                        Ok(value) => {
                                                                            if !value.is_normal() {
                                                                                log_error(
                                                                                self.yytextpos(),
                                                                                crate::CompilerError::Lexer(format!("Invalid float literal")),
                                                                                self.offset,
                                                                                &read_source_to_string().unwrap(),
                                                                            );
                                                                            std::process::exit(1)
                                                                            }
                                                                        }
                                                                    };
                                                                    return Ok(TokenKind::TokenFloatLiteral);
                                                                } }
                    139 => { /* nothing */ }
                    48 => { return Ok(TokenKind::TokenAssign); }
                    140 => { /* nothing */ }
                    49 => { return Ok(TokenKind::TokenEqual); }
                    141 => { /* nothing */ }
                    50 => { return Ok(TokenKind::TokenNotEqual); }
                    142 => { /* nothing */ }
                    51 => { return Ok(TokenKind::TokenLessEqual); }
                    143 => { /* nothing */ }
                    52 => { return Ok(TokenKind::TokenGreaterEqual); }
                    144 => { /* nothing */ }
                    53 => { return Ok(TokenKind::TokenId); }
                    145 => { /* nothing */ }
                    54 => { return Ok(TokenKind::TokenInt); }
                    146 => { /* nothing */ }
                    55 => { return Ok(TokenKind::TokenId); }
                    147 => { /* nothing */ }
                    56 => { return Ok(TokenKind::TokenNot); }
                    148 => { /* nothing */ }
                    57 => { return Ok(TokenKind::TokenId); }
                    149 => { /* nothing */ }
                    58 => { return Ok(TokenKind::TokenId); }
                    150 => { /* nothing */ }
                    59 => { return Ok(TokenKind::TokenId); }
                    151 => { /* nothing */ }
                    60 => { return Ok(TokenKind::TokenAnd); }
                    152 => { /* nothing */ }
                    61 => { return Ok(TokenKind::TokenId); }
                    153 => { /* nothing */ }
                    62 => { return Ok(TokenKind::TokenId); }
                    154 => { /* nothing */ }
                    63 => { return Ok(TokenKind::TokenId); }
                    155 => { /* nothing */ }
                    64 => { return Ok(TokenKind::TokenId); }
                    156 => { /* nothing */ }
                    65 => { return Ok(TokenKind::TokenId); }
                    157 => { /* nothing */ }
                    66 => { return Ok(TokenKind::TokenId); }
                    158 => { /* nothing */ }
                    67 => { {
                                                                    match self.yytext().parse::<f32>() {
                                                                        Err(e) => {
                                                                            log_error(
                                                                                self.yytextpos(),
                                                                                crate::CompilerError::Lexer(format!("Invalid float literal {e}")),
                                                                                self.offset,
                                                                                &read_source_to_string().unwrap(),
                                                                            );
                                                                            std::process::exit(1)
                                                                        }
                                                                        Ok(value) => {
                                                                            if !value.is_normal() {
                                                                                log_error(
                                                                                self.yytextpos(),
                                                                                crate::CompilerError::Lexer(format!("Invalid float literal")),
                                                                                self.offset,
                                                                                &read_source_to_string().unwrap(),
                                                                            );
                                                                            std::process::exit(1)
                                                                            }
                                                                        }
                                                                    };
                                                                    return Ok(TokenKind::TokenFloatLiteral);
                                                                } }
                    159 => { /* nothing */ }
                    68 => { {
                                                                    if let Err(e) = self.yytext().parse::<u64>() {
                                                                        log_error(
                                                                            self.yytextpos(),
                                                                            crate::CompilerError::Lexer(format!("Invalid integer literal {e}")),
                                                                            self.offset,
                                                                            &read_source_to_string().unwrap(),
                                                                        );
                                                                        std::process::exit(1)
                                                                    }
                                                                    return Ok(TokenKind::TokenIntLiteral);
                                                                } }
                    160 => { /* nothing */ }
                    69 => { return Ok(TokenKind::TokenInit); }
                    161 => { /* nothing */ }
                    70 => { return Ok(TokenKind::TokenId); }
                    162 => { /* nothing */ }
                    71 => { return Ok(TokenKind::TokenTrue); }
                    163 => { /* nothing */ }
                    72 => { return Ok(TokenKind::TokenId); }
                    164 => { /* nothing */ }
                    73 => { return Ok(TokenKind::TokenId); }
                    165 => { /* nothing */ }
                    74 => { return Ok(TokenKind::TokenId); }
                    166 => { /* nothing */ }
                    75 => { return Ok(TokenKind::TokenRead); }
                    167 => { /* nothing */ }
                    76 => { return Ok(TokenKind::TokenId); }
                    168 => { /* nothing */ }
                    77 => { return Ok(TokenKind::TokenId); }
                    169 => { /* nothing */ }
                    78 => { return Ok(TokenKind::TokenElse); }
                    170 => { /* nothing */ }
                    79 => { return Ok(TokenKind::TokenId); }
                    171 => { /* nothing */ }
                    80 => { return Ok(TokenKind::TokenId); }
                    172 => { /* nothing */ }
                    81 => { return Ok(TokenKind::TokenFloat); }
                    173 => { /* nothing */ }
                    82 => { return Ok(TokenKind::TokenFalse); }
                    174 => { /* nothing */ }
                    83 => { return Ok(TokenKind::TokenId); }
                    175 => { /* nothing */ }
                    84 => { return Ok(TokenKind::TokenWrite); }
                    176 => { /* nothing */ }
                    85 => { return Ok(TokenKind::TokenWhile); }
                    177 => { /* nothing */ }
                    86 => { return Ok(TokenKind::TokenId); }
                    178 => { /* nothing */ }
                    87 => { return Ok(TokenKind::TokenIsZero); }
                    179 => { /* nothing */ }
                    88 => { return Ok(TokenKind::TokenString); }
                    180 => { /* nothing */ }
                    89 => { return Ok(TokenKind::TokenId); }
                    181 => { /* nothing */ }
                    90 => { return Ok(TokenKind::TokenId); }
                    182 => { /* nothing */ }
                    91 => { return Ok(TokenKind::TokenConvDate); }
                    183 => { /* nothing */ }
                    92 => { return Ok(TokenKind::TokenDate); }
                    184 => { /* nothing */ }

                    _ => {
                        return Err(Error::Unmatch);
                    }
                }
            }
        }   // loop
        // never reach end of function
    }

}
