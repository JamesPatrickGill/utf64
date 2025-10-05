# UTF64 Character Encoding Specification

## Version 1.0

**Status:** Standard
**Date:** 2025-01-10
**Authors:** UTF64 Working Group
**Category:** Character Encoding Standard

---

## Abstract

This document defines UTF64, a fixed-width character encoding scheme that represents each Unicode code point using exactly 64 bits (8 octets). UTF64 addresses the computational complexity of variable-width encodings while maintaining compatibility with existing UTF-8 infrastructure through embedded encoding. This specification defines the encoding format, validation requirements, error handling, and extension mechanisms for future versions.

---

## Table of Contents

1. [Introduction](#1-introduction)
2. [Terminology](#2-terminology)
3. [Encoding Format](#3-encoding-format)
4. [Encoding Process](#4-encoding-process)
5. [Decoding Process](#5-decoding-process)
6. [Validation Requirements](#6-validation-requirements)
7. [Error Handling](#7-error-handling)
8. [Byte Order and Serialization](#8-byte-order-and-serialization)
9. [Security Considerations](#9-security-considerations)
10. [Version Compatibility](#10-version-compatibility)
11. [Implementation Requirements](#11-implementation-requirements)
12. [Appendix A: Examples](#appendix-a-examples)
13. [Appendix B: Comparison with Other Encodings](#appendix-b-comparison-with-other-encodings)
14. [References](#references)

---

## 1. Introduction

### 1.1 Motivation

Variable-width character encodings (UTF-8, UTF-16) impose significant computational overhead for common string operations:

- **Character indexing** requires O(n) time complexity due to boundary scanning
- **Length calculation** necessitates full string traversal
- **Cache performance** suffers from unpredictable memory access patterns
- **Implementation complexity** increases error surface area

UTF64 eliminates these issues through a fixed-width design that enables O(1) indexing while embedding UTF-8 encoding for seamless interoperability.

### 1.2 Design Goals

1. **Constant-time character access**: Enable O(1) indexing without boundary detection
2. **UTF-8 compatibility**: Embed standard UTF-8 encoding for zero-overhead conversion
3. **Future extensibility**: Reserve space for backward-compatible enhancements
4. **Cache efficiency**: Align to hardware cache line boundaries
5. **Implementation simplicity**: Minimize error-prone parsing logic

### 1.3 Scope

This specification defines:

- Binary representation format for UTF64-encoded text
- Encoding and decoding algorithms
- Validation and error detection mechanisms
- Version compatibility guarantees

This specification does NOT define:

- File format or container specifications
- Network transmission protocols
- Compression schemes
- Grapheme cluster or normalization handling

---

## 2. Terminology

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL" in this document are to be interpreted as described in RFC 2119.

**Character**: A single Unicode code point (U+0000 to U+10FFFF)

**Code Point**: A numerical value in the Unicode codespace

**Octet**: An 8-bit byte

**UTF64 Character Unit**: A single 64-bit (8-octet) encoded character value

**Reserved Bits**: Bits allocated for future specification versions, MUST be zero in v1.0

**Upper Half**: Bits 63-32 of a UTF64 character unit

**Lower Half**: Bits 31-0 of a UTF64 character unit

---

## 3. Encoding Format

### 3.1 Character Unit Structure

Each UTF64 character unit consists of 64 bits organized as follows:

```
 Bit Position:  63                    32 31                     0
                ├───────────────────────┼───────────────────────┤
                │    Upper Half (U)     │    Lower Half (L)     │
                │    UTF-8 Encoding     │   Reserved Bits       │
                ├───────────────────────┼───────────────────────┤
Octet Index:    7   6   5   4           3   2   1   0
```

**Figure 3.1**: UTF64 Character Unit Structure (Big-Endian Representation)

### 3.2 Upper Half (Bits 63-32)

The upper 32 bits contain the UTF-8 encoding of the Unicode code point, left-aligned and zero-padded:

```
Bits 63-56:  First UTF-8 octet (REQUIRED)
Bits 55-48:  Second UTF-8 octet (if UTF-8 length ≥ 2, else 0x00)
Bits 47-40:  Third UTF-8 octet (if UTF-8 length ≥ 3, else 0x00)
Bits 39-32:  Fourth UTF-8 octet (if UTF-8 length = 4, else 0x00)
```

UTF-8 encoding follows RFC 3629 with the following constraints:

- **1-byte sequence** (U+0000 to U+007F): `0xxxxxxx 00 00 00`
- **2-byte sequence** (U+0080 to U+07FF): `110xxxxx 10xxxxxx 00 00`
- **3-byte sequence** (U+0800 to U+FFFF): `1110xxxx 10xxxxxx 10xxxxxx 00`
- **4-byte sequence** (U+10000 to U+10FFFF): `11110xxx 10xxxxxx 10xxxxxx 10xxxxxx`

Overlong encodings, surrogate pairs (U+D800 to U+DFFF), and invalid code points MUST NOT be encoded.

### 3.3 Lower Half (Bits 31-0)

In UTF64 v1.0, all 32 bits of the lower half MUST be set to zero (0x00000000).

**Rationale**: Reserved for future specification versions to enable:

- Character metadata and styling flags
- Version identification markers
- Application-specific extensions
- Backward-compatible feature additions

### 3.4 Bit Ordering

UTF64 uses **big-endian** bit ordering by default:

- Most significant bit is bit 63
- Least significant bit is bit 0
- Octets are numbered 7 (MSB) to 0 (LSB)

Little-endian systems MUST perform appropriate byte-order conversion during serialization and deserialization.

---

## 4. Encoding Process

### 4.1 Algorithm

Given a Unicode code point U, the encoding process produces a 64-bit UTF64 character unit:

**Input**: Unicode code point U (U+0000 to U+10FFFF)
**Output**: 64-bit UTF64 character unit

**Steps**:

1. **Validate code point**:

   - MUST be in range [U+0000, U+10FFFF]
   - MUST NOT be in surrogate range [U+D800, U+DFFF]

2. **Encode to UTF-8**:

   - Apply UTF-8 encoding per RFC 3629
   - Result: 1-4 octets (b₀, b₁, b₂, b₃)

3. **Construct upper half**:

   ```
   U = (b₀ << 24) | (b₁ << 16) | (b₂ << 8) | b₃
   ```

   Where missing octets default to 0x00

4. **Construct UTF64 unit**:

   ```
   UTF64_UNIT = (U << 32) | 0x00000000
   ```

5. **Return** UTF64_UNIT

### 4.2 Pseudocode

```
function encode_utf64(codepoint: u32) -> u64:
    if codepoint > 0x10FFFF:
        return ERROR_INVALID_CODEPOINT

    if codepoint >= 0xD800 and codepoint <= 0xDFFF:
        return ERROR_SURROGATE_PAIR

    utf8_bytes = encode_utf8(codepoint)  // 1-4 bytes

    upper_half = 0
    for i in 0..utf8_bytes.length:
        upper_half |= (utf8_bytes[i] << (24 - i * 8))

    return (upper_half << 32) | 0x00000000
```

---

## 5. Decoding Process

### 5.1 Algorithm

Given a 64-bit UTF64 character unit, the decoding process produces a Unicode code point:

**Input**: 64-bit UTF64 character unit
**Output**: Unicode code point U

**Steps**:

1. **Validate reserved bits**:

   - Lower 32 bits MUST be 0x00000000
   - If non-zero, return ERROR_NON_ZERO_RESERVED

2. **Extract upper half**:

   ```
   U = (UTF64_UNIT >> 32) & 0xFFFFFFFF
   ```

3. **Extract UTF-8 octets**:

   ```
   b₀ = (U >> 24) & 0xFF
   b₁ = (U >> 16) & 0xFF
   b₂ = (U >>  8) & 0xFF
   b₃ = (U >>  0) & 0xFF
   ```

4. **Determine UTF-8 sequence length**:

   - If b₀ == 0x00: return ERROR_INVALID_UTF64
   - If b₀ < 0x80: length = 1
   - If b₀ < 0xE0: length = 2
   - If b₀ < 0xF0: length = 3
   - Else: length = 4

5. **Decode UTF-8**:

   - Extract octets [b₀, b₁, ..., b_{length-1}]
   - Apply UTF-8 decoding per RFC 3629
   - Validate: no overlong encodings, valid continuation bytes

6. **Return** decoded code point

### 5.2 Pseudocode

```
function decode_utf64(unit: u64) -> u32:
    // Validate reserved bits
    if (unit & 0xFFFFFFFF) != 0:
        return ERROR_NON_ZERO_RESERVED

    // Extract upper half
    upper = (unit >> 32)

    // Extract UTF-8 bytes
    bytes = [
        (upper >> 24) & 0xFF,
        (upper >> 16) & 0xFF,
        (upper >>  8) & 0xFF,
        (upper >>  0) & 0xFF
    ]

    // Validate first byte
    if bytes[0] == 0x00:
        return ERROR_INVALID_UTF64

    // Determine length
    length = utf8_sequence_length(bytes[0])

    // Decode UTF-8
    return decode_utf8(bytes[0..length])
```

---

## 6. Validation Requirements

### 6.1 Encoding Validation

Implementations MUST validate during encoding:

1. **Code point range**: U+0000 ≤ U ≤ U+10FFFF
2. **No surrogates**: U < U+D800 OR U > U+DFFF
3. **UTF-8 compliance**: Resulting UTF-8 adheres to RFC 3629

### 6.2 Decoding Validation

Implementations MUST validate during decoding:

1. **Reserved bits**: Lower 32 bits == 0x00000000
2. **Non-zero upper half**: First octet != 0x00
3. **Valid UTF-8**: Proper continuation bytes, no overlong encodings
4. **Code point validity**: Result in valid Unicode range

### 6.3 Validation Failure

Upon validation failure, implementations:

- MUST return an error (MUST NOT silently continue)
- MUST NOT produce undefined behavior
- SHOULD provide diagnostic information
- MAY define application-specific recovery strategies

---

## 7. Error Handling

### 7.1 Error Types

Implementations MUST distinguish between:

**InvalidCodePoint**

- Description: Input code point outside valid Unicode range
- Condition: U < U+0000 OR U > U+10FFFF OR (U+D800 ≤ U ≤ U+DFFF)

**InvalidUtf8**

- Description: Malformed UTF-8 in upper half
- Condition: Overlong encoding, invalid continuation, or illegal byte sequence

**InvalidUtf64**

- Description: Malformed UTF64 structure
- Condition: First octet is 0x00, or structural inconsistency

**NonZeroReservedBits**

- Description: Reserved bits are not zero
- Condition: (unit & 0xFFFFFFFF) != 0x00000000

### 7.2 Error Reporting

Implementations SHOULD provide:

- Error type classification
- Byte offset of error (for sequences)
- Diagnostic context (invalid byte values, expected vs. actual)

---

## 8. Byte Order and Serialization

### 8.1 Memory Representation

UTF64 character units MAY be stored in either byte order:

**Big-Endian** (Network Byte Order):

```
Offset:  0   1   2   3   4   5   6   7
        ┌───┬───┬───┬───┬───┬───┬───┬───┐
        │ b₀│ b₁│ b₂│ b₃│ 00│ 00│ 00│ 00│
        └───┴───┴───┴───┴───┴───┴───┴───┘
```

**Little-Endian**:

```
Offset:  0   1   2   3   4   5   6   7
        ┌───┬───┬───┬───┬───┬───┬───┬───┐
        │ 00│ 00│ 00│ 00│ b₃│ b₂│ b₁│ b₀│
        └───┴───┴───┴───┴───┴───┴───┴───┘
```

### 8.2 File Format

UTF64 files:

- MAY use any byte order (system-dependent)
- SHOULD include a Byte Order Mark (BOM) for interchange
- UTF64 BOM: 0x0000FEFF00000000 (big-endian) representing U+FEFF

### 8.3 Network Transmission

For network protocols:

- Big-endian SHOULD be used (network byte order)
- Protocol specifications MAY mandate byte order
- Implementations MUST document byte order requirements

---

## 9. Security Considerations

### 9.1 Overlong Encodings

Implementations MUST reject overlong UTF-8 encodings in the upper half. Overlong encodings can bypass security filters and MUST be treated as malformed data.

### 9.2 Surrogate Pairs

UTF64 MUST NOT encode surrogate code points (U+D800 to U+DFFF). These are not valid Unicode scalar values and indicate malformed input.

### 9.3 Reserved Bits Exploitation

Implementations MUST validate reserved bits are zero. Non-zero reserved bits:

- May indicate future specification versions
- May indicate corrupted data
- MUST NOT be silently ignored

### 9.4 Buffer Overflows

Fixed-width encoding simplifies bounds checking:

- Character count \* 8 = byte count
- No complex boundary detection required
- Reduces buffer overflow attack surface

### 9.5 Denial of Service

Constant-time operations reduce DoS attack vectors:

- No pathological inputs causing O(n²) behavior
- Predictable memory and CPU usage
- Length calculation is O(1)

---

## 10. Version Compatibility

### 10.1 Version Identification

UTF64 v1.0 is identified by:

- All reserved bits (lower 32) set to zero
- No explicit version marker required

Future versions MAY use reserved bits for:

- Explicit version numbers
- Feature flags
- Extended metadata

### 10.2 Forward Compatibility

UTF64 v1.0 implementations encountering non-zero reserved bits:

- MUST reject the data (fail-safe)
- SHOULD report NonZeroReservedBits error
- MAY provide version detection hints to user

This ensures v1.0 parsers never silently misinterpret future-versioned data.

### 10.3 Backward Compatibility

Future UTF64 versions:

- MUST support decoding v1.0 data (all-zero reserved bits)
- SHOULD auto-detect version from reserved bits
- MAY provide version-specific processing paths

### 10.4 Extension Mechanism

Reserved bits enable extensions:

```
Potential v2.0 Layout:
Bits 63-32:  UTF-8 encoding (unchanged)
Bits 31-28:  Version identifier (0x0 = v1.0, 0x1 = v2.0, etc.)
Bits 27-0:   Version-specific data
```

---

## 11. Implementation Requirements

### 11.1 Conformance Levels

**Level 1: Basic Conformance**

- Encode valid Unicode to UTF64
- Decode UTF64 to Unicode
- Validate reserved bits are zero
- Report errors for invalid input

**Level 2: Extended Conformance**

- All Level 1 requirements
- BOM handling
- Byte order conversion
- Comprehensive error diagnostics

**Level 3: Full Conformance**

- All Level 2 requirements
- Version detection
- Performance optimizations (SIMD, cache alignment)
- Streaming encode/decode

### 11.2 Testing Requirements

Conformant implementations MUST pass test cases for:

1. **Round-trip fidelity**: encode(decode(x)) == x for all valid x
2. **Error detection**: Reject all invalid inputs
3. **Boundary conditions**: U+0000, U+007F, U+0080, U+07FF, U+0800, U+FFFF, U+10000, U+10FFFF
4. **Surrogate rejection**: U+D800 to U+DFFF
5. **Reserved bit validation**: Non-zero lower 32 bits

### 11.3 Performance Expectations

Implementations SHOULD achieve:

- O(1) character indexing
- O(1) length calculation
- O(n) string traversal (single pass)
- Cache-efficient sequential access

---

## Appendix A: Examples

### A.1 ASCII Character 'A' (U+0041)

```
Code Point:   U+0041
UTF-8:        0x41
UTF64:        0x4100000000000000

Binary:       01000001 00000000 00000000 00000000
              00000000 00000000 00000000 00000000
              └─ UTF-8 ─────────────────┘└─Reserved─┘
```

### A.2 Euro Sign '€' (U+20AC)

```
Code Point:   U+20AC
UTF-8:        0xE2 0x82 0xAC (3 bytes)
UTF64:        0xE282AC0000000000

Binary:       11100010 10000010 10101100 00000000
              00000000 00000000 00000000 00000000
              └─ UTF-8 ─────────────────┘└─Reserved─┘
```

### A.3 Emoji '😀' (U+1F600)

```
Code Point:   U+1F600
UTF-8:        0xF0 0x9F 0x98 0x80 (4 bytes)
UTF64:        0xF09F988000000000

Binary:       11110000 10011111 10011000 10000000
              00000000 00000000 00000000 00000000
              └─ UTF-8 ─────────────────┘└─Reserved─┘
```

### A.4 String "Hi🌍"

```
Character  Code Point  UTF64 (hex)
─────────────────────────────────────────────
'H'        U+0048      0x4800000000000000
'i'        U+0069      0x6900000000000000
'🌍'       U+1F30D     0xF09F8C8D00000000

Memory Layout (24 bytes, big-endian):
48 00 00 00 00 00 00 00
69 00 00 00 00 00 00 00
F0 9F 8C 8D 00 00 00 00
```

---

## Appendix B: Comparison with Other Encodings

### B.1 Space Efficiency

| Character Type | UTF-8   | UTF-16  | UTF-32  | UTF64   |
| -------------- | ------- | ------- | ------- | ------- |
| ASCII (A-Z)    | 1 byte  | 2 bytes | 4 bytes | 8 bytes |
| Latin-1 (é)    | 2 bytes | 2 bytes | 4 bytes | 8 bytes |
| CJK (中)       | 3 bytes | 2 bytes | 4 bytes | 8 bytes |
| Emoji (😀)     | 4 bytes | 4 bytes | 4 bytes | 8 bytes |

### B.2 Algorithmic Complexity

| Operation          | UTF-8 | UTF-16 | UTF-32 | UTF64 |
| ------------------ | ----- | ------ | ------ | ----- |
| Character indexing | O(n)  | O(n)\* | O(1)   | O(1)  |
| Length counting    | O(n)  | O(n)\* | O(1)   | O(1)  |
| String traversal   | O(n)  | O(n)   | O(n)   | O(n)  |
| Validation         | O(n)  | O(n)   | O(1)   | O(n)† |

\* O(1) for BMP-only text, O(n) with surrogates
† Validation is O(n) but per-character validation is simpler

### B.3 Cache Performance

**Cache Line Utilization** (64-byte cache lines):

- UTF-8: Variable (1-64 characters per line)
- UTF-16: 32 characters (BMP) or 16 characters (with surrogates)
- UTF-32: 16 characters
- UTF64: **8 characters (perfect alignment)**

---

## References

### Normative References

- **[RFC 3629]**: UTF-8, a transformation format of ISO 10646
  https://www.rfc-editor.org/rfc/rfc3629.html

- **[RFC 2119]**: Key words for use in RFCs to Indicate Requirement Levels
  https://www.rfc-editor.org/rfc/rfc2119.html

- **[Unicode 15.0]**: The Unicode Consortium. The Unicode Standard, Version 15.0
  https://www.unicode.org/versions/Unicode15.0.0/

### Informative References

- **[RFC 2781]**: UTF-16, an encoding of ISO 10646
  https://www.rfc-editor.org/rfc/rfc2781.html

- **[ISO/IEC 10646]**: Information technology — Universal Coded Character Set (UCS)
  https://www.iso.org/standard/76835.html

- **[Hennessy & Patterson]**: Computer Architecture: A Quantitative Approach, 6th Edition (2017)
  ISBN: 978-0128119051

---

## Acknowledgments

The UTF64 specification builds upon decades of character encoding research and the foundational work of the Unicode Consortium. Special recognition to the developers of UTF-8, UTF-16, and UTF-32 whose designs informed this specification.

---

**Document History**

| Version | Date       | Changes                       |
| ------- | ---------- | ----------------------------- |
| 1.0     | 2025-01-10 | Initial specification release |

---

**Copyright Notice**

This document is released into the public domain. Implementers may freely use this specification without restriction.

---

**End of Specification**
