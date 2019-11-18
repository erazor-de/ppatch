use crate::{PatternSearchType, Taker};

pub struct PatternTakeResultIterator<I, T> {
    iter: I,
    taker: Taker<T>,
}

impl<I, T> PatternTakeResultIterator<I, T>
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

impl<I, E, T> Iterator for PatternTakeResultIterator<I, T>
where
    I: Iterator<Item = Result<PatternSearchType<T>, E>>,
    E: std::error::Error,
{
    type Item = Result<PatternSearchType<T>, E>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(byte) = self.taker.handle_existing_data() {
            return Some(Ok(PatternSearchType::NonMatch(byte)));
        }

        match self.iter.next() {
            Some(result) => match result {
                Ok(search_type) => Some(Ok(self.taker.handle_next(search_type))),
                Err(error) => Some(Err(error)),
            },
            None => None,
        }
    }
}

pub trait PatternTakeResultExt<E, T>:
    Iterator<Item = Result<PatternSearchType<T>, E>> + Sized
where
    E: std::error::Error,
{
    fn take_pattern(self, count: usize) -> PatternTakeResultIterator<Self, T>;
}

impl<I, E, T> PatternTakeResultExt<E, T> for I
where
    I: Iterator<Item = Result<PatternSearchType<T>, E>>,
    E: std::error::Error,
{
    fn take_pattern(self, count: usize) -> PatternTakeResultIterator<Self, T> {
        PatternTakeResultIterator::new(self, count)
    }
}
