pub fn gcd<T: num::Integer + Copy>(mut a: T, mut b: T) -> T {
    while !b.is_zero() {
        (a, b) = (b, a % b);
    }
    a
}

pub fn lcm<T: num::Integer + Copy>(a: T, b: T) -> T {
    let d = gcd(a, b);

    if d.is_zero() { d } else { (a * b) / d }
}