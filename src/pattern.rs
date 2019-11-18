use crate::MaskedByte;
use std::fmt;
use std::ops;
use std::str;

/// Pattern is a container of MaskedBytes.
#[derive(Default)]
pub struct Pattern<T> {
    values: Vec<MaskedByte<T>>,
}

impl<T> str::FromStr for Pattern<T>
where
    T: From<u8>
        + ops::ShlAssign<u32>
        + ops::BitOrAssign
        + ops::BitAndAssign
        + PartialEq
        + num::PrimInt
        + num::Unsigned
        + Default
        + num::PrimInt<FromStrRadixErr = std::num::ParseIntError>,
{
    type Err = crate::Error;

    /// Input is a list of MaskedBytes separated with whitespace
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut values = Vec::new();

        for part in string.split_whitespace() {
            let v: MaskedByte<T> = MaskedByte::from_str(part)?;
            values.push(v);
        }
        Ok(Pattern { values })
    }
}

impl<T> fmt::Debug for Pattern<T>
where
    T: fmt::Binary + num::PrimInt,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Pattern [{:?}]", self.values)
    }
}

impl<T> fmt::Display for Pattern<T>
where
    T: num::PrimInt + ops::ShrAssign<u8>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[ ")?;
        for b in &self.values {
            write!(f, "{} ", b)?;
        }
        write!(f, "]")
    }
}

impl<T> Pattern<T>
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
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, index: usize) -> Option<&MaskedByte<T>> {
        self.values.get(index)
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    // data is taken, elements are replaced/added and returned
    pub fn replace(&self, mut data: Vec<T>) -> crate::Result<Vec<T>> {
        let mut index = 0;

        for masked_byte in &self.values {
            match data.get_mut(index) {
                Some(byte) => {
                    *byte = masked_byte.set(*byte);
                }
                None => match masked_byte.defined() {
                    Some(value) => data.push(value),
                    None => return Err(crate::Error::ReplaceNotDefined),
                },
            }

            index += 1;
        }
        Ok(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn string_conversion() {
        let p = Pattern::<u8>::from_str("0b?01? 0x?f");
        assert!(p.is_ok());
        let p = p.unwrap();
        assert_eq!(p.len(), 2);
    }

    #[test]
    fn replace_smaller() {
        let p = Pattern::<u8>::from_str("0x?a").unwrap();
        let source = vec![0x12, 0x1b];
        let result = p.replace(source);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), [0x1a, 0x1b]);
    }

    #[test]
    fn replace_bigger() {
        let p = Pattern::<u8>::from_str("0x?a 0x2? 0x3c 0x4d").unwrap();
        let source = vec![0x12, 0x1b];
        let result = p.replace(source);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), [0x1a, 0x2b, 0x3c, 0x4d]);
    }

    #[test]
    fn replace_undefined() {
        let p = Pattern::<u8>::from_str("0x?a 0x2? 0x3?").unwrap();
        let source = vec![0x12, 0x1b];
        let result = p.replace(source);
        assert!(result.is_err());
    }
}
