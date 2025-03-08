use std::fmt::Debug;

pub fn gcd<T: num::Integer + Copy>(mut a: T, mut b: T) -> T {
    while !b.is_zero() {
        (a, b) = (b, a % b);
    }
    a
}

pub fn inverse_mod<T: num::Integer + Copy + Debug>(a: T, n: T) -> T {
    // If a*x + n*y = 1, then a*x = 1 - n*y === 1 (mod n).
    let (mut old_r, mut r) = (a, n);
    let (mut old_s, mut s) = (T::one(), T::zero());
    let (mut old_t, mut t) = (T::zero(), T::one());

    while r != T::zero() {
        let quotient = old_r / r;
        (old_r, r) = (r, old_r - quotient * r);
        (old_s, s) = (s, old_s - quotient * s);
        (old_t, t) = (t, old_t - quotient * t);
    }

    assert_eq!(old_r, T::one());

    // Now a*old_s + n*old_t = 1
    old_s % n
}

pub fn lcm<T: num::Integer + Copy>(a: T, b: T) -> T {
    let d = gcd(a, b);

    if d.is_zero() { d } else { (a * b) / d }
}