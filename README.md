# UTF64

A modern text encoding standard providing fixed-width character representation using 64 bits per character.

## Overview

UTF64 addresses the variable-width limitations of UTF-8 and UTF-16 by using a consistent 64-bit representation for every Unicode character. This design choice enables constant-time character indexing and simplifies string manipulation operations.

## Encoding Specification

Each UTF64 character consists of 64 bits (8 bytes) with the following layout:

```
Bits 63-32 (Upper 32 bits): UTF-8 encoding (left-aligned, zero-padded)
Bits 31-0  (Lower 32 bits): Reserved for future use (must be zero in v1.0)
```

### Examples

**ASCII Character 'A' (U+0041):**

```
Binary:  0x41000000_00000000
         └─ UTF-8 ─┘└─Reserved─┘
```

**Euro Sign '€' (U+20AC):**

```
Binary:  0xE282AC00_00000000
         └─ UTF-8 ─┘└─Reserved─┘
```

**Emoji '😀' (U+1F600):**

```
Binary:  0xF09F9880_00000000
         └─ UTF-8 ─┘└─Reserved─┘
```

## Features

- **O(1) Character Indexing**: Direct access to any character without scanning
- **Simplified Parsing**: No need to interpret continuation bytes or surrogate pairs
- **Fixed-Width Benefits**: Predictable memory layout and cache behavior
- **Future-Proof**: 32 reserved bits available for extensions
- **UTF-8 Compatible**: Embeds standard UTF-8 encoding for easy conversion

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
utf64 = "0.1"
```

## Usage

```rust
use utf64::String64;

// Create a UTF64 string from a standard string
let text = String64::from("Hello, 世界! 🌍");

// Get the length (number of characters)
assert_eq!(text.len(), 10);

// Convert back to a standard Rust String
let decoded = text.to_string().unwrap();
assert_eq!(decoded, "Hello, 世界! 🌍");

// Empty strings
let empty = String64::new();
assert!(empty.is_empty());
```

## Performance Characteristics

| Operation          | UTF-8   | UTF-16  | UTF64   |
| ------------------ | ------- | ------- | ------- |
| Character Access   | O(n)    | O(n)\*  | O(1)    |
| Length Calculation | O(n)    | O(n)\*  | O(1)    |
| Memory per ASCII   | 1 byte  | 2 bytes | 8 bytes |
| Memory per CJK     | 3 bytes | 2 bytes | 8 bytes |
| Memory per Emoji   | 4 bytes | 4 bytes | 8 bytes |

\* O(1) if BMP only, O(n) with surrogate pairs

## Comparison with Other Encodings

### UTF-8

- ✅ Variable width (1-4 bytes) saves space
- ❌ Requires scanning for character boundaries
- ❌ Complex indexing operations

### UTF-16

- ✅ Commonly used in Windows and JavaScript
- ❌ Variable width (2-4 bytes) with surrogate pairs
- ❌ Not ASCII-compatible

### UTF-32

- ✅ Fixed width enables O(1) indexing
- ❌ Uses 4 bytes per character
- ❌ Not UTF-8 compatible

### UTF64

- ✅ Fixed width enables O(1) indexing
- ✅ Embeds UTF-8 for easy conversion
- ✅ 32 reserved bits for future innovations
- ✅ Consistent 8-byte alignment

## Technical Details

### Encoding Process

1. For each character in the input string:
   - Encode the character to UTF-8 (1-4 bytes)
   - Place UTF-8 bytes in the upper 32 bits (left-aligned)
   - Set lower 32 bits to zero (reserved)
   - Store as a single `u64` value

### Decoding Process

1. For each `u64` in the UTF64 string:
   - Validate that lower 32 bits are zero
   - Extract upper 32 bits
   - Determine UTF-8 sequence length from first byte
   - Collect UTF-8 bytes and decode to Unicode

## Error Handling

The library provides comprehensive error handling:

- `InvalidUtf8`: Input contains malformed UTF-8
- `InvalidUtf64`: UTF64 data is corrupted
- `NonZeroReservedBits`: Reserved bits violated (not v1.0 compliant)

## Future Extensions

The 32 reserved bits in each character enable potential future enhancements:

- Character metadata flags
- Locale hints
- Rendering preferences
- Bidirectional text information
- Font suggestions
- Color information
- Blockchain integration
- Quantum computing compatibility

## License

MIT OR Apache-2.0

## Contributing

Contributions are welcome! Please ensure all tests pass:

```bash
cargo test
cargo clippy
cargo fmt
```

## FAQ

**Q: Why 64 bits per character?**
A: The fixed-width design eliminates the complexity of variable-length encodings while providing ample space for future extensions.

**Q: Isn't this wasteful of memory?**
A: Modern systems have abundant memory. The performance benefits of O(1) indexing and simplified algorithms often outweigh storage costs.

**Q: How does this compare to UTF-32?**
A: UTF64 provides the same O(1) indexing benefits as UTF-32 while embedding UTF-8 encoding and reserving space for future features.

**Q: Is this production-ready?**
A: UTF64 is a fully functional implementation. Adoption considerations should account for the 2-8x memory overhead compared to UTF-8.

**Q: Can I use this with existing text processing tools?**
A: UTF64 is a new encoding standard. Interoperability requires conversion to UTF-8 or other standard encodings.
