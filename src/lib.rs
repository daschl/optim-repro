extern crate arrayvec;

use arrayvec::ArrayString;

const MAX_STACK_TERM_LEN: usize = 23;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Token<'a> {
    term: Term<'a>,
    start_offset: usize,
    position: usize,
}

#[derive(Debug, Eq, Hash)]
enum Term<'a> {
    Borrowed(&'a str),
    Stack(ArrayString<[u8; MAX_STACK_TERM_LEN]>),
    Heap(String),
}

impl<'a> PartialEq for Term<'a> {
    fn eq(&self, other: &Term<'a>) -> bool {
        let this = match *self {
            Term::Stack(ref s) => s.as_ref(),
            Term::Heap(ref s) => s.as_ref(),
            Term::Borrowed(ref s) => s,
        };

        let that = match *other {
            Term::Stack(ref s) => s.as_ref(),
            Term::Heap(ref s) => s.as_ref(),
            Term::Borrowed(ref s) => s,
        };
        this == that
    }
}

impl<'a> Token<'a> {
    #[inline]
    pub fn from_str(term: &'a str, start_offset: usize, position: usize) -> Self {
        Token {
            term: Term::Borrowed(term),
            start_offset: start_offset,
            position: position,
        }
    }
}

pub trait Tokenizer<'a> {
    type TokenIter: Iterator<Item = Token<'a>>;
    fn tokenize(&self, input: &'a str) -> Self::TokenIter;
}

pub struct CharTokenIter<'a> {
    filter: fn(char) -> bool,
    input: &'a str,
    byte_offset: usize,
    char_offset: usize,
    position: usize,
}

impl<'a> CharTokenIter<'a> {
    pub fn new(filter: fn(char) -> bool, input: &'a str) -> Self {
        CharTokenIter {
            filter: filter,
            input: input,
            byte_offset: 0,
            char_offset: 0,
            position: 0,
        }
    }
}

impl<'a> Iterator for CharTokenIter<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Token<'a>> {
        let mut skipped_bytes = 0;
        let mut skipped_chars = 0;
        self.input[self.byte_offset..]
            .char_indices()
            .enumerate()
            .skip_while(|&(_, (_, c))| if (self.filter)(c) {
                skipped_bytes += c.len_utf8();
                skipped_chars += 1;
                true
            } else {
                false
            })
            .find(|&(_, (_, c))| (self.filter)(c))
            .map(|(cidx, (bidx, _))| {
                let slice = &self.input[self.byte_offset + skipped_bytes..self.byte_offset + bidx];
                let token = Token::from_str(slice, self.char_offset + skipped_chars, self.position);
                self.byte_offset += bidx + 1;
                self.char_offset += cidx + 1;
                self.position += 1;
                token
            })
            .or_else(|| {
                if self.byte_offset + skipped_bytes < self.input.len() {
                    let slice = &self.input[self.byte_offset + skipped_bytes..];
                    let token =
                        Token::from_str(slice, self.char_offset + skipped_chars, self.position);
                    self.byte_offset = self.input.len();
                    Some(token)
                } else {
                    None
                }
            })
    }
}

pub struct WhitespaceTokenizer;

impl<'a> Tokenizer<'a> for WhitespaceTokenizer {
    type TokenIter = CharTokenIter<'a>;

    fn tokenize(&self, input: &'a str) -> Self::TokenIter {
        CharTokenIter::new(is_whitespace, input)
    }
}

#[inline]
pub fn is_whitespace(c: char) -> bool {
    c.is_whitespace()
}
