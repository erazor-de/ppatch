/// Container which can be given a Vector and returns its single elements on request
pub struct OptFifo<T> {
    data: Option<Vec<T>>,
}

impl<T> OptFifo<T> {
    /// Initializes an empty OptFifo.
    pub fn new() -> Self {
        Self { data: None }
    }

    /// A new value can only be set if OptFifo::get() returned None once.
    pub fn set(&mut self, data: Vec<T>) {
        assert!(self.data.is_none());
        self.data = Some(data);
    }

    /// Returns some value or None if contained Vector is empty. In this case
    /// the Vector gets removed and a new Vector can be set.
    pub fn get(&mut self) -> Option<T> {
        if let Some(ref mut data) = self.data {
            if data.is_empty() {
                self.data = None;
            } else {
                let element = data.remove(0);
                return Some(element);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lifecycle() {
        let mut fifo = OptFifo::new();

        // Starts empty
        assert_eq!(fifo.get(), None);

        fifo.set(vec![2, 3]);

        assert_eq!(fifo.get(), Some(2));
        assert_eq!(fifo.get(), Some(3));
        assert_eq!(fifo.get(), None);

        // Make sure None is returned repeatedly
        assert_eq!(fifo.get(), None);
    }

    #[test]
    #[should_panic]
    fn test_double_set() {
        let mut fifo = OptFifo::<i32>::new();
        fifo.set(Vec::new());
        fifo.set(Vec::new());
    }
}
