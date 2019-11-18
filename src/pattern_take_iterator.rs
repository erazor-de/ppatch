use crate::{PatternSearchType, Taker};

pub struct PatternTakeIterator<I, T> {
    iter: I,
    taker: Taker<T>,
}

impl<I, T> PatternTakeIterator<I, T>
where
    I: Iterator,
{
    pub fn new(iter: I, count: usize) -> Self {
        Self {
            iter,
            taker: Taker::new(count),
        }
    }
}

impl<I, T> Iterator for PatternTakeIterator<I, T>
where
    I: Iterator<Item = PatternSearchType<T>>,
{
    type Item = PatternSearchType<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(byte) = self.taker.handle_existing_data() {
            return Some(PatternSearchType::NonMatch(byte));
        }

        match self.iter.next() {
            Some(search_type) => Some(self.taker.handle_next(search_type)),
            None => None,
        }
    }
}

pub trait PatternTakeExt<T>: Iterator<Item = PatternSearchType<T>> + Sized {
    fn take_pattern(self, count: usize) -> PatternTakeIterator<Self, T>;
}

impl<I, T> PatternTakeExt<T> for I
where
    I: Iterator<Item = PatternSearchType<T>>,
{
    fn take_pattern(self, count: usize) -> PatternTakeIterator<Self, T> {
        PatternTakeIterator::new(self, count)
    }
}
