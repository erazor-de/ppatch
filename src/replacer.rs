use crate::{OptFifo, Pattern, PatternSearchType};
use std::fmt;
use std::ops;

pub struct Replacer<'a, T> {
    pattern: &'a Pattern<T>,
    data: OptFifo<T>,
}

impl<'a, T> Replacer<'a, T>
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
        Self {
            pattern,
            data: OptFifo::new(),
        }
    }

    // Emptying an existing match vector
    // Returns None if vector is used up and removed else the next byte is returned
    pub fn handle_existing_data(&mut self) -> Option<T> {
        self.data.get()
    }

    // Handles PatternSearchType by returning a single byte or an error that
    // happened in the replace process
    pub fn handle_next(&mut self, search_type: PatternSearchType<T>) -> crate::Result<T> {
        match search_type {
            PatternSearchType::NonMatch(byte) => Ok(byte),
            PatternSearchType::Match { data, index: _ } => match self.pattern.replace(data) {
                Ok(mut item) => {
                    let byte = item.remove(0);
                    self.data.set(item);
                    Ok(byte)
                }
                Err(error) => Err(error),
            },
        }
    }
}
