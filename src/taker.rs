use crate::{OptFifo, PatternSearchType};

pub struct Taker<T> {
    found: usize,
    count: usize,
    data: OptFifo<T>,
}

impl<T> Taker<T> {
    pub fn new(count: usize) -> Self {
        Self {
            found: 0,
            count,
            data: OptFifo::new(),
        }
    }

    pub fn handle_existing_data(&mut self) -> Option<T> {
        self.data.get()
    }

    pub fn handle_next(&mut self, search_type: PatternSearchType<T>) -> PatternSearchType<T> {
        match search_type {
            PatternSearchType::Match { mut data, index } => {
                self.found += 1;
                if self.found > self.count {
                    let byte = data.remove(0);
                    self.data.set(data);
                    return PatternSearchType::NonMatch(byte);
                } else {
                    return PatternSearchType::Match { data, index };
                }
            }
            PatternSearchType::NonMatch(_) => return search_type,
        }
    }
}
