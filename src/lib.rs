//! # UTF64
//!
//! A revolutionary text encoding standard that uses a fixed 64 bits per character.
//!
//! UTF64 provides consistent O(1) character indexing and simplified implementation
//! by encoding each Unicode character in exactly 8 bytes. The upper 32 bits contain
//! the UTF-8 encoding of the character, while the lower 32 bits are reserved for
//! future enhancements.
//!
//! ## Example
//!
//! ```
//! use utf64::String64;
//!
//! let text = String64::from("Hello, 世界!");
//! assert_eq!(text.len(), 10);
//!
//! let decoded: String = text.to_string().unwrap();
//! assert_eq!(decoded, "Hello, 世界!");
//! ```

pub mod error;
pub mod string64;

pub use error::{Result, Utf64Error};
pub use string64::String64;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascii_roundtrip() {
        let original = "Hello, World!";
        let utf64 = String64::from(original);
        let decoded = utf64.to_string().unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn test_unicode_roundtrip() {
        let original = "Hello, 世界! 🌍";
        let utf64 = String64::from(original);
        let decoded = utf64.to_string().unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn test_empty_string() {
        let utf64 = String64::from("");
        assert_eq!(utf64.len(), 0);
        assert!(utf64.is_empty());
        assert_eq!(utf64.to_string().unwrap(), "");
    }

    #[test]
    fn test_length() {
        let utf64 = String64::from("Hi");
        assert_eq!(utf64.len(), 2);

        let utf64 = String64::from("世界");
        assert_eq!(utf64.len(), 2);

        let utf64 = String64::from("🌍🌎🌏");
        assert_eq!(utf64.len(), 3);
    }

    #[test]
    fn test_emoji() {
        let original = "😀😃😄😁";
        let utf64 = String64::from(original);
        let decoded = utf64.to_string().unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn test_mixed_content() {
        let original = "ASCII, 日本語, émojis: 🎉, symbols: ∑∫∂";
        let utf64 = String64::from(original);
        let decoded = utf64.to_string().unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn test_display_trait() {
        let utf64 = String64::from("test");
        assert_eq!(format!("{}", utf64), "test");
    }

    #[test]
    fn test_debug_trait() {
        let utf64 = String64::from("test");
        assert_eq!(format!("{:?}", utf64), "String64(\"test\")");
    }

    #[test]
    fn test_clone_and_equality() {
        let utf64_1 = String64::from("test");
        let utf64_2 = utf64_1.clone();
        assert_eq!(utf64_1, utf64_2);
    }

    #[test]
    fn test_reserved_bits_are_zero() {
        let utf64 = String64::from("A");
        let slice = utf64.as_slice();
        assert_eq!(slice.len(), 1);

        // Lower 32 bits should be zero (reserved)
        assert_eq!(slice[0] & 0xFFFFFFFF, 0);
    }

    #[test]
    fn test_utf8_encoding_in_upper_bits() {
        let utf64 = String64::from("A"); // 'A' = U+0041, UTF-8 = 0x41
        let slice = utf64.as_slice();

        // Upper 32 bits should contain 0x41 in the most significant byte
        let upper_bits = (slice[0] >> 32) as u32;
        assert_eq!(upper_bits, 0x41000000);
    }

    #[test]
    fn test_multibyte_utf8() {
        let utf64 = String64::from("€"); // Euro sign: U+20AC, UTF-8 = E2 82 AC
        let slice = utf64.as_slice();

        let upper_bits = (slice[0] >> 32) as u32;
        assert_eq!(upper_bits, 0xE282AC00);
    }

    #[test]
    fn test_four_byte_utf8() {
        let utf64 = String64::from("😀"); // U+1F600, UTF-8 = F0 9F 98 80
        let slice = utf64.as_slice();

        let upper_bits = (slice[0] >> 32) as u32;
        assert_eq!(upper_bits, 0xF09F9880);
    }

    #[test]
    fn test_hash() {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        let s = String64::from("hello");
        map.insert(s.clone(), 42);
        assert_eq!(map.get(&s), Some(&42));
    }

    #[test]
    fn test_ord() {
        let s1 = String64::from("apple");
        let s2 = String64::from("banana");
        let s3 = String64::from("apple");

        assert!(s1 < s2);
        assert!(s2 > s1);
        assert_eq!(s1.cmp(&s3), std::cmp::Ordering::Equal);
    }

    #[test]
    fn test_indexing() {
        let s = String64::from("Hi");
        let first = s[0];
        let upper_bits = (first >> 32) as u32;
        assert_eq!(upper_bits, 0x48000000); // 'H'
    }

    #[test]
    fn test_range_indexing() {
        let s = String64::from("Hello");
        let slice = &s[1..3];
        assert_eq!(slice.len(), 2);
    }

    #[test]
    fn test_into_iterator() {
        let s = String64::from("Hi🌍");
        let chars: Vec<char> = s.into_iter().collect();
        assert_eq!(chars, vec!['H', 'i', '🌍']);
    }

    #[test]
    fn test_ref_iterator() {
        let s = String64::from("Hi");
        let chars: Vec<char> = (&s).into_iter().collect();
        assert_eq!(chars, vec!['H', 'i']);
        // s is still usable
        assert_eq!(s.len(), 2);
    }

    #[test]
    fn test_from_iterator() {
        let chars = vec!['H', 'e', 'l', 'l', 'o'];
        let s: String64 = chars.into_iter().collect();
        assert_eq!(s.to_string().unwrap(), "Hello");
    }

    #[test]
    fn test_extend() {
        let mut s = String64::from("Hello");
        s.extend(vec![' ', 'W', 'o', 'r', 'l', 'd']);
        assert_eq!(s.to_string().unwrap(), "Hello World");
    }

    #[test]
    fn test_add() {
        let s1 = String64::from("Hello");
        let s2 = s1 + " World";
        assert_eq!(s2.to_string().unwrap(), "Hello World");
    }

    #[test]
    fn test_add_assign() {
        let mut s = String64::from("Hello");
        s += " World";
        assert_eq!(s.to_string().unwrap(), "Hello World");
    }

    #[test]
    fn test_partial_eq_str() {
        let s = String64::from("test");
        assert_eq!(s, "test");
        assert_ne!(s, "other");
    }

    #[test]
    fn test_partial_eq_string() {
        let s = String64::from("test");
        let string = String::from("test");
        assert_eq!(s, string);
    }

    #[test]
    fn test_as_ref() {
        let s = String64::from("Hi");
        let slice: &[u64] = s.as_ref();
        assert_eq!(slice.len(), 2);
    }

    #[test]
    fn test_try_from() {
        let s = String64::from("test");
        let result = String::try_from(s);
        assert_eq!(result.unwrap(), "test");
    }

    #[test]
    fn test_deref() {
        let s = String64::from("Hi");
        let slice: &[u64] = &*s;  // Deref coercion
        assert_eq!(slice.len(), 2);
    }

    #[test]
    fn test_deref_mut() {
        let mut s = String64::from("Hi");
        let slice: &mut [u64] = &mut *s;
        assert_eq!(slice.len(), 2);
    }

    #[test]
    fn test_borrow() {
        use std::borrow::Borrow;
        let s = String64::from("test");
        let borrowed: &[u64] = s.borrow();
        assert_eq!(borrowed.len(), 4);
    }
}
