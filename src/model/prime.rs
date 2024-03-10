use super::*;

use std::fs::File;
use std::io::Read;
use num_bigint::{BigInt, RandBigInt};
/// Implements the Lehmann primality test, a probabilistic test for determining if a number is prime.
///
/// The Lehmann test is based on Fermat's Little Theorem and is more reliable than simpler 
/// probabilistic tests like the Miller-Rabin test.  However, it's still probabilistic and can 
/// produce false positives (incorrectly identifying a composite number as prime), albeit with
/// very low likelihood.
///
/// # Arguments
///
/// * `n`: The integer to test for primality.
///
/// # Returns
///
/// `true` if 'n' is likely a prime number, `false` otherwise.
///
/// # Notes
/// 
///  * This test has a very low probability of false positives, but they can still occur. 
///  * For absolute certainty about primality, use a deterministic primality test.
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
            if x != BigInt::from(1)  && x != n - 1 {
                return false;
            }
        }
    }

    true
}

/// Determines if an integer is prime using a combination of deterministic and probabilistic tests.
///
/// This function employs several strategies to efficiently and accurately determine primality:
///
/// * **Small Number Checks:** Quickly eliminates small non-primes using divisibility checks.
/// * **Optimized Trial Division:**  Checks divisibility up to the square root of 'n', with an 
///    optimization for divisors that are multiples of 6 plus or minus 1.
/// * **Lehmann Test:** For larger numbers, applies the probabilistic Lehmann primality test.
///
/// # Arguments
///
/// * `n`: The integer to test for primality.
///
/// # Returns
///
/// `true` if 'n' is a prime number, `false` otherwise.
pub fn is_prime(n: &BigInt) -> bool {
    if n.sign() == num_bigint::Sign::Minus {
        return false;
    }
    if n <= &BigInt::from(1) {
        return false;
    } else if n <= &BigInt::from(3) {
        return true;
    } else if (n % 2) == BigInt::from(0) || (n % 3) == BigInt::from(0) {
        return false;
    }

    if n <= &BigInt::from(10000){
        let (_,num) = n.to_u32_digits();
        let lim:u32 = (num[0] as f64).sqrt() as u32;
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

/// Reads the specified number of bits from a binary file and returns them as an integer.
///
/// The function efficiently reads bits from a file, storing them in an integer representation. 
/// It handles bit-level operations for extracting and shifting the bits.
///
/// # Arguments
/// 
/// * `filename`: The path to the binary file.
/// * `n`: The number of bits to read from the file.
///
/// # Returns
///
/// An integer (`BigInt`) representing the first 'n' bits read from the file. 

fn read_n_bits_file(filename: &str, n: usize) -> BigInt{
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

/// Generates a prime number by reading bits from a binary file and applying primality testing. 
///
/// The function works as follows:
/// 1. **Read bits:** Reads a specified number of bits from the file.
/// 2. **Ensure oddness:** Sets the least significant bit to 1, guaranteeing an odd number.
/// 3. **Establish upper bound:** Calculates an upper bound for efficient searching.
/// 4. **Primality testing and incrementing:**  Iterates through odd numbers, testing for primality  
///    using `is_prime` until a prime number is found.
///
/// # Arguments
///
/// * `n`: The number of bits to read from the file (determines the size of the prime).
/// * `filename`: The path to the binary file containing random bits.
///
/// # Returns
/// 
/// A prime number generated from the bits in the file. 
pub fn gen_prime_from_file(n: usize, filename: &str) -> BigInt {
    let mut num:BigInt = read_n_bits_file(filename, n);
    num = num | BigInt::from(3); 

    while !is_prime(&num) || !is_safe_prime(&num) {
        num += BigInt::from(4);
    }
    return num;
}

fn is_safe_prime(p: &BigInt) -> bool {
    let q = (p - 1) / 2;
    is_prime(&q) && is_prime(p)
}