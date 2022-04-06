#[derive(Debug, PartialEq)]
enum ErrorType {
    NoError,
    SingleBitError,
    ParityBitError,
    DoubleBitError,
}

/// Encode n using Hamming(8, 4).
///     h 2 1 0 d 3 2 1 0   d
/// G = [ 0 1 1 | 1 0 0 0   3
///       1 0 1 | 0 1 0 0   2
///       1 1 0 | 0 0 1 0   1
///       1 1 1 | 0 0 0 1 ] 0
/// x = nG: (h2 h1 h0 d3 d2 d1 d0)
/// Returns:
///     x bits + parity bit i.e. h2 h1 h0 d3 d2 d1 d0 p
fn hamming_encode(n: u8) -> u8 {
    // Data bits
    let d: [u8; 4] = [n >> 0 & 1, n >> 1 & 1, n >> 2 & 1, n >> 3 & 1];

    // Calculate Hamming bits using G
    let h: [u8; 3] = [d[3] ^ d[2] ^ d[0], d[3] ^ d[1] ^ d[0], d[2] ^ d[1] ^ d[0]];

    // Calculate parity bit
    let p: u8 = h[2] ^ h[1] ^ h[0] ^ d[3] ^ d[2] ^ d[1] ^ d[0];

    h[2] << 7 | h[1] << 6 | h[0] << 5 | d[3] << 4 | d[2] << 3 | d[1] << 2 | d[0] << 1 | p
}

/// Decode x into 4 data bits.
///     x 7 6 5   4 3 2 1   s
/// H = [ 1 0 0 | 0 1 1 1   0
///       0 1 0 | 1 0 1 1   1
///       0 0 1 | 1 1 0 1 ] 2
/// s = Hx: (s0 s1 s2)^T
/// Returns: error-corrected data bits i.e. error-corrected x4 x3 x2 x1
fn hamming_decode(x: u8) -> (u8, ErrorType) {
    // Calculate syndrome bits using H
    let s: [u8; 3] = [
        (x >> 7 & 1) ^ (x >> 3 & 1) ^ (x >> 2 & 1) ^ (x >> 1 & 1),
        (x >> 6 & 1) ^ (x >> 4 & 1) ^ (x >> 2 & 1) ^ (x >> 1 & 1),
        (x >> 5 & 1) ^ (x >> 4 & 1) ^ (x >> 3 & 1) ^ (x >> 1 & 1),
    ];

    // Parse syndrome bits taking left bits to be more significant.
    let syndrome: usize = (s[0] as usize) << 2 | (s[1] as usize) << 1 | (s[2] as usize) << 0;

    // Map syndrome to incorrect bit position: 7 -> 1, 6 -> 2, 5 -> 3, 3 -> 4, 1 -> 5, 2 -> 6, 4 -> 7
    // e.g. if syndrome = 4 then flip bit x7.
    const SYNDROME_TO_BIT: [usize; 8] = [0, 5, 6, 4, 7, 3, 2, 1];
    let incorrect_bit = SYNDROME_TO_BIT[syndrome];
    let data = (x ^ 1 << incorrect_bit) >> 1 & 0xF; // Flip incorrect bit and return data bits
    let x_parity = (x >> 7 & 1)
        ^ (x >> 6 & 1)
        ^ (x >> 5 & 1)
        ^ (x >> 4 & 1)
        ^ (x >> 3 & 1)
        ^ (x >> 2 & 1)
        ^ (x >> 1 & 1);

    let error_type;
    if x & 1 != x_parity {
        if incorrect_bit > 0 {
            error_type = ErrorType::SingleBitError;
        } else {
            error_type = ErrorType::ParityBitError;
        }
    } else {
        if incorrect_bit == 0 {
            error_type = ErrorType::NoError;
        } else {
            error_type = ErrorType::DoubleBitError;
        }
    }
    (data, error_type)
}

fn main() {
    const N: [u8; 4] = [0b00001011, 0b00001000, 0b00001111, 0b00001010];
    const EXPECTED_X: [u8; 4] = [0b01010110, 0b01110001, 0b11111111, 0b10110100];
    let x: [u8; 4] = [
        hamming_encode(N[0]),
        hamming_encode(N[1]),
        hamming_encode(N[2]),
        hamming_encode(N[3]),
    ];

    test_expected_x(&x, &EXPECTED_X);
    test_single_bit_or_parity_error(&x, &N);
    test_no_error(&x, &N);
    test_double_bit_error(&x);
    println!("ALL TESTS PASSED!!!!!!!!");
}

fn test_double_bit_error(x: &[u8; 4]) {
    let mut count;
    let mut errored;
    let mut decoded;
    let mut e1;
    let mut e2;
    let mut e;

    for c in x {
        e1 = 1;
        e2 = 2;
        count = 0;

        // 8 choose 2 = 28 combinations. Can't think of a better way to do this.
        while e1 >> 6 & 1 == 0 {
            while e2 >> 7 & 1 == 0 {
                e = e1 | e2;
                errored = *c ^ e;
                decoded = hamming_decode(errored);
                assert_eq!(decoded.1, ErrorType::DoubleBitError);
                e2 <<= 1;
                count = count + 1;
            }
            e1 <<= 1;
            e2 = e1 << 1;
        }
        for i in 0..=6 {
            e = e1 >> i | e2;
            errored = *c ^ e;
            decoded = hamming_decode(errored);
            assert_eq!(decoded.1, ErrorType::DoubleBitError);
            count = count + 1;
        }
        assert_eq!(count, 28); // Sanity check
    }
    println!("Double-bit errors were successfully detected");
}

fn test_no_error(x: &[u8; 4], n: &[u8; 4]) {
    let mut decoded;
    for (i, c) in x.iter().enumerate() {
        decoded = hamming_decode(*c);
        assert_eq!(decoded.0, n[i]);
        assert_eq!(decoded.1, ErrorType::NoError);
    }
    println!("Data with no errors successfully decoded.");
}

fn test_single_bit_or_parity_error(x: &[u8; 4], n: &[u8; 4]) {
    let mut decoded;
    for (i, c) in x.iter().enumerate() {
        for j in 0..8 {
            let errored = *c ^ 1 << j;
            decoded = hamming_decode(errored);
            assert_eq!(decoded.0, n[i]);
            if j == 0 {
                assert_eq!(decoded.1, ErrorType::ParityBitError);
            } else {
                assert_eq!(decoded.1, ErrorType::SingleBitError);
            }
        }
    }
    println!("Single-bit/parity-bit errors were successfully error corrected");
}

fn test_expected_x(x: &[u8; 4], expected_x: &[u8; 4]) {
    for (i, c) in x.iter().enumerate() {
        assert_eq!(*c, expected_x[i]);
    }
    println!("Encoded data matches expected");
}
