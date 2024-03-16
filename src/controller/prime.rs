
use super::super::model::prime;
use num_bigint::BigInt;
pub fn prime_gen(file_path: &String, bits_length: usize, safe: bool) -> BigInt {
    println! {"{} {} {}", file_path, bits_length, safe};
    let prime: BigInt;
    if safe{
        prime = prime::gen_safe_prime_from_file(bits_length, file_path);
    } else {
        prime = prime::gen_prime_from_file(bits_length, file_path);
    }
    prime
}