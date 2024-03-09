mod model;
use model::*;
// mod test;
use rand::Rng;
// use num_bigint::BigInt;
// use num_traits::pow::Pow;

fn main() {
    let file = "./demo.png";
    util::show_head_file(file, 1300);
    let bits = 1024;
    // let lowerbound: BigInt = math::fast_exponential(BigInt::from(2), bits-1);
    // let upperbound: BigInt = BigInt::from(2).pow(bits)-1;
    let prime = prime::gen_prime_from_file(bits, &file);


    println!("the generated prime is {}", prime);
    println!("test prime result: {}", prime::is_prime(&prime));
    // println!("lowerbound: {:b};", lowerbound);
    // println!("upperbound: {:b};", upperbound);
    println!("prime     : {:b};", prime);
    println!("prime length: {}", prime.bits());
    // println!("test prime value: {}", (prime>=lowerbound && prime<= upperbound));

    // let (e, eiv, prime) = gen_random_nowith_inverse(prime);
    // println!("generate number: {}", e);
    // println!("invert of the number in mod prime: {}", eiv);
    // println!("test invert number: {}", math::mul_mod(e, eiv, prime))
}

// fn gen_random_nowith_inverse(n: i128) -> (i128,i128,i128){ 
//     let mut rngs = rand::thread_rng();
//     let e = rngs.gen_range(0..n);
//     let eiv = math::find_mod_invert(e, n);
//     return (e,eiv,n)
// }

