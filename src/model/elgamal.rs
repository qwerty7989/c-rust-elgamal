use num_bigint::{BigInt, RandBigInt};

use super::*;

#[derive(Debug)]
pub struct PublicKey {
    pub p: BigInt,
    pub g: BigInt,
    pub y: BigInt,
}

#[derive(Debug)]
pub struct PrivateKey {
    pub u: BigInt,
}

pub fn gen_generator(prime: &BigInt) -> BigInt {
    if !prime::is_safe_prime(prime){
        panic!("The prime number is not a safe prime");
    }
    //random generator
    let mut rng = rand::thread_rng();
    let g: BigInt = rng.gen_bigint_range(&BigInt::from(2), &(prime - 1));
    if check_generator(prime, &g) {
        return g;
    }else{
        return prime - g;
    }
}

fn check_generator(prime: &BigInt, g: &BigInt) -> bool {
    let power : BigInt = (prime - 1)/2;
    if math::fast_exponential(g, &power, prime) != BigInt::from(1) {
        return true;
    }
    return false;
}

pub fn gen_keys(p: &BigInt) -> (PublicKey, PrivateKey) {
    let mut rng = rand::thread_rng();
    let g = gen_generator(p);
    let u: BigInt = rng.gen_bigint_range(&BigInt::from(2), &(p - 1));
    
    let y = math::fast_exponential(&g, &u, p);
    
    let public_key = PublicKey { p: p.clone(), g, y };
    let private_key = PrivateKey { u };
    return (public_key, private_key);
}

pub fn encrypt(key: &PublicKey, message: &BigInt) -> (BigInt, BigInt) {
    let mut rng = rand::thread_rng();
    let upperbound = key.p.clone() - BigInt::from(1);
    let k: BigInt = rng.gen_bigint_range(&BigInt::from(2), &upperbound);
    let a = math::fast_exponential(&key.g, &k, &key.p);
    let b = (message.clone() * math::fast_exponential(&key.y, &k, &key.p)) % key.p.clone();
    return (a, b);
}


pub fn decrypt(p: &BigInt, x: &BigInt, a: &BigInt, b: &BigInt) -> BigInt {
    let s = math::fast_exponential(a, x, p);
    let s_inv = math::find_mod_invert(&s, p);
    return b.clone() * s_inv % p;
}