mod model;
use model::*;
mod test;

use num_bigint::{BigInt, RandBigInt};
// use num_bigint::BigInt;
// use num_traits::pow::Pow;

fn main() {
    phase1()
}

fn phase1(){
    let file = "./foo";
    let bits = 3000;
    let start_time = std::time::Instant::now();
    let prime = prime::gen_prime_from_file(bits, &file);
    let end_time = std::time::Instant::now();
    println!("time used: {:?}", end_time.duration_since(start_time));
    println!("the generated prime is {}", prime);
    println!("test prime result: {}", prime::is_prime(&prime));
    println!("prime     : {:b};", prime);
    println!("prime length: {}", prime.bits());

    let (e, eiv, prime) = gen_random_nowith_inverse(&prime);
    println!("generate number: {}", e);
    println!("invert of the number in mod prime: {}", eiv);
    println!("test invert number: {}", (e*eiv)%prime);
}

fn gen_random_nowith_inverse(n: &BigInt) -> (BigInt,BigInt,BigInt){ 
    let mut rngs = rand::thread_rng();
    let e = rngs.gen_bigint_range(&BigInt::from(2), &(n-1));
    let eiv = math::find_mod_invert(&e, &n);
    return (e,eiv,n.clone())
}

