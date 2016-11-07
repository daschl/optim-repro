#![feature(test)]
extern crate test;
extern crate optim_repro;

use optim_repro::{Tokenizer, Token};
use test::Bencher;

static INPUT: &'static str =
    "In addition to conventional static typing, before version 0.4, Rust also supported \
     typestates. The typestate system modeled assertions before and after program statements, \
     through use of a special check statement. Discrepancies could be discovered at compile time, \
     rather than when a program was running, as might be the case with assertions in C or C++ \
     code. The typestate concept was not unique to Rust, as it was first introduced in the \
     language NIL. Typestates were removed because in practice they found little use, though the \
     same functionality can still be achieved with branding patterns.

The style changed between \
     0.2, 0.3 and 0.4. Version 0.2 introduced classes for the first time, with version 0.3 adding \
     a number of features including destructors and polymorphism through the use of interfaces. \
     In Rust 0.4, traits were added as a means to provide inheritance; In January 2014, the \
     editor-in-chief of Dr Dobb's, Andrew Binstock, commented on Rust's chances to become a \
     competitor to C++.";

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

#[bench]
fn bench_allinone(b: &mut Bencher) {
    let t = WhitespaceTokenizer;
    b.iter(|| t.tokenize(INPUT).last());
}
