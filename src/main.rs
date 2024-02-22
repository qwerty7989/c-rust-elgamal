fn main() {

    // println!("{}", get_first_n_bits_binary("./Cargo.lock", 1000));
    // match out{
    //     Ok(v) => println!("{}", v),
    //     Err(e) => println!("{}", e)
    // }
    println!("{}", GenPrime(8,"./Cargo.lock").unwrap());
    let (e,eiv,_) = GenRandomNowithInverse(281);
    println!("{} {}", e, eiv)
}

/// return modular in set between `[0, modular-1 ]`
pub fn modular(number: i128, modular: i128) -> i128{
    let result = number % modular;
    if result < 0 {
        result + modular
    } else {
        result
    }
}

/// return GCD between two number (a,n)
pub fn gcd(a: i128, n: i128) -> i128 {
    if n == 0 {
        return a;
    }
    return gcd( n, a%n);
}

/// return Inverse of number `a` mod `p` with Extended Euclidean algorithm
/// ```
/// a -> number
/// p -> prime number
/// ```
pub fn find_inverse( a: i128, p: i128) -> i128 {
    fn extended_gcd( n1: i128, n2: i128) -> (i128, i128) {
        // gcd = xa+yb
        if n2 == 0 {
            return (1, 0);
        }

        let (mut  x, mut y) = extended_gcd(n2, n1%n2);
        (x,y) = (y ,x -  n1/n2 * y );
        
        (x, y)
    }

    let (x,_y) = extended_gcd(a,p);
    
    return modular(x, p);
}

pub fn fast_exponential( a: i128, b: i128, n: i128 ) -> i128 {
    if b == 1 { return modular(a, n) };
    
    if b%2 == 0 {
        let x = fast_exponential(a, b/2, n);
        return modular(x*x,n)
    }
    else{
        let x = fast_exponential(a, b-1, n);
        return modular(x*a,n);
    }
}

use rand::Rng;
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
        for i in (5..lim).step_by(6){
            if n%i == 0 || n% (i+2) == 0{
                return false;
            }
        } 
        return true
    } else {

        let mut rng = rand::thread_rng();
        let mut rand_numbers: Vec<i128> = Vec::new();
        let pow = (n-1)/2 ;
        // random 100 number
        while rand_numbers.len() < 100 {
            let rand_num = rng.gen_range(2..n);
            if !rand_numbers.contains(&rand_num){
                rand_numbers.push(rand_num);
            }
        }
        
        for i in rand_numbers{
            if gcd(i, n) > 1 {return false}
            else {
                let x = fast_exponential(i, pow, n);
                if x != 1 && x!= n-1 { return false }
            }
        }

        true
    }
}

use std::fs::File;
use std::io::{Error, ErrorKind, Read};

// fn print_binary_file(filename: &str) -> Result<(), Error> {
//     let mut file = File::open(filename)?;
//     let mut buffer = Vec::new();

//     file.read_to_end(&mut buffer)?;

//     for byte in buffer {
//         print!("{:08b}", byte); // Print each byte in 8-bit binary format
//     }
//     println!();
//     Ok(())

// }

fn GenPrime(n: usize, filename: &str) -> Result<i128,Error>{
    let mut n = generate_n_bits_binary(filename, n).unwrap();
    while !is_prime(n) {
        n+=2
    }
    return Ok(n)
}

fn generate_n_bits_binary(filename: &str, n: usize) -> Result<i128, Error> {
    let mut file = File::open(filename)?;

    let mut number: i128 = 0;
    let mut shift_count = 0; 
    let mut buffer = vec![0u8;1];
    while number < (1 << n) {
        if shift_count % 8 == 0 {
            file.read_exact(&mut buffer)?;
        }
        let bit_shift = (buffer[0] & 0x80) >> 7;
        buffer[0] = buffer[0] << 1 ;
        number = (number << 1) | (bit_shift as i128);
        shift_count+=1;
    }

    Ok(number) // Return the first N bits
}

pub fn GenRandomNowithInverse(n: i128) -> (i128,i128,i128){ 
    let mut rngs = rand::thread_rng();
    let e = rngs.gen_range(0..n);
    let eiv = find_inverse(e, n);
    return (e,eiv,n)
}