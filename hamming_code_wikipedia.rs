/// Hamming(8, 4) implementation - based on Wikipedia's G and H matrices.
/// Encode 4-bits using hamming_encode().
/// Error-correct 8-bit encoding using hamming_error_correct().
/// Decode 8-bit encoding to 4-bits of data using hamming_decode().

/// Bit error types.
#[derive(Debug, PartialEq)]
enum ErrorType {
    NoError,
    SingleBitError,
    ParityBitError,
    DoubleBitError,
}

/// Encode n in Hamming(8, 4).
///     x 7 6 5 4 3 2 1 0   d
/// G = [ 1 1 1 0 0 0 0 1   3
///       1 0 0 1 1 0 0 1   2
///       0 1 0 1 0 1 0 1   1
///       1 1 0 1 0 0 1 0 ] 0
/// x = nG: (x7 x6 x5 x4 x3 x2 x1 x0)
/// Returns:
///     x bits
fn hamming_encode(n: u8) -> u8 {
    // Data bits
    let d: [u8; 4] = [n >> 0 & 1, n >> 1 & 1, n >> 2 & 1, n >> 3 & 1];

    // Calculate Hamming bits using G
    let x: [u8; 8] = [
        d[3] ^ d[2] ^ d[1],
        d[0],
        d[1],
        d[2],
        d[2] ^ d[1] ^ d[0],
        d[3],
        d[3] ^ d[1] ^ d[0],
        d[3] ^ d[2] ^ d[0],
    ];

    x[7] << 7 | x[6] << 6 | x[5] << 5 | x[4] << 4 | x[3] << 3 | x[2] << 2 | x[1] << 1 | x[0]
}

/// Error correct a Hamming(8, 4) encoded byte using H.
///     x 7 6 5 4 3 2 1   s
/// H = [ 1 0 1 0 1 0 1   0
///       0 1 1 0 0 1 1   1
///       0 0 0 1 1 1 1 ] 2
/// s = Hx: (s0 s1 s2)^T, map s to incorrect bit position
/// Returns: error corrected byte
fn hamming_error_correct(x: u8) -> (u8, ErrorType) {
    // Calculate syndrome bits using H
    let s: [u8; 3] = [
        (x >> 7 & 1) ^ (x >> 5 & 1) ^ (x >> 3 & 1) ^ (x >> 1 & 1),
        (x >> 6 & 1) ^ (x >> 5 & 1) ^ (x >> 2 & 1) ^ (x >> 1 & 1),
        (x >> 4 & 1) ^ (x >> 3 & 1) ^ (x >> 2 & 1) ^ (x >> 1 & 1),
    ];

    let syndrome: usize = (s[2] << 2 | s[1] << 1 | s[0] << 0) as usize;

    let x_parity = (x >> 7 & 1)
        ^ (x >> 6 & 1)
        ^ (x >> 5 & 1)
        ^ (x >> 4 & 1)
        ^ (x >> 3 & 1)
        ^ (x >> 2 & 1)
        ^ (x >> 1 & 1);

    let error_type;
    if x & 1 != x_parity {
        if syndrome > 0 {
            error_type = ErrorType::SingleBitError;
        } else {
            return (x ^ 1, ErrorType::ParityBitError);
        }
    } else {
        if syndrome == 0 {
            return (x, ErrorType::NoError);
        } else {
            error_type = ErrorType::DoubleBitError;
        }
    }

    // Other cases: single-bit or double-bit error.
    // Double-bit error cannot be corrected, but try anyway.
    // Map syndrome to incorrect bit position, e.g. if syndrome = 4 then flip bit x3.
    // Syndrome should not be 0 at this point.
    const SYNDROME_TO_BIT: [u8; 8] = [u8::MAX, 7, 6, 5, 4, 3, 2, 1];
    let incorrect_bit = SYNDROME_TO_BIT[syndrome];
    (x ^ 1 << incorrect_bit, error_type)
}

/// Returns: error-corrected data bits i.e. error-corrected x7 x6 x5 x4
fn hamming_decode(x: u8) -> (u8, ErrorType) {
    let (data, error_type) = hamming_error_correct(x);
    let d: [u8; 4] = [data >> 1 & 1, data >> 2 & 1, data >> 3 & 1, data >> 5 & 1];
    (d[3] << 3 | d[2] << 2 | d[1] << 1 | d[0] << 0, error_type)
}

fn main() {
    const N: [u8; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    // Calculated by hand (x = nG)
    const EXPECTED_X: [u8; 16] = [
        0, 0b11010010, 0b01010101, 0b10000111, 0b10011001, 0b01001011, 0b11001100, 0b00011110,
        0b11100001, 0b00110011, 0b10110100, 0b01100110, 0b01111000, 0b10101010, 0b00101101, 0xFF,
    ];
    let x = [
        hamming_encode(N[0]),
        hamming_encode(N[1]),
        hamming_encode(N[2]),
        hamming_encode(N[3]),
        hamming_encode(N[4]),
        hamming_encode(N[5]),
        hamming_encode(N[6]),
        hamming_encode(N[7]),
        hamming_encode(N[8]),
        hamming_encode(N[9]),
        hamming_encode(N[10]),
        hamming_encode(N[11]),
        hamming_encode(N[12]),
        hamming_encode(N[13]),
        hamming_encode(N[14]),
        hamming_encode(N[15]),
    ];

    test_expected_x(&x, &EXPECTED_X);
    test_single_bit_or_parity_error(&x, &N);
    test_no_error(&x, &N);
    test_double_bit_error(&x);
    println!("ALL TESTS PASSED!!!!!!!!");
}

/// Check every combination of 2-bit errors is detected for each encoded x.
fn test_double_bit_error(x: &[u8]) {
    const NUM_BITS: u8 = 8;
    let mut error_type;
    let mut e;

    for encoded in x {
        for i in 0..NUM_BITS - 1 {
            for j in i + 1..NUM_BITS {
                e = 1 << i | 1 << j;
                (_, error_type) = hamming_decode(encoded ^ e);
                assert_eq!(error_type, ErrorType::DoubleBitError);
            }
        }
    }
    println!("Double-bit errors successfully detected.");
}

/// Test no-error if decoding the encoded byte.
fn test_no_error(x: &[u8], n: &[u8]) {
    let mut decoded;
    let mut error_type;

    for (i, encoded) in x.iter().enumerate() {
        (decoded, error_type) = hamming_decode(*encoded);
        assert_eq!(decoded, n[i]);
        assert_eq!(error_type, ErrorType::NoError);
    }
    println!("Unchanged encodings successfully detected and decoded.");
}

/// For each encoded x byte, flip every bit and check if the decoded 4 bits are correct.
fn test_single_bit_or_parity_error(x: &[u8], n: &[u8]) {
    let mut decoded;
    let mut error_type;

    for (i, encoded) in x.iter().enumerate() {
        for bit in 0..8 {
            let errored = *encoded ^ 1 << bit;
            (decoded, error_type) = hamming_decode(errored);
            assert_eq!(decoded, n[i]);
            if bit == 0 {
                assert_eq!(error_type, ErrorType::ParityBitError);
            } else {
                assert_eq!(error_type, ErrorType::SingleBitError);
            }
        }
    }
    println!("Single-bit/parity-bit errors successfully detected and error-corrected.");
}

/// Assert each x matches expected x.
fn test_expected_x(x: &[u8], expected_x: &[u8]) {
    for (i, encoded) in x.iter().enumerate() {
        assert_eq!(*encoded, (*expected_x)[i]);
    }
    println!("Encoded data matches expected.");
}
