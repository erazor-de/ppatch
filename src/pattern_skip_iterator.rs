use crate::{PatternSearchType, Skipper};

pub struct PatternSkipIterator<I, T> {
    iter: I,
    skipper: Skipper<T>,
}

impl<I, T> PatternSkipIterator<I, T>
where
    I: Iterator,
{
    pub fn new(iter: I, count: usize) -> Self {
        Self {
            iter,
            skipper: Skipper::new(count),
        }
    }
}

impl<I, T> Iterator for PatternSkipIterator<I, T>
where
    I: Iterator<Item = PatternSearchType<T>>,
{
    type Item = PatternSearchType<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(byte) = self.skipper.handle_existing_data() {
            return Some(PatternSearchType::NonMatch(byte));
        }

        match self.iter.next() {
            Some(search_type) => Some(self.skipper.handle_next(search_type)),
            None => None,
        }
    }
}

pub trait PatternSkipExt<T>: Iterator<Item = PatternSearchType<T>> + Sized {
    fn skip_pattern(self, count: usize) -> PatternSkipIterator<Self, T>;
}

impl<I, T> PatternSkipExt<T> for I
where
    I: Iterator<Item = PatternSearchType<T>>,
{
    fn skip_pattern(self, count: usize) -> PatternSkipIterator<Self, T> {
        PatternSkipIterator::new(self, count)
    }
}
