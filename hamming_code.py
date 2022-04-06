# Hamming(7, 4) implementation using different G and H matrices than Wikipedia.
import numpy as np

def hamming_encode_7_4(p):
    G = np.array(
        [[ 0, 1, 1, 1, 0, 0, 0 ],
         [ 1, 0, 1, 0, 1, 0, 0 ],
         [ 1, 1, 0, 0, 0, 1, 0 ],
         [ 1, 1, 1, 0, 0, 0, 1 ]], dtype=np.uint8)

    x = (p @ G) % 2
    return x

def hamming_decode_7_4(x):
    H = np.array(
        [[1, 0, 0, 0, 1, 1, 1],
         [0, 1, 0, 1, 0, 1, 1],
         [0, 0, 1, 1, 1, 0, 1]], dtype=np.uint8)
    s = (H @ x) % 2

    if s.sum() > 0:
        index = _s_to_index(s)
        x[index] ^= 1

    data = _get_data(x)
    return data

def _get_data(x):
    d = []
    for i in range(3, 7):
        d.append(x[i])
    return np.array(d, dtype=np.uint8)

def _s_to_index(s):
    s = list(s)
    index = int(''.join([str(n) for n in s]), 2)
    return (4, 2, 1, 3, 5, 6, 7).index(index)

def main():
    n = (
        np.array([1, 0, 1, 1], dtype=np.uint8),
        np.array([1, 0, 0, 0], dtype=np.uint8),
        np.array([1, 1, 1, 1], dtype=np.uint8),
        np.array([1, 0, 1, 0], dtype=np.uint8),
        np.array([1, 1, 0, 0], dtype=np.uint8)
    )
    x = (
        hamming_encode_7_4(n[0]),
        hamming_encode_7_4(n[1]),
        hamming_encode_7_4(n[2]),
        hamming_encode_7_4(n[3]),
        hamming_encode_7_4(n[4])
    )
    expected_x = (
        np.array([0, 1, 0, 1, 0, 1, 1], dtype=np.uint8),
        np.array([0, 1, 1, 1, 0, 0, 0], dtype=np.uint8),
        np.array([1, 1, 1, 1, 1, 1, 1], dtype=np.uint8),
        np.array([1, 0, 1, 1, 0, 1, 0], dtype=np.uint8),
        np.array([1, 1, 0, 1, 1, 0, 0], dtype=np.uint8)
    )
    for i, c in enumerate(x):
        assert np.array_equal(c, expected_x[i])
    print('All encoded data matches expected')
    for j, c in enumerate(x):
        for i in range(7):
            e = np.zeros(7, dtype=np.uint8)
            e[i] = 1
            d = hamming_decode_7_4((c + e) % 2)
            assert np.array_equal(d, n[j])
    print('All single-bit errors were successfully corrected')

if __name__ == '__main__':
    main()
