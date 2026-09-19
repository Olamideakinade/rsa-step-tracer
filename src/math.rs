use num_bigint::{BigInt, Sign};
use num_traits::{Zero, One, Signed, ToPrimitive};

pub fn gcd(a: &BigInt, b: &BigInt) -> BigInt {
    let mut x = a.abs();
    let mut y = b.abs();
    while !y.is_zero() {
        let temp = y.clone();
        y = &x % &y;
        x = temp;
    }
    x
}

pub fn extended_gcd(a: &BigInt, b: &BigInt) -> (BigInt, BigInt, BigInt) {
    let (mut old_r, mut r) = (a.clone(), b.clone());
    let (mut old_s, mut s) = (BigInt::one(), BigInt::zero());
    let (mut old_t, mut t) = (BigInt::zero(), BigInt::one());

    while !r.is_zero() {
        let quotient = &old_r / &r;
        
        let next_r = &old_r - &quotient * &r;
        old_r = r;
        r = next_r;

        let next_s = &old_s - &quotient * &s;
        old_s = s;
        s = next_s;

        let next_t = &old_t - &quotient * &t;
        old_t = t;
        t = next_t;
    }

    (old_r, old_s, old_t)
}

pub fn mod_inverse(e: &BigInt, phi: &BigInt) -> Result<BigInt, String> {
    let (g, x, _) = extended_gcd(e, phi);
    if g != BigInt::one() {
        return Err(format!("Modular inverse does not exist because gcd is not 1 (gcd = {})", g));
    }
    let result = x % phi;
    if result.is_negative() {
        Ok(&result + phi)
    } else {
        Ok(result)
    }
}

pub fn mod_pow(base: &BigInt, exp: &BigInt, modulus: &BigInt) -> BigInt {
    let mut res = BigInt::one();
    let mut b = base % modulus;
    let mut e = exp.clone();
    let zero = BigInt::zero();
    let two = BigInt::from(2);

    while e > zero {
        if &e % &two != zero {
            res = (&res * &b) % modulus;
        }
        b = (&b * &b) % modulus;
        e /= &two;
    }

    if res.is_negative() {
        res + modulus
    } else {
        res
    }
}

pub fn is_prime(n: &BigInt) -> bool {
    let two = BigInt::from(2);
    if n < &two {
        return false;
    }
    let mut i = two;
    let limit = n.sqrt();
    while &i <= &limit {
        if n % &i == BigInt::zero() {
            return false;
        }
        i += BigInt::one();
    }
    true
}

#[cfg(test)]
tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(&BigInt::from(48), &BigInt::from(18)), BigInt::from(6));
    }

    #[test]
    fn test_mod_inverse() {
        let e = BigInt::from(17);
        let phi = BigInt::from(3120);
        let inv = mod_inverse(&e, &phi).unwrap();
        assert_eq!(inv, BigInt::from(2753));
    }

    #[test]
    fn test_mod_pow() {
        let base = BigInt::from(4);
        let exp = BigInt::from(13);
        let modulus = BigInt::from(497);
        assert_eq!(mod_pow(&base, &exp, &modulus), BigInt::from(445));
    }
}
