use crate::error::{Result, Utf64Error};
use std::{
    fmt,
    hash::{Hash, Hasher},
    iter::{Extend, FromIterator},
    ops::{Add, AddAssign, Index, Range, RangeFrom, RangeFull, RangeTo},
    str::FromStr,
};

/// A UTF64-encoded string.
///
/// UTF64 is a fixed-width encoding where each character occupies exactly 64 bits (8 bytes).
/// The upper 32 bits contain the UTF-8 encoding of the character (left-aligned, zero-padded),
/// while the lower 32 bits are reserved for future use and must be zero in v1.0.
///
/// # Examples
///
/// ```
/// use utf64::String64;
///
/// let s = String64::from("Hello, 世界!");
/// assert_eq!(s.len(), 10); // 10 characters
/// ```
#[derive(Clone, PartialEq, Eq)]
pub struct String64 {
    data: Vec<u64>,
}

impl String64 {
    /// Creates a new empty `String64`.
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    /// Creates a new `String64` with the specified capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
        }
    }

    /// Returns the length of this `String64` in characters.
    ///
    /// Note: This is O(1) as each character is exactly one u64.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns `true` if this `String64` has a length of zero.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns a slice of the underlying u64 data.
    pub fn as_slice(&self) -> &[u64] {
        &self.data
    }

    /// Encodes a string slice into UTF64 format.
    fn encode(s: &str) -> Result<Self> {
        let mut data = Vec::with_capacity(s.chars().count());

        for ch in s.chars() {
            let mut utf8_buf = [0u8; 4];
            let utf8_bytes = ch.encode_utf8(&mut utf8_buf).as_bytes();

            // Pack UTF-8 bytes into upper 32 bits (big-endian style)
            let mut upper_bits: u32 = 0;
            for (i, &byte) in utf8_bytes.iter().enumerate() {
                upper_bits |= (byte as u32) << (24 - (i * 8));
            }

            // Upper 32 bits = UTF-8, Lower 32 bits = reserved (0)
            let utf64_char = (upper_bits as u64) << 32;
            data.push(utf64_char);
        }

        Ok(Self { data })
    }

    /// Decodes this UTF64 string back to a standard Rust String.
    pub fn to_string(&self) -> Result<String> {
        let mut utf8_bytes = Vec::new();

        for &utf64_char in &self.data {
            // Check that reserved bits (lower 32) are zero
            if (utf64_char & 0xFFFFFFFF) != 0 {
                return Err(Utf64Error::NonZeroReservedBits);
            }

            // Extract upper 32 bits
            let upper_bits = (utf64_char >> 32) as u32;

            // Extract UTF-8 bytes (up to 4 bytes)
            let bytes = [
                ((upper_bits >> 24) & 0xFF) as u8,
                ((upper_bits >> 16) & 0xFF) as u8,
                ((upper_bits >> 8) & 0xFF) as u8,
                (upper_bits & 0xFF) as u8,
            ];

            // Find the actual length of the UTF-8 sequence
            // UTF-8 first byte tells us the length
            let len = if bytes[0] == 0 {
                return Err(Utf64Error::InvalidUtf64);
            } else if bytes[0] < 0x80 {
                1
            } else if bytes[0] < 0xE0 {
                2
            } else if bytes[0] < 0xF0 {
                3
            } else {
                4
            };

            utf8_bytes.extend_from_slice(&bytes[..len]);
        }

        String::from_utf8(utf8_bytes).map_err(|_| Utf64Error::InvalidUtf8)
    }
}

impl Default for String64 {
    fn default() -> Self {
        Self::new()
    }
}

impl From<&str> for String64 {
    fn from(s: &str) -> Self {
        Self::encode(s).expect("valid UTF-8 &str should always encode to UTF64")
    }
}

impl From<String> for String64 {
    fn from(s: String) -> Self {
        Self::encode(&s).expect("valid UTF-8 String should always encode to UTF64")
    }
}

impl FromStr for String64 {
    type Err = Utf64Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::encode(s)
    }
}

impl fmt::Display for String64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.to_string() {
            Ok(s) => write!(f, "{s}"),
            Err(_) => write!(f, "<invalid UTF64>"),
        }
    }
}

impl fmt::Debug for String64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.to_string() {
            Ok(s) => write!(f, "String64({s:?})"),
            Err(_) => write!(f, "String64(<invalid>)"),
        }
    }
}

impl Hash for String64 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.data.hash(state);
    }
}

impl PartialOrd for String64 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for String64 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Lexicographic comparison by decoding to strings
        match (self.to_string(), other.to_string()) {
            (Ok(s1), Ok(s2)) => s1.cmp(&s2),
            (Ok(_), Err(_)) => std::cmp::Ordering::Greater,
            (Err(_), Ok(_)) => std::cmp::Ordering::Less,
            (Err(_), Err(_)) => std::cmp::Ordering::Equal,
        }
    }
}

impl Index<usize> for String64 {
    type Output = u64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl Index<Range<usize>> for String64 {
    type Output = [u64];

    fn index(&self, range: Range<usize>) -> &Self::Output {
        &self.data[range]
    }
}

impl Index<RangeFrom<usize>> for String64 {
    type Output = [u64];

    fn index(&self, range: RangeFrom<usize>) -> &Self::Output {
        &self.data[range]
    }
}

impl Index<RangeTo<usize>> for String64 {
    type Output = [u64];

    fn index(&self, range: RangeTo<usize>) -> &Self::Output {
        &self.data[range]
    }
}

impl Index<RangeFull> for String64 {
    type Output = [u64];

    fn index(&self, range: RangeFull) -> &Self::Output {
        &self.data[range]
    }
}

/// Iterator that yields characters from a String64 by consuming it.
pub struct IntoIter {
    data: std::vec::IntoIter<u64>,
}

impl Iterator for IntoIter {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.data.next().map(|utf64_char| {
            // Extract upper 32 bits and decode the UTF-8
            let upper_bits = (utf64_char >> 32) as u32;
            let bytes = [
                ((upper_bits >> 24) & 0xFF) as u8,
                ((upper_bits >> 16) & 0xFF) as u8,
                ((upper_bits >> 8) & 0xFF) as u8,
                (upper_bits & 0xFF) as u8,
            ];

            // Determine UTF-8 length and decode
            let len = if bytes[0] < 0x80 {
                1
            } else if bytes[0] < 0xE0 {
                2
            } else if bytes[0] < 0xF0 {
                3
            } else {
                4
            };

            std::str::from_utf8(&bytes[..len])
                .ok()
                .and_then(|s| s.chars().next())
                .expect("valid UTF64 should decode to valid char")
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.data.size_hint()
    }
}

impl ExactSizeIterator for IntoIter {
    fn len(&self) -> usize {
        self.data.len()
    }
}

impl IntoIterator for String64 {
    type Item = char;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            data: self.data.into_iter(),
        }
    }
}

/// Iterator that yields characters from a &String64 without consuming it.
pub struct Iter<'a> {
    data: std::slice::Iter<'a, u64>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.data.next().map(|&utf64_char| {
            // Extract upper 32 bits and decode the UTF-8
            let upper_bits = (utf64_char >> 32) as u32;
            let bytes = [
                ((upper_bits >> 24) & 0xFF) as u8,
                ((upper_bits >> 16) & 0xFF) as u8,
                ((upper_bits >> 8) & 0xFF) as u8,
                (upper_bits & 0xFF) as u8,
            ];

            // Determine UTF-8 length and decode
            let len = if bytes[0] < 0x80 {
                1
            } else if bytes[0] < 0xE0 {
                2
            } else if bytes[0] < 0xF0 {
                3
            } else {
                4
            };

            std::str::from_utf8(&bytes[..len])
                .ok()
                .and_then(|s| s.chars().next())
                .expect("valid UTF64 should decode to valid char")
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.data.size_hint()
    }
}

impl<'a> ExactSizeIterator for Iter<'a> {
    fn len(&self) -> usize {
        self.data.len()
    }
}

impl<'a> IntoIterator for &'a String64 {
    type Item = char;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            data: self.data.iter(),
        }
    }
}

impl FromIterator<char> for String64 {
    fn from_iter<T: IntoIterator<Item = char>>(iter: T) -> Self {
        let mut s = String64::new();
        s.extend(iter);
        s
    }
}

impl Extend<char> for String64 {
    fn extend<T: IntoIterator<Item = char>>(&mut self, iter: T) {
        for ch in iter {
            let mut utf8_buf = [0u8; 4];
            let utf8_bytes = ch.encode_utf8(&mut utf8_buf).as_bytes();

            // Pack UTF-8 bytes into upper 32 bits
            let mut upper_bits: u32 = 0;
            for (i, &byte) in utf8_bytes.iter().enumerate() {
                upper_bits |= (byte as u32) << (24 - (i * 8));
            }

            let utf64_char = (upper_bits as u64) << 32;
            self.data.push(utf64_char);
        }
    }
}

impl Add<&str> for String64 {
    type Output = String64;

    fn add(mut self, rhs: &str) -> Self::Output {
        self.extend(rhs.chars());
        self
    }
}

impl AddAssign<&str> for String64 {
    fn add_assign(&mut self, rhs: &str) {
        self.extend(rhs.chars());
    }
}

impl PartialEq<str> for String64 {
    fn eq(&self, other: &str) -> bool {
        match self.to_string() {
            Ok(s) => s == other,
            Err(_) => false,
        }
    }
}

impl PartialEq<&str> for String64 {
    fn eq(&self, other: &&str) -> bool {
        self.eq(*other)
    }
}

impl PartialEq<String> for String64 {
    fn eq(&self, other: &String) -> bool {
        self.eq(other.as_str())
    }
}

impl AsRef<[u64]> for String64 {
    fn as_ref(&self) -> &[u64] {
        &self.data
    }
}

impl TryFrom<String64> for String {
    type Error = Utf64Error;

    fn try_from(value: String64) -> Result<Self> {
        value.to_string()
    }
}

impl TryFrom<&String64> for String {
    type Error = Utf64Error;

    fn try_from(value: &String64) -> Result<Self> {
        value.to_string()
    }
}
