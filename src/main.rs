mod math;
mod prime;
mod util;
mod test;
use rand::Rng;

fn main() {
    let file = "./demo.png";
    util::show_head_file(file, 18);
    let bits = 126;
    let lowerbound : i128 = 1<<(bits-1);
    let upperbound: i128 = (1<<bits) - 1;
    let prime = prime::gen_prime_from_file(bits, &file);


    println!("the generated prime is {}", prime);
    println!("test prime result: {}", prime::is_prime(prime));
    println!("lowerbound: {:b};", lowerbound);
    println!("upperbound: {:b};", upperbound);
    println!("prime     : {:b};", prime);
    println!("test prime value: {}", (prime>=lowerbound && prime<= upperbound));

    let (e, eiv, prime) = gen_random_nowith_inverse(prime);
    println!("generate number: {}", e);
    println!("invert of the number in mod prime: {}", eiv);
    println!("test invert number: {}", math::mul_mod(e, eiv, prime))
}

fn gen_random_nowith_inverse(n: i128) -> (i128,i128,i128){ 
    let mut rngs = rand::thread_rng();
    let e = rngs.gen_range(0..n);
    let eiv = math::find_mod_invert(e, n);
    return (e,eiv,n)
}