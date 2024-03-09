use super::*;
use rand::Rng;
use std::fs::File;
use std::io::Read;

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
fn lehmann_test(n: i128) -> bool {
    let mut rng = rand::thread_rng();
    let mut rand_numbers: Vec<i128> = Vec::new();
    let pow = (n - 1) / 2;

    // random 100 number
    while rand_numbers.len() < 100 {
        let rand_num = rng.gen_range(2..n-2);
        if !rand_numbers.contains(&rand_num) {
            rand_numbers.push(rand_num);
        }
    }

    for i in rand_numbers {
        if math::gcd(i, n) > 1 {
            return false;
        } else {
            let x = math::fast_exponential(i, pow, n);
            if x != 1 && x != n - 1 {
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
pub fn is_prime(n: i128) -> bool {
    if n <= 1 {
        return false;
    } else if n <= 3 {
        return true;
    } else if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    if n <= 10000 {
        let lim: i128 = (n as f64).sqrt() as i128;
        for i in (5..lim).step_by(6) {
            if n % i == 0 || n % (i + 2) == 0 {
                return false;
            }
        }
        return true;
    } else {
        return lehmann_test(n);
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
/// An integer (`i128`) representing the first 'n' bits read from the file. 

fn read_n_bits_file(filename: &str, n: usize) -> i128{
    let mut file = File::open(filename).unwrap();
    let mut number: i128 = 0;
    let mut shift_count : u8 = 0;
    let mut buffer = vec![0u8; 1];
    while number < (1 << (n-1)) {
        if shift_count % 8 == 0 {
            file.read_exact(&mut buffer).unwrap();
            shift_count = 0;
        }
        let bit_shift = (buffer[0] & 0x80) >> 7;
        buffer[0] = buffer[0] << 1;
        number = (number << 1) | (bit_shift as i128);
        shift_count += 1;
    }
    number // Return the first N bits
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
pub fn gen_prime_from_file(n: usize, filename: &str) -> i128 {
    let mut n: i128 = read_n_bits_file(filename, n);
    if n & 0b1 == 0 {
        n+=1;
    }

    let upperbound = util::fill_bits_one(n);
    while !is_prime(n) {
        if n == upperbound { // this handle filled bits 1
            n = util::fill_bits_zero(n) | 1 ;
        }
        n += 2;
    }
    return n;
}
