mod model;
use model::*;
mod test;

use num_bigint::{BigInt, RandBigInt};
// use num_bigint::BigInt;
// use num_traits::pow::Pow;

fn main() {

    // Phase 1
    let file = "./UwU.jpg";
    let bits = 128;
    let start_time = std::time::Instant::now();
    let prime = prime::gen_safe_prime_from_file(bits, &file);
    let end_time = std::time::Instant::now();
    println!("time used: {:?}", end_time.duration_since(start_time));
    println!("the generated prime is {}", &prime);
    println!("test prime result: {}", prime::is_prime(&prime));
    println!("prime     : {:b};", &prime);
    println!("prime length: {}", &prime.bits());

    let (e, eiv, prime_temp) = gen_random_nowith_inverse(&prime);
    println!("generate number: {}", e);
    println!("invert of the number in mod prime: {}", eiv);
    println!("test invert number: {}", (e*eiv)%prime_temp);

    // Phase 2
    let generator = elgamal::gen_generator(&prime);
    println!("generator: {}", generator);

    let (public_key, private_key) = elgamal::gen_keys(&prime);
    println!("public key: {:?}", public_key);
    println!("private key: {:?}", private_key);

    let message = BigInt::from(123456789);
    let (a, b) = elgamal::encrypt(&public_key, &message);
    println!("encrypted message: ({}, {})", a, b);

    let decrypted_message = elgamal::decrypt(&prime, &private_key.u, &a, &b);
    println!("decrypted message: {}", decrypted_message);
}


fn gen_random_nowith_inverse(n: &BigInt) -> (BigInt,BigInt,BigInt){ 
    let mut rngs = rand::thread_rng();
    let e = rngs.gen_bigint_range(&BigInt::from(2), &(n-1));
    let eiv = math::find_mod_invert(&e, &n);
    return (e,eiv,n.clone())
}

