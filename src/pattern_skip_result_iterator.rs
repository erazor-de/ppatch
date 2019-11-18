use crate::{PatternSearchType, Skipper};

pub struct PatternSkipResultIterator<I, T> {
    iter: I,
    skipper: Skipper<T>,
}

impl<I, T> PatternSkipResultIterator<I, T>
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

impl<I, E, T> Iterator for PatternSkipResultIterator<I, T>
where
    I: Iterator<Item = Result<PatternSearchType<T>, E>>,
    E: std::error::Error,
{
    type Item = Result<PatternSearchType<T>, E>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(byte) = self.skipper.handle_existing_data() {
            return Some(Ok(PatternSearchType::NonMatch(byte)));
        }

        match self.iter.next() {
            Some(result) => match result {
                Ok(search_type) => Some(Ok(self.skipper.handle_next(search_type))),
                Err(error) => Some(Err(error)),
            },
            None => None,
        }
    }
}

pub trait PatternSkipResultExt<E, T>:
    Iterator<Item = Result<PatternSearchType<T>, E>> + Sized
where
    E: std::error::Error,
{
    fn skip_pattern(self, count: usize) -> PatternSkipResultIterator<Self, T>;
}

impl<I, E, T> PatternSkipResultExt<E, T> for I
where
    I: Iterator<Item = Result<PatternSearchType<T>, E>>,
    E: std::error::Error,
{
    fn skip_pattern(self, count: usize) -> PatternSkipResultIterator<Self, T> {
        PatternSkipResultIterator::new(self, count)
    }
}
