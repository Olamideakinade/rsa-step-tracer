use num_bigint::BigInt;
use num_traits::{One, Zero};
crate::math::{gcd, mod_inverse, mod_pow, is_prime};

pub struct RsaKeypair {
    pub n: BigInt,
    pub e: BigInt,
    pub d: BigInt,
}

pub fn generate_keypair(p: &BigInt, q: &BigInt, e: &BigInt) -> Result<RsaKeypair, String> {
    if !is_prime(p) {
        return Err(format!("p = {} is not a prime number.", p));
    }
    if !is_prime(q) {
        return Err(format!("q = {} is not a prime number.", q));
    }

    let n = p * q;
    let one = BigInt::one();
    let p_sub = p - &one;
    let q_sub = q - &one;
    let phi = &p_sub * &q_sub;

    let g = gcd(e, &phi);
    if g != one {
        return Err(format!("e = {} and phi(n) = {} are not coprime (gcd = {}).", e, phi, g));
    }

    let d = mod_inverse(e, &phi)?;

    Ok(RsaKeypair { n, e: e.clone(), d })
}

pub fn encrypt(msg: &BigInt, e: &BigInt, n: &BigInt) -> BigInt {
    mod_pow(msg, e, n)
}

pub fn decrypt(cipher: &BigInt, d: &BigInt, n: &BigInt) -> BigInt {
    mod_pow(cipher, d, n)
}
