use num_bigint::BigInt;
use num_traits::{One, Zero};

pub fn extended_euclid(a: BigInt, b: BigInt) -> (BigInt, BigInt, BigInt) {
    if b == BigInt::zero() {
        return (a, BigInt::one(), BigInt::zero());
    }

    let (d_prime, x_prime, y_prime) = extended_euclid(b.clone(), a.clone() % b.clone());
    (d_prime, y_prime.clone(), x_prime - a.clone() / b.clone() * y_prime)
}

pub fn modular_exponentiation(a: BigInt, b: BigInt, n: BigInt) -> BigInt {
    let mut d = BigInt::one();

    for i in (0..b.bits()).rev() {
        d = d.clone() * d % n.clone();

        if b.bit(i) {
            d = d * a.clone() % n.clone();
        }
    }

    d
}

pub fn pseudo_prime(n: BigInt) -> bool {
    modular_exponentiation(BigInt::from(2), n.clone() - BigInt::one(), n.clone()) % n == BigInt::one()
}


#[test]
fn test_utils() {
    let extended_euclid_test = extended_euclid(BigInt::from(99), BigInt::from(78));
    let modular_exponentiation_test = modular_exponentiation(BigInt::from(7), BigInt::from(560), BigInt::from(561));
    let pseudo_prime_prime = pseudo_prime(BigInt::from(13));
    let pseudo_prime_composite = pseudo_prime(BigInt::from(15));
    assert_eq!(extended_euclid_test, (BigInt::from(3), BigInt::from(-11), BigInt::from(14)));
    assert_eq!(modular_exponentiation_test, BigInt::one());
    assert_eq!(pseudo_prime_prime, true);
    assert_eq!(pseudo_prime_composite, false);
}
