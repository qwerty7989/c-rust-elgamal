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
    pub p: BigInt,
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
    let private_key = PrivateKey { u, p: p.clone() };
    return (public_key, private_key);
}

pub fn encrypt_block(key: &PublicKey, message: &BigInt) -> Vec<(BigInt,BigInt)> {
    let keysize = util::get_most_significant_bit(&key.p);
    let blocksize = keysize - 1;

    // Block the message
    let blockfilter = util::fill_bits_one(&(BigInt::from(1) << blocksize));
    let mut blocks: Vec<BigInt> = Vec::new();
    let mut msg = message.clone();
    while msg > BigInt::from(0) {
        let block = msg.clone() & blockfilter.clone();
        blocks.push(block);
        msg = msg >> blocksize;
    }
    
    // Encrypt the blocks
    let mut encrypted_blocks: Vec<(BigInt, BigInt)> = Vec::new();
    for block in blocks {
        let (a, b) = encrypt(key, &block);
        encrypted_blocks.push((a, b));
    }
    encrypted_blocks.reverse();
    return encrypted_blocks;
}

pub fn encrypt(key: &PublicKey, message: &BigInt) -> (BigInt, BigInt) {
    if message > &key.p {
        panic!("The message is larger than the prime number");
    }

    let mut rng = rand::thread_rng();
    let upperbound = key.p.clone() - BigInt::from(1);
    let mut k: BigInt = rng.gen_bigint_range(&BigInt::from(2), &upperbound);
    while math::gcd(&k, &(key.p.clone()-1)) != BigInt::from(1) {
        k= rng.gen_bigint_range(&BigInt::from(2), &upperbound);
    }
    let a = math::fast_exponential(&key.g, &k, &key.p);
    let b = (message.clone() * math::fast_exponential(&key.y, &k, &key.p)) % key.p.clone();
    return (a, b);
}

pub fn decrypt(key: &PrivateKey, a: &BigInt, b: &BigInt) -> BigInt {
    let s = math::fast_exponential(a, &key.u, &key.p);
    let s_inv = math::find_mod_invert(&s, &key.p);
    return b * s_inv % &key.p;
}

pub fn decrypt_block(key: &PrivateKey, blocks: &Vec<(BigInt,BigInt)>) -> BigInt {
    let keysize = util::get_most_significant_bit(&key.p);
    let blocksize = keysize - 1;
    let mut message: BigInt = BigInt::from(0);
    for (a, b) in blocks {
        let decrypted_block = decrypt(key, a, b);
        message = message << blocksize;
        message = message | decrypted_block;
    }
    return message;
}
