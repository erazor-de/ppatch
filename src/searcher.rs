use crate::{Pattern, PatternSearchType};
use std::fmt;
use std::mem;
use std::ops;

pub struct Searcher<'a, T> {
    pattern: &'a Pattern<T>,
    matched: bool,
    data: Vec<T>,
    taken: usize,
}

impl<'a, T> Searcher<'a, T>
where
    T: From<u8>
        + fmt::Binary
        + num::PrimInt
        + num::Unsigned
        + Default
        + ops::ShlAssign<u32>
        + PartialEq
        + num::PrimInt<FromStrRadixErr = std::num::ParseIntError>
        + ops::BitOrAssign
        + ops::BitAndAssign,
{
    pub fn new(pattern: &'a Pattern<T>) -> Self {
        let capacity = pattern.len();
        Self {
            pattern,
            matched: false,
            data: Vec::with_capacity(capacity),
            taken: 0,
        }
    }

    // Handles remaining data of a partial match
    pub fn handle_existing_data(&mut self) -> Option<PatternSearchType<T>> {
        if !self.data.is_empty() {
            let byte = self.data.remove(0);
            return Some(PatternSearchType::NonMatch(byte));
        }
        None
    }

    // Handles next input byte. Needs to be called after handle_existing_data returns None and
    // returns None if byte is eaten as part of a possible match
    pub fn handle_next(&mut self, byte: T) -> Option<PatternSearchType<T>> {
        self.taken += 1;
        self.data.push(byte);

        if self.pattern.get(self.data.len() - 1).unwrap().matches(byte) {
            if self.data.len() == self.pattern.len() {
                self.matched = true;
                let mut other = Vec::with_capacity(self.pattern.len());
                mem::swap(&mut other, &mut self.data);
                return Some(PatternSearchType::Match {
                    data: other,
                    index: self.taken - self.pattern.len(),
                });
            }
            None
        } else {
            let byte = self.data.remove(0);
            Some(PatternSearchType::NonMatch(byte))
        }
    }
}
