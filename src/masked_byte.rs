use crate::ParseError;
use snafu::{ensure, ResultExt, Snafu};
use std::convert::From;
use std::fmt;
use std::ops;
use std::str;

#[derive(PartialEq)]
pub struct MaskedByte<T> {
    // No assumptions are made for bits where according mask bit is 0.
    value: T,

    // mask bit is 0 if value is undefined
    mask: T,
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Number is too big to fit into type"))]
    NumberTooBig,

    #[snafu(display("Invalid char {}: {}", char, source))]
    InvalidChar {
        char: char,
        source: std::num::ParseIntError,
    },

    #[snafu(display("No or unknown prefix"))]
    UnknownPrefix,
}

fn bits<T>() -> u32
where
    T: num::PrimInt,
{
    T::zero().count_zeros()
}

impl<T> fmt::Debug for MaskedByte<T>
where
    T: fmt::Binary + num::PrimInt,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "MaskedByte {{ value: {1:#00$b}, mask: {2:#00$b} }}",
            (bits::<T>() + 2) as usize,
            self.value,
            self.mask
        )
    }
}

impl<T> fmt::Display for MaskedByte<T>
where
    T: num::PrimInt + ops::ShrAssign<u8>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut string = String::new();
        let mut value = self.value;
        let mut mask = self.mask;

        for _ in 0..bits::<T>() {
            let char = match ((value & T::one()).is_zero(), (mask & T::one()).is_zero()) {
                (_, true) => '?',
                (true, false) => '0',
                (false, false) => '1',
            };

            string.push(char);

            mask >>= 1_u8;
            value >>= 1_u8;
        }

        write!(f, "{}", string.chars().rev().collect::<String>())
    }
}

impl<T> MaskedByte<T>
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
    /// Returns new MaskedByte
    pub fn new(value: T, mask: T) -> Self {
        Self { value, mask }
    }

    /// Matches data against MaskedByte.
    /// Where mask bit is 1 the bit of value has to fit data, mask bit 0 means
    /// data bit can be anything. Returns true if masked bits match.
    pub fn matches(&self, data: T) -> bool {
        data & self.mask == self.value & self.mask
    }

    /// Sets data bits to value bits where masked bits are 1
    pub fn set(&self, data: T) -> T {
        (data & !self.mask) | (self.value & self.mask)
    }

    /// Returns value if mask is fully defined
    pub fn defined(&self) -> Option<T> {
        if self.mask == T::max_value() {
            Some(self.value)
        } else {
            None
        }
    }

    fn from_string_without_header(string: &str, bits: u32) -> Result<Self, Error> {
        let mut value = T::min_value();
        let mut mask = T::max_value();

        // Using the smallest type possible so it can be converted to bigger ones
        let radix: u8 = 2_u8.pow(bits);
        let part_mask: T = (radix - 1).into(); //From::<u8>::from(radix - 1);

        for c in string.chars() {
            // Overflow checking is extensive because octal numbers bitcount doesn't
            // exactly fit primitive types bitcounts and string can have leading zeros.
            // Values are first rotated shifting the values to test to the front,
            // after test bits are cleared.

            // Is essentially
            // value <<= bits;
            // with overflow checking.
            // value is initialized with 0s. If a 1 is shifted out the value is too big.
            value = value.rotate_left(bits);
            ensure!(value & part_mask == Default::default(), NumberTooBig);
            value &= !part_mask;

            // Is essentially
            // mask <<= bits;
            // with overflow checking.
            // mask is initialized with 1s to account for leading 0s in value. If a 0 is
            // shifted out the value is too big to fit.
            mask = mask.rotate_left(bits);
            ensure!(mask & part_mask == part_mask, NumberTooBig);
            mask &= !part_mask;

            match c {
                '?' => {}
                h => {
                    let part = T::from_str_radix(&h.to_string(), radix as u32)
                        .context(InvalidChar { char: h })?;
                    value |= part;
                    mask |= part_mask;
                }
            }
        }

        Ok(Self::new(value, mask))
    }
}

impl<T> str::FromStr for MaskedByte<T>
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

    /// Creates MaskedByte from string representation. String needs to be preceded
    /// with a base prefix. Allowed are "0b" for binary, "0x" for hexadecimal and
    /// "0o" for octal number representation.
    /// The "?" char stands for undefined bits.
    /// Examples:
    /// 0x?5 equals 0b????0101 which represents a value of 0b00000101 with a mask
    /// of 0b00001111.
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let bits = match &string[..2] {
            "0b" => 1,
            "0x" => 4,
            "0o" => 3,
            _ => {
                return Err(Error::UnknownPrefix).context(ParseError {
                    string: string.to_string(),
                })
            }
        };
        Self::from_string_without_header(&string[2..], bits).context(ParseError {
            string: string.to_string(),
        })
    }
}

impl<T> Default for MaskedByte<T>
where
    T: Default,
{
    // default does nothing
    fn default() -> Self {
        Self {
            value: Default::default(),
            mask: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn matches() {
        let byte = MaskedByte::<u8>::new(0b10101010, 0b00001111);
        assert!(byte.matches(0b10011010));
    }

    #[test]
    fn sets() {
        let byte = MaskedByte::<u8>::new(0b10101010, 0b00001111);
        assert_eq!(byte.set(0b10011101), 0b10011010);
    }

    #[test]
    fn string_conversion_from_binary() {
        let b = MaskedByte::from_str("0b?01?");
        assert!(b.is_ok());
        assert_eq!(b.unwrap(), MaskedByte::<u8>::new(0b00000010, 0b11110110));
    }

    #[test]
    fn string_conversion_from_hex() {
        let h = MaskedByte::from_str("0x?a");
        assert!(h.is_ok());
        assert_eq!(h.unwrap(), MaskedByte::<u8>::new(0x0a, 0x0f));
    }

    #[test]
    fn string_conversion_from_octal() {
        let h = MaskedByte::from_str("0o3?7");
        assert!(h.is_ok());
        assert_eq!(h.unwrap(), MaskedByte::<u8>::new(0o307, 0o307));
    }

    #[test]
    fn leading_zeroes() {
        let h = MaskedByte::<u8>::from_str("0x0000ff");
        assert!(h.is_ok());
        assert_eq!(h.unwrap(), MaskedByte::<u8>::new(0xff, 0xff));
    }

    #[test]
    fn too_big() {
        let h = MaskedByte::<u8>::from_str("0o777");
        assert!(h.is_err());
    }

    #[test]
    fn bitcount() {
        assert_eq!(bits::<i8>(), 8);
        assert_eq!(bits::<u32>(), 32);
    }
}
