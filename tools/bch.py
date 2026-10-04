"""BCH(63,16,23) for the P25 NID: reference encoder/decoder used to validate firmware/src/bch.rs.

Generator (octal) 6331141367235453 per TIA-102.BAAA (as used by OP25 bch.cc / SDRTrunk
BCH_63_16_23_P25). GF(2^6) with primitive polynomial x^6 + x + 1. Codeword bit 62 is the first
transmitted bit; info (NAC 12 bits, DUID 4 bits) occupies bits 62..47, parity bits 46..0.
"""
import random

GEN = int("6331141367235453", 8)  # degree 47
EXP = [0] * 126
LOG = [0] * 64
x = 1
for i in range(63):
    EXP[i] = EXP[i + 63] = x
    LOG[x] = i
    x <<= 1
    if x & 0x40:
        x ^= 0x43  # x^6 + x + 1


def gmul(a, b):
    return 0 if a == 0 or b == 0 else EXP[LOG[a] + LOG[b]]


def poly_eval_bits(word, power):
    """Evaluate the 63-bit codeword polynomial at alpha^power."""
    s = 0
    for i in range(63):
        if word >> i & 1:
            s ^= EXP[(i * power) % 63]
    return s


def encode(info16):
    msg = info16 << 47
    rem = msg
    for i in range(62, 46, -1):
        if rem >> i & 1:
            rem ^= GEN << (i - 47)
    return msg | rem


def decode(word, t=11):
    """Returns (info16, corrected_bits) or (None, -1) if uncorrectable."""
    syn = [poly_eval_bits(word, i) for i in range(1, 2 * t + 1)]
    if not any(syn):
        return word >> 47, 0
    # Berlekamp-Massey over GF(64).
    C, B, L, m, b = [1] + [0] * 2 * t, [1] + [0] * 2 * t, 0, 1, 1
    for n in range(2 * t):
        d = syn[n]
        for i in range(1, L + 1):
            d ^= gmul(C[i], syn[n - i])
        if d == 0:
            m += 1
        elif 2 * L <= n:
            T = C[:]
            coef = gmul(d, EXP[(63 - LOG[b]) % 63])
            for i in range(m, 2 * t + 1):
                C[i] ^= gmul(coef, B[i - m])
            L, B, b, m = n + 1 - L, T, d, 1
        else:
            coef = gmul(d, EXP[(63 - LOG[b]) % 63])
            for i in range(m, 2 * t + 1):
                C[i] ^= gmul(coef, B[i - m])
            m += 1
    if L > t:
        return None, -1
    # Chien search: error at position i if C(alpha^-i) == 0.
    errs = []
    for i in range(63):
        s = 0
        for j in range(L + 1):
            if C[j]:
                s ^= EXP[(LOG[C[j]] + j * (63 - i)) % 63]
        if s == 0:
            errs.append(i)
    if len(errs) != L:
        return None, -1
    for i in errs:
        word ^= 1 << i
    return word >> 47, L


def self_test():
    # Generator must vanish at alpha^1..alpha^22 (designed distance 23).
    g_roots = [poly_eval_bits(GEN, i) == 0 for i in range(1, 23)]
    assert all(g_roots), "generator/field mismatch"
    rnd = random.Random(1)
    for _ in range(3000):
        info = rnd.getrandbits(16)
        cw = encode(info)
        nerr = rnd.randint(0, 11)
        bad = cw
        for p in rnd.sample(range(63), nerr):
            bad ^= 1 << p
        got, n = decode(bad)
        assert got == info and n == nerr, (hex(info), nerr, got, n)
    print("bch self-test: generator roots alpha^1..22 ok; 3000 random words with 0..11 errors decoded")


if __name__ == "__main__":
    self_test()
