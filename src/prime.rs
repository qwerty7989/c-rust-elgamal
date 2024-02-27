use super::*;
use rand::Rng;
use std::fs::File;
use std::io::Read;

fn lehmann_test(n: i128) -> bool {
    let mut rng = rand::thread_rng();
    let mut rand_numbers: Vec<i128> = Vec::new();
    let pow = (n - 1) / 2;

    // random 100 number
    while rand_numbers.len() < 100 {
        let rand_num = rng.gen_range(2..n);
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

pub fn gen_prime_from_file(n: usize, filename: &str) -> i128 {
    let mut n = read_n_bits_file(filename, n);
    if n & 0b1 == 0 {
        n+=1;
    }
    // this still bug if read bit is 1111111
    while !is_prime(n) {
        n += 2;
    }
    return n;
}
