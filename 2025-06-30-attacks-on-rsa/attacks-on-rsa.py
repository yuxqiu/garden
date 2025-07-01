import random
import math

def miller_rabin_test(n, k=5):
    if n == 2 or n == 3:
        return True
    if n < 2 or n % 2 == 0:
        return False

    # Write n - 1 as 2^r * d
    r, d = 0, n - 1
    while d % 2 == 0:
        r += 1
        d //= 2

    # Witness loop
    for _ in range(k):
        a = random.randrange(2, n - 1)
        x = pow(a, d, n)
        if x == 1 or x == n - 1:
            continue
        for _ in range(r - 1):
            x = (x * x) % n
            if x == n - 1:
                break
        else:
            return False
    return True

def generate_prime(k, e=3):
    while True:
        p = random.randint(2**(k-1), 2**k - 1)
        if miller_rabin_test(p) and p % e != 1: # Avoid e | (pp-1)
            return p

def generate_safe_prime(k, e=3):
    while True:
        p = random.randint(2**(k-1), 2**k - 1)
        if miller_rabin_test(p):
            pp = 2 * p + 1
            # choose a safe prime
            if miller_rabin_test(pp) and pp % e != 1: # Avoid e | (pp-1)
                return pp

def mod_pow(base, exp, mod):
    return pow(base, exp, mod)

def extended_gcd(a, b):
    if a == 0:
        return b, 0, 1
    gcd, x1, y1 = extended_gcd(b % a, a)
    x = y1 - (b // a) * x1
    y = x1
    return gcd, x, y

def mod_inverse(e, phi):
    gcd, x, _ = extended_gcd(e, phi)
    if gcd != 1:
        raise ValueError("Modular inverse does not exist")
    return x % phi

def find_tr(k):
    t = 0
    r = k
    while r % 2 == 0:
        r //= 2
        t += 1
    return t, r

def generate_rsa_keys(k, e=3):
    while True:
        # Generate two distinct k-bit primes ensuring e is coprime with phi_N
        while True:
            p = generate_prime(k, e)
            q = generate_prime(k, e)
            if p != q:
                break

        N = p * q
        phi_N = (p - 1) * (q - 1)

        d = mod_inverse(e, phi_N)

        k_val = d * e - 1
        t, r = find_tr(k_val)

        return N, p, q, e, d, t, r, phi_N

def strategy1(g, N, t, r, p, q):
    tp, rp = find_tr(p - 1)
    tq, rq = find_tr(q - 1)
    mt = max(tp, tq)

    t = max(tp, tq) - 1
    exp = 2**t * r
    g_k2 = mod_pow(g, exp, N)
    factor = math.gcd(g_k2 - 1, N)
    if 1 < factor < N:
        return factor, True

    return None, False

def strategy2(g, N, t, r, p, q):
    for i in range(t-1, -1, -1):
        exp = 2**i * r
        g_k2 = mod_pow(g, exp, N)

        factor = math.gcd(g_k2 - 1, N)
        if 1 < factor < N:
            return factor, True

    return None, False

def try_factorize(N, t, r, p, q):
    while True:
        g = random.randint(1, N - 1)
        if math.gcd(g, N) == 1:
            break

    return strategy1(g, N, t, r, p, q)
    # return strategy2(g, N, t, r, p, q)

def simulate_probability(k, trials):
    success_count = 0
    for _ in range(trials):
        N, p, q, e, d, t, r, phi_N = generate_rsa_keys(k, e=3)
        _, success = try_factorize(N, t, r, p, q)
        if success:
            success_count += 1

    probability = success_count / trials
    return probability

# Parameters
key_size = 40  # Small key size for simulation
num_trials = 100000  # Number of trials for probability estimation

# Run simulation
prob = simulate_probability(key_size, num_trials)
print(f"Estimated probability of factoring N: {prob:.4f}")
