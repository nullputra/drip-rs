# Setup:
# $ pip install numpy
# Run:
# $ py -3 tests\data\fft.py
import numpy as np


def convolve(f, g):
    # Ref: https://maspypy.com/数学・numpy-高速フーリエ変換fftによる畳み込み
    # Usage:
    #   f = np.array(a, np.int64)
    #   g = np.array(a, np.int64)
    #   h = convolve(f, g)
    fft_len = 1
    while 2 * fft_len < len(f) + len(g) - 1:
        fft_len *= 2
    fft_len *= 2
    # FFT
    Ff = np.fft.rfft(f, fft_len)
    Fg = np.fft.rfft(g, fft_len)
    Fh = Ff * Fg
    # IFFT
    h = np.fft.irfft(Fh, fft_len)
    # 整数にまるめる
    h = np.rint(h).astype(np.int64)
    return h[: len(f) + len(g) - 1]


def main():
    f = [1, 2, 3, 4]
    g = [1, 2, 4, 8]
    # 1, 4, 11, 26, 36, 40, 32
    print(*convolve(f, g))


if __name__ == "__main__":
    main()
