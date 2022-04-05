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
    z = (H @ x) % 2

    if z.sum() > 0:
        index = _z_to_index(z)
        x[index] ^= 1

    data = _get_data(x)
    return data

def _get_data(x):
    d = []
    for i in range(3, 7):
        d.append(x[i])
    return np.array(d)

def _z_to_index(z):
    z = list(z)
    index = int(''.join([str(n) for n in z]), 2)
    return (4, 2, 1, 3, 5, 6, 7).index(index)

def main():
    expected_z = (
        np.array([1, 0, 1, 1], dtype=np.uint8),
        np.array([1, 0, 0, 0], dtype=np.uint8),
        np.array([1, 1, 1, 1], dtype=np.uint8),
        np.array([1, 0, 1, 0], dtype=np.uint8)
    )
    x = (
        hamming_encode_7_4(expected_z[0]),
        hamming_encode_7_4(expected_z[1]),
        hamming_encode_7_4(expected_z[2]),
        hamming_encode_7_4(expected_z[3])
    )
    expected_x = (
        np.array([0, 1, 0, 1, 0, 1, 1]),
        np.array([0, 1, 1, 1, 0, 0, 0]),
        np.array([1, 1, 1, 1, 1, 1, 1]),
        np.array([1, 0, 1, 1, 0, 1, 0])
    )
    for i, c in enumerate(x):
        assert np.array_equal(c, expected_x[i])
    print('All encoded data matches expected')
    for j, c in enumerate(x):
        for i in range(7):
            e = np.zeros(7, dtype=np.uint8)
            e[i] = 1
            z = hamming_decode_7_4((c + e) % 2)
            assert np.array_equal(z, expected_z[j])
    print('All single-bit errors were successfully corrected')

if __name__ == '__main__':
    main()
