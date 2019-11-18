use crate::{Pattern, PatternSearchType, Searcher};
use std::fmt;
use std::iter::Iterator;
use std::ops;

pub struct PatternSearchIterator<'a, I, T> {
    iter: I,
    searcher: Searcher<'a, T>,
}

impl<'a, I, T> PatternSearchIterator<'a, I, T>
where
    I: Iterator,
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
            searcher: Searcher::new(pattern),
        }
    }
}

impl<'a, I, T> Iterator for PatternSearchIterator<'a, I, T>
where
    I: Iterator<Item = T>,
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
    type Item = PatternSearchType<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(result) = self.searcher.handle_existing_data() {
            return Some(result);
        }

        while let Some(byte) = self.iter.next() {
            if let Some(result) = self.searcher.handle_next(byte) {
                return Some(result);
            }
        }

        self.searcher.handle_existing_data()
    }
}

pub trait PatternSearchExt<'a, T>: Iterator<Item = T> + Sized
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
    fn search_pattern(self, pattern: &'a Pattern<T>) -> PatternSearchIterator<'a, Self, T>;
}

impl<'a, I, T> PatternSearchExt<'a, T> for I
where
    I: Iterator<Item = T>,
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
    fn search_pattern(self, pattern: &'a Pattern<T>) -> PatternSearchIterator<'a, Self, T> {
        PatternSearchIterator::new(self, pattern)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn search_global() {
        let d = [0x1a, 0x2b, 0x3c, 0x4d, 0x5e, 0x6f];
        let s = Pattern::<u8>::from_str("0b???0???? 0b???1????").unwrap();
        let mut iter = d.to_vec().into_iter().search_pattern(&s);
        assert_eq!(Some(PatternSearchType::NonMatch(0x1a)), iter.next());
        assert_eq!(
            Some(PatternSearchType::Match {
                data: [0x2b, 0x3c].to_vec(),
                index: 1
            }),
            iter.next()
        );
        assert_eq!(
            Some(PatternSearchType::Match {
                data: [0x4d, 0x5e].to_vec(),
                index: 3
            }),
            iter.next()
        );
        // partial match before end
        assert_eq!(Some(PatternSearchType::NonMatch(0x6f)), iter.next());
        assert_eq!(None, iter.next());
    }
}
