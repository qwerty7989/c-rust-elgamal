use std::fs::{File, Metadata};
use std::fs;
use std::io::Read;

use num_bigint::BigInt;

/// Sets all bits of a binary string to 1.
/// 
/// The function efficiently finds the most significant bit position, then fills the integer with 
/// ones by repeatedly shifting and setting the least significant bit.
///
/// # Arguments
///
/// * `n`: The binary string to modify.
///
/// # Returns
///
/// The binary string `n` with all bits set to 1.
///
/// # Example
/// 
/// ```
/// let result = fill_bits_one("1001");  // Binary:1001  
/// assert_eq!(result, "1111");         // Binary: 1111
/// ``` 
pub fn fill_bits_one(n: &BigInt) -> BigInt {
    let mut n_clone = n.clone();
    let mut shift_count: u32 = 0;
    while n_clone > BigInt::from(1) {
        shift_count+=1;
        n_clone = n_clone >> 1;
    }

    while shift_count > 0{
        n_clone = n_clone << 1;
        n_clone = n_clone | BigInt::from(1);
        shift_count -= 1;
    }

    return n_clone
}

pub fn get_most_significant_bit(n: &BigInt) -> u32 {
    let mut n_clone = n.clone();
    let mut shift_count: u32 = 0;
    while n_clone > BigInt::from(0) {
        shift_count+=1;
        n_clone = n_clone >> 1;
    }
    return shift_count;
}

pub fn read_file_bytes(filename: &str) -> (Metadata,Vec<u8>) {
    let mut f = File::open(&filename).expect("no file found");
    let metadata: fs::Metadata = fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    return (metadata,buffer);
}