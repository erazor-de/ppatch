use crate::{Pattern, PatternSearchType, Replacer};
use std::fmt;
use std::ops;

pub struct PatternReplaceResultIterator<'a, I, T> {
    iter: I,
    replacer: Replacer<'a, T>,
}

impl<'a, I, T> PatternReplaceResultIterator<'a, I, T>
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

impl<'a, I, E, T> Iterator for PatternReplaceResultIterator<'a, I, T>
where
    I: Iterator<Item = std::result::Result<PatternSearchType<T>, E>>,
    E: 'static + std::error::Error,
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
            Some(result) => match result {
                Ok(search_type) => Some(self.replacer.handle_next(search_type)),
                Err(error) => Some(Err(crate::Error::IteratorError {
                    source: error.into(),
                })),
            },
            None => None,
        }
    }
}

pub trait PatternReplaceResultExt<'a, E, T>:
    Iterator<Item = Result<PatternSearchType<T>, E>> + Sized
where
    E: std::error::Error,
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
    fn replace_pattern(self, pattern: &'a Pattern<T>) -> PatternReplaceResultIterator<'a, Self, T>;
}

impl<'a, I, E, T> PatternReplaceResultExt<'a, E, T> for I
where
    I: Iterator<Item = Result<PatternSearchType<T>, E>>,
    E: std::error::Error,
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
    fn replace_pattern(self, pattern: &'a Pattern<T>) -> PatternReplaceResultIterator<'a, Self, T> {
        PatternReplaceResultIterator::new(self, pattern)
    }
}
