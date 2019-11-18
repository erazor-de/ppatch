use crate::{Pattern, PatternSearchType, Replacer};
use std::fmt;
use std::ops;

pub struct PatternReplaceIterator<'a, I, T> {
    iter: I,
    replacer: Replacer<'a, T>,
}

impl<'a, I, T> PatternReplaceIterator<'a, I, T>
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
    pub fn new(iter: I, pattern: &'a Pattern<T>) -> Self {
        Self {
            iter,
            replacer: Replacer::new(pattern),
        }
    }
}

impl<'a, I, T> Iterator for PatternReplaceIterator<'a, I, T>
where
    I: Iterator<Item = PatternSearchType<T>>,
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
    type Item = crate::Result<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(result) = self.replacer.handle_existing_data() {
            return Some(Ok(result));
        }

        match self.iter.next() {
            Some(search_type) => Some(self.replacer.handle_next(search_type)),
            None => None,
        }
    }
}

pub trait PatternReplaceExt<'a, T>: Iterator<Item = PatternSearchType<T>> + Sized
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
    fn replace_pattern(self, pattern: &'a Pattern<T>) -> PatternReplaceIterator<'a, Self, T>;
}

impl<'a, I, T> PatternReplaceExt<'a, T> for I
where
    I: Iterator<Item = PatternSearchType<T>>,
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
    fn replace_pattern(self, pattern: &'a Pattern<T>) -> PatternReplaceIterator<'a, Self, T> {
        PatternReplaceIterator::new(self, pattern)
    }
}
