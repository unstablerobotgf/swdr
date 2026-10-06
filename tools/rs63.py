"""RS(63,35) over GF(64) (x^6+x+1, roots a^1..a^28) with erasures, for P25 Phase 2 ACCH.
Symbol 0 is the highest-order coefficient (first transmitted)."""
EXP, LOG = [0] * 128, [0] * 64
x = 1
for i in range(63):
    EXP[i] = EXP[i + 63] = x
    LOG[x] = i
    x <<= 1
    if x & 0x40:
        x ^= 0x43
N, NPAR = 63, 28


def mul(a, b):
    return 0 if a == 0 or b == 0 else EXP[LOG[a] + LOG[b]]


def inv(a):
    return EXP[63 - LOG[a]]


def peval(p, xv):  # p[0] = lowest degree
    y = 0
    for c in reversed(p):
        y = mul(y, xv) ^ c
    return y


def pmul(p, q):
    r = [0] * (len(p) + len(q) - 1)
    for i, a in enumerate(p):
        for j, b in enumerate(q):
            r[i + j] ^= mul(a, b)
    return r


def syndromes(cw):
    return [peval(cw[::-1], EXP[j + 1]) for j in range(NPAR)]  # S_j = c(a^(j+1))


def decode(cw, erasures=()):
    """Errors and erasures; returns (corrected codeword, symbols changed) or (None, -1)."""
    cw = list(cw)
    S = syndromes(cw)
    if not any(S):
        return cw, 0
    era = sorted(set(erasures))
    gamma = [1]  # erasure locator, prod (1 + X_i x), X_i = a^(N-1-pos)
    for p in era:
        gamma = pmul(gamma, [1, EXP[N - 1 - p]])
    # Modified syndromes remove the erasures; plain Berlekamp-Massey on T_e..T_27 finds the errors.
    T = pmul(gamma, S)[:NPAR]
    seq = T[len(era):]
    C, B, L, m, b = [1], [1], 0, 1, 1
    for n in range(len(seq)):
        d = seq[n]
        for i in range(1, L + 1):
            if i < len(C) and n - i >= 0:
                d ^= mul(C[i], seq[n - i])
        if d == 0:
            m += 1
            continue
        coef = mul(d, inv(b))
        adj = [0] * m + [mul(coef, v) for v in B]
        new = [(C[i] if i < len(C) else 0) ^ (adj[i] if i < len(adj) else 0) for i in range(max(len(C), len(adj)))]
        if 2 * L <= n:
            B, L, b, m = C, n + 1 - L, d, 1
        else:
            m += 1
        C = new
    if 2 * L + len(era) > NPAR:
        return None, -1
    psi = pmul(C, gamma)
    while len(psi) > 1 and psi[-1] == 0:
        psi.pop()
    pos = [p for p in range(N) if peval(psi, inv(EXP[N - 1 - p])) == 0]
    if len(pos) != len(psi) - 1:
        return None, -1
    om = pmul(S, psi)[:NPAR]
    dpsi = [psi[i] if i % 2 == 1 else 0 for i in range(1, len(psi))]
    changed = 0
    for p in pos:
        xi_inv = inv(EXP[N - 1 - p])
        den = peval(dpsi, xi_inv)
        if den == 0:
            return None, -1
        y = mul(peval(om, xi_inv), inv(den))  # fcr = 1: Y = Omega(X^-1) / Psi'(X^-1)
        changed += y != 0
        cw[p] ^= y
    if any(syndromes(cw)):
        return None, -1
    return cw, changed
