/// Encode n using hamming(8, 4).
///     h 2 1 0 d 3 2 1 0   d
/// G = [ 0 1 1 | 1 0 0 0   3
///       1 0 1 | 0 1 0 0   2
///       1 1 0 | 0 0 1 0   1
///       1 1 1 | 0 0 0 1 ] 0
/// z = nG: (h2 h1 h0 d3 d2 d1 d0)
/// returns:
///     concatenated z bits + parity bit
///     i.e. h2 h1 h0 d3 d2 d1 d0 p
fn hamming_encode(n: u8) -> u8 {
    let d: [u8; 4] = [
        n >> 0 & 1,
        n >> 1 & 1,
        n >> 2 & 1,
        n >> 3 & 1,
    ];
    let h: [u8; 3] = [
        d[3] ^ d[2] ^ d[0],
        d[3] ^ d[1] ^ d[0],
        d[2] ^ d[1] ^ d[0],
    ];
    let mut p: u8 = 0;
    for i in d {
        p ^= i;
    }
    for i in h {
        p ^= i;
    }
    h[2] << 7
        | h[1] << 6
        | h[0] << 5
        | d[3] << 4
        | d[2] << 3
        | d[1] << 2
        | d[0] << 1
        | p
}

fn main() {
    assert_eq!(hamming_encode(0b00001011), 0b01010110);
    assert_eq!(hamming_encode(0b00001000), 0b01110001);
    assert_eq!(hamming_encode(0b00001111), 0b11111111);
    assert_eq!(hamming_encode(0b00001010), 0b10110100);
}
