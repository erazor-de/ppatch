use crate::{Pattern, PatternSearchType, Searcher};
use std::fmt;
use std::iter::Iterator;
use std::ops;

pub struct PatternSearchResultIterator<'a, I, T> {
    iter: I,
    searcher: Searcher<'a, T>,
}

impl<'a, I, T> PatternSearchResultIterator<'a, I, T>
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

impl<'a, I, E, T> Iterator for PatternSearchResultIterator<'a, I, T>
where
    I: Iterator<Item = Result<T, E>>,
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
    type Item = Result<PatternSearchType<T>, E>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(result) = self.searcher.handle_existing_data() {
            return Some(Ok(result));
        }

        while let Some(result) = self.iter.next() {
            match result {
                Ok(byte) => {
                    if let Some(result) = self.searcher.handle_next(byte) {
                        return Some(Ok(result));
                    }
                }
                Err(error) => {
                    return Some(Err(error));
                }
            }
        }

        match self.searcher.handle_existing_data() {
            Some(result) => Some(Ok(result)),
            None => None,
        }
    }
}

pub trait PatternSearchResultExt<'a, E, T>: Iterator<Item = Result<T, E>> + Sized
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
    fn search_pattern(self, pattern: &'a Pattern<T>) -> PatternSearchResultIterator<'a, Self, T>;
}

impl<'a, I, E, T> PatternSearchResultExt<'a, E, T> for I
where
    I: Iterator<Item = Result<T, E>>,
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
    fn search_pattern(self, pattern: &'a Pattern<T>) -> PatternSearchResultIterator<'a, Self, T> {
        PatternSearchResultIterator::new(self, pattern)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;
    use std::str::FromStr;

    #[test]
    fn search_global() {
        let d = [0x1a, 0x2b, 0x3c, 0x4d, 0x5e, 0x6f];
        let s = Pattern::<u8>::from_str("0b???0???? 0b???1????").unwrap();
        let mut iter = d.bytes().search_pattern(&s);
        assert_eq!(
            PatternSearchType::NonMatch(0x1a),
            iter.next().unwrap().unwrap()
        );
        assert_eq!(
            PatternSearchType::Match {
                data: [0x2b, 0x3c].to_vec(),
                index: 1
            },
            iter.next().unwrap().unwrap()
        );
        assert_eq!(
            PatternSearchType::Match {
                data: [0x4d, 0x5e].to_vec(),
                index: 3
            },
            iter.next().unwrap().unwrap()
        );
        // partial match before end
        assert_eq!(
            PatternSearchType::NonMatch(0x6f),
            iter.next().unwrap().unwrap()
        );
        assert!(iter.next().is_none());
    }
}
