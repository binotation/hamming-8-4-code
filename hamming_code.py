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
    print(z)

    if z.sum() > 0:
        print('error detected')
        index = _z_to_index(z)
        x[index] ^= 1

    data =_get_data(x)
    return data

def _get_data(x):
    d = []
    for i in range(3, 7):
        d.append(x[i])
    return d

def _z_to_index(z):
    z = list(z)
    index = int(''.join([str(n) for n in z]), 2)
    return (4, 2, 1, 3, 5, 6, 7).index(index)

def main():
    x = hamming_encode_7_4(np.array([1, 0, 0, 0], dtype=np.uint8))
    print(x)
    for i in range(7):
        e = np.zeros(7, dtype=np.uint8)
        e[i] = 1
        z = hamming_decode_7_4((x + e) % 2)
        assert z == [1, 0, 0, 0]

if __name__ == '__main__':
    main()
