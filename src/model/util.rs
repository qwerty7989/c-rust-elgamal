use std::fs::File;
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
pub fn fill_bits_one(n: &str) -> String {
    let mut result = String::new();
    let mut shift_count: u8 = 0;
    let mut n = n.len() as u8;
    while n != 1 {
        shift_count+=1;
        n = n >> 1
    }

    while shift_count > 0{
        result.push('1');
        shift_count -= 1;
    }

    return result
}
/// Sets all bits of a binary string to 0, except for the most significant bit.
///
/// The function finds the most significant bit position and then repeatedly shifts left, clearing  
/// bits until only the leading '1' remains.
///
/// # Arguments
///
/// * `n`: The binary string to modify.
///
/// # Returns
///
/// The binary string `n` with all bits set to 0, except for the most significant bit.
///
/// # Example
/// 
/// ```
/// let result = fill_bits_zero("1011");  // Binary:1011
/// assert_eq!(result, "1000");           // Binary:1000
/// ```
pub fn fill_bits_zero(n: &str) -> String {
    let mut result = String::new();
    let mut shift_count: u8 = 0;
    let mut n = n.len() as u8;
    while n != 1 {
        shift_count+=1;
        n = n >> 1
    }

    while shift_count > 0{
        result.push('0');
        shift_count -= 1;
    }
    result.push('1');
    return result
}

/// Converts a `BigInt` to a binary string.
pub fn bigint_to_binary_string(n: &BigInt) -> String {
    let (_,num) = n.to_u32_digits();
    let mut result = String::new();
    for i in num {
        result.push_str(&format!("{:032b}", i));
    }
    result
}

/// Converts a binary string to a `BigInt`.
pub fn binary_string_to_bigint(s: &str) -> BigInt {
    let mut result = BigInt::from(0);
    for c in s.chars() {
        result = result << 1;
        if c == '1' {
            result = result | BigInt::from(1);
        }
    }
    result
}
