use self::math::modular;

use super::*;

use num_bigint::{BigInt, RandBigInt};
use std::fs::File;
use std::io::Read;

fn lehmann_test(n: &BigInt) -> bool {
    let mut rng = rand::thread_rng();
    let mut rand_numbers: Vec<BigInt> = Vec::new();
    let pow = (n - 1) / 2;

    // random 100 number
    while rand_numbers.len() < 100 {
        let rand_num = rng.gen_bigint_range(&BigInt::from(2), &(n - 1));
        if !rand_numbers.contains(&rand_num) {
            rand_numbers.push(rand_num);
        }
    }

    for i in rand_numbers {
        if math::gcd(&i, &n) > BigInt::from(1) {
            return false;
        } else {
            let x = math::fast_exponential(&i, &pow, &n);
            if x != BigInt::from(1) && x != n - 1 {
                return false;
            }
        }
    }

    true
}

pub fn is_prime(n: &BigInt) -> bool {
    if n.sign() == num_bigint::Sign::Minus {
        return false;
    }
    if n <= &BigInt::from(1) {
        return false;
    }
    if n <= &BigInt::from(3) {
        return true;
    }
    // prime number less than 100
    let small_primes = vec![
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
        97,
    ];

    for i in small_primes {
        if n == &BigInt::from(i) {
            return true;
        }
        if n % i == BigInt::from(0) {
            return false;
        }
    }

    if n <= &BigInt::from(10000) {
        let (_, num) = n.to_u32_digits();
        let lim: u32 = (num[0] as f64).sqrt() as u32;
        for i in (5..lim).step_by(6) {
            if num[0] % i == 0 || num[0] % (i + 2) == 0 {
                return false;
            }
        }
        true
    } else {
        lehmann_test(&n)
    }
}

fn read_n_bits_file(filename: &str, n: usize) -> BigInt {
    let mut file: File = File::open(filename).unwrap();
    let mut binary_string: Vec<char> = Vec::new();
    let mut shift_count: usize = 0;
    let mut buffer = vec![0u8; 1];
    while binary_string.len() < n || binary_string[binary_string.len() - n] == '0' {
        if shift_count % 8 == 0 {
            file.read_exact(&mut buffer).unwrap();
        }
        let bit_shift = (buffer[0] & 0x80) >> 7;
        buffer[0] = buffer[0] << 1;
        binary_string.push((bit_shift + 48) as char);
        shift_count += 1;
    }
    let base_index = (shift_count - n) as usize;
    let mut result: BigInt = BigInt::from(0);
    for i in 0..n {
        let index = base_index + i; // start from the least significant bit
        if binary_string[index] == '1' {
            result = result | (BigInt::from(1) << (n - i - 1));
        }
    }
    result
}

pub fn gen_prime_from_file(n: usize, filename: &str) -> BigInt {
    let max = BigInt::from(1) << n;
    let min = BigInt::from(1) << (n - 1);
    let mut num: BigInt = read_n_bits_file(filename, n);
    num = num | &BigInt::from(1); // make last bit 1
    while !is_prime(&num) {
        if num < min {
            num += min.clone();
        }
        num = modular(&(num + BigInt::from(2)), &max);
    }
    return num;
}

pub fn gen_safe_prime_from_file(n: usize, filename: &str) -> BigInt {
    let max = BigInt::from(1) << n;
    let min = BigInt::from(1) << (n - 1);
    let mut num: BigInt = read_n_bits_file(filename, n);

    num = num | &BigInt::from(3); // make last bit 11
    while !is_safe_prime(&num) {
        if num < min {
            num += min.clone();
        }
        num = modular(&(num + BigInt::from(4)), &max);
    }
    return num;
}

pub fn is_safe_prime(p: &BigInt) -> bool {
    let q = p >> 1;
    is_prime(&q) && is_prime(p)
}
