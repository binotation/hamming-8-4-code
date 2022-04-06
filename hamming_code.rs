/// Encode n using hamming(8, 4).
///     h 2 1 0 d 3 2 1 0   d
/// G = [ 0 1 1 | 1 0 0 0   3
///       1 0 1 | 0 1 0 0   2
///       1 1 0 | 0 0 1 0   1
///       1 1 1 | 0 0 0 1 ] 0
/// h = nG: (h2 h1 h0 d3 d2 d1 d0)
/// returns:
///     concatenated z bits + parity bit
///     i.e. h2 h1 h0 d3 d2 d1 d0 p
fn hamming_encode(n: u8) -> u8 {
    let d: [u8; 4] = [n >> 0 & 1, n >> 1 & 1, n >> 2 & 1, n >> 3 & 1];
    let h: [u8; 3] = [d[3] ^ d[2] ^ d[0], d[3] ^ d[1] ^ d[0], d[2] ^ d[1] ^ d[0]];
    let mut p: u8 = 0;
    for i in d {
        p ^= i;
    }
    for i in h {
        p ^= i;
    }
    h[2] << 7 | h[1] << 6 | h[0] << 5 | d[3] << 4 | d[2] << 3 | d[1] << 2 | d[0] << 1 | p
}

/// Decode x into 4 data bits.
///     x 7 6 5   4 3 2 1   s
/// H = [ 1 0 0 | 0 1 1 1   0
///       0 1 0 | 1 0 1 1   1
///       0 0 1 | 1 1 0 1 ] 2
/// s = Hx: (s0 s1 s2)^T
/// returns error-corrected data bits
/// i.e. error-corrected x4 x3 x2 x1
fn hamming_decode(mut x: u8) -> u8 {
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
    x = x ^ 1 << SYNDROME_TO_BIT[syndrome];

    x >> 1 & 0xF // Throw away parity bit
}

fn main() {
    let mut decoded: u8;
    let n: [u8; 4] = [0b00001011, 0b00001000, 0b00001111, 0b00001010];

    let x: [u8; 4] = [
        hamming_encode(n[0]),
        hamming_encode(n[1]),
        hamming_encode(n[2]),
        hamming_encode(n[3]),
    ];

    assert_eq!(x[0], 0b01010110);
    assert_eq!(x[1], 0b01110001);
    assert_eq!(x[2], 0b11111111);
    assert_eq!(x[3], 0b10110100);
    println!("Encoded data matches expected");
    for (i, c) in x.iter().enumerate() {
        for j in 1..8 {
            let errored = c ^ 1 << j;
            decoded = hamming_decode(errored);
            assert_eq!(decoded, n[i]);
        }
    }
    println!("Decoded data was successfully error corrected")
}
