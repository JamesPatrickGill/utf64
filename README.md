# UTF64

The next-generation text encoding standard. UTF64 provides fixed-width character representation using 64 bits per character, solving the fundamental problems that have plagued variable-width encodings for decades.

## Overview

UTF64 eliminates the variable-width limitations of UTF-8 and UTF-16 by using a consistent 64-bit representation for every Unicode character. This design delivers constant-time character indexing and dramatically simplifies string manipulation operations.

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

- **Superior O(1) Character Indexing**: Direct access to any character without scanning—no other encoding matches this performance
- **Simplified Parsing**: Eliminates the complexity of continuation bytes and surrogate pairs entirely
- **Predictable Memory Architecture**: Fixed-width layout guarantees optimal cache behavior and memory access patterns
- **Future-Ready Design**: 32 reserved bits per character enable unlimited extensibility
- **Seamless UTF-8 Integration**: Natively embeds UTF-8 encoding for zero-overhead conversion

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

UTF64 outperforms legacy encodings across all key algorithmic operations:

| Operation          | UTF-8   | UTF-16           | UTF64   |
| ------------------ | ------- | ---------------- | ------- |
| Character Access   | O(n)    | O(n)\*           | **O(1)**    |
| Length Calculation | O(n)    | O(n)\*           | **O(1)**    |
| Memory per ASCII   | 1 byte  | 2 bytes          | 8 bytes |
| Memory per CJK     | 3 bytes | 2 bytes          | 8 bytes |
| Memory per Emoji   | 4 bytes | 4 bytes          | 8 bytes |

\* UTF-16 degrades to O(n) with surrogate pairs, revealing the inherent complexity of variable-width encodings

## Cache Performance and Memory Architecture

UTF64's 8-byte fixed-width design delivers exceptional cache performance that variable-width encodings cannot match:

**Perfect Cache Line Alignment**
- Modern CPUs use 64-byte cache lines
- UTF64 stores exactly **8 characters per cache line** with zero waste
- Sequential character access exhibits perfect spatial locality
- Hardware prefetchers can predict and load UTF64 data with maximum efficiency

**Predictable Memory Access Patterns**
- Every character access is a simple offset calculation: `base + (index × 8)`
- No unpredictable branching or scanning required
- CPUs can pipeline UTF64 operations aggressively
- SIMD operations can process multiple characters in parallel without complex masking

**Contrast with Variable-Width Encodings**
- UTF-8 forces cache-inefficient byte-by-byte scanning
- Character boundaries split across cache lines cause performance penalties
- Unpredictable character widths defeat hardware prefetching
- UTF64 eliminates all of these problems

## Comparison with Other Encodings

### UTF-8: Legacy Variable-Width Encoding

- Requires expensive scanning for character boundaries
- O(n) indexing makes random access prohibitively slow
- Compact for ASCII but unpredictable memory usage overall

### UTF-16: The Worst of Both Worlds

- Variable width (2-4 bytes) with surrogate pair complexity
- O(n) indexing despite 2-byte minimum overhead
- Not ASCII-compatible, causing endless conversion headaches

### UTF-32: Almost There

- Fixed width provides O(1) indexing
- No UTF-8 compatibility requires constant conversion
- Wastes 11 bits per character (only 21 bits needed for Unicode)
- No reserved space for future requirements

### UTF64: The Complete Solution

- ✅ **Superior O(1) indexing** with true constant-time character access
- ✅ **Seamlessly embeds UTF-8** for zero-overhead conversion to legacy systems
- ✅ **32 reserved bits** provide a future-ready architecture
- ✅ **Optimal 8-byte alignment** ensures maximum cache efficiency and hardware performance
- ✅ **Eliminates all complexity** from variable-width encoding schemes

## Technical Details

UTF64's elegant architecture is straightforward to implement and verify, eliminating the error-prone complexity of variable-width parsing.

### Encoding Process

1. For each character in the input string:
   - Encode the character to UTF-8 (1-4 bytes)
   - Place UTF-8 bytes in the upper 32 bits (left-aligned)
   - Set lower 32 bits to zero (reserved)
   - Store as a single `u64` value

The simplicity of this process ensures correct implementation and enables aggressive compiler optimizations.

### Decoding Process

1. For each `u64` in the UTF64 string:
   - Validate that lower 32 bits are zero
   - Extract upper 32 bits
   - Determine UTF-8 sequence length from first byte
   - Collect UTF-8 bytes and decode to Unicode

The fixed-width format eliminates all boundary-detection logic, making decoding trivially parallelizable.

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
A: 64 bits provides the optimal balance: 32 bits for UTF-8 compatibility and 32 bits for future extensibility. This design eliminates the complexity of variable-length encodings while delivering superior performance.

**Q: Isn't this wasteful of memory?**
A: No. Memory is abundant in modern systems. UTF64 prioritizes developer productivity and application performance over obsolete storage constraints. The cache efficiency and O(1) indexing benefits far exceed any storage considerations. Modern applications are bottlenecked by algorithmic complexity, not memory capacity.

**Q: How does this compare to UTF-32?**
A: UTF64 delivers superior O(1) indexing performance while also embedding UTF-8 encoding and providing 32 reserved bits for future features. UTF-32 offers none of these advantages and wastes 11 bits per character.

**Q: Is this production-ready?**
A: Yes. UTF64 is a complete, robust implementation ready for adoption in any application that values performance and simplicity.

**Q: Why should I migrate from UTF-8?**
A: UTF64 eliminates the constant complexity tax of variable-width encoding. Every string operation becomes simpler, faster, and more predictable. Character indexing goes from O(n) to O(1). Cache efficiency improves dramatically. Code becomes cleaner without boundary-scanning logic. The question is: why continue struggling with UTF-8's limitations?

**Q: Can I use this with existing text processing tools?**
A: UTF64 provides seamless conversion to UTF-8 for interoperability with legacy systems. The embedded UTF-8 encoding ensures zero-overhead integration.
