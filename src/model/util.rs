use std::fs::File;
use std::io::Read;

/// Prints the first 'b' bytes of a file in binary format.
///
/// This function opens a file, reads a specified number of bytes, and displays their binary
/// representation.
///
/// # Arguments
///
/// * `filename`: The path to the file.
/// * `b`: The number of bytes to read and display in binary.
///
/// # Example
///
/// ```
/// show_head_file("my_document.txt", 10); 
/// // This might print something like:
/// // binary 10 bytes of my_document.txt
/// // 01010100 01100101 01111000 01110100 ... 
/// ```
pub fn show_head_file(filename :&str,b: usize) {
    let mut file = File::open(filename).unwrap();
    let mut buffer: Vec<u8> = vec![0u8; b];

    file.read_exact(&mut buffer).unwrap();
    println!("binary {} bytes of {}", b,filename);

    for i in buffer {
        print!("{:08b}", i);
    }
    println!();
}

/// Sets all bits of an integer to 1.
/// 
/// The function efficiently finds the most significant bit position, then fills the integer with 
/// ones by repeatedly shifting and setting the least significant bit.
///
/// # Arguments
///
/// * `n`: The integer to modify.
///
/// # Returns
///
/// The integer `n` with all bits set to 1.
///
/// # Example
/// 
/// ```
/// let result = fill_bits_one(9);  // Binary: 1001 
/// assert_eq!(result, 15);         // Binary: 1111
/// ``` 
pub fn fill_bits_one(mut n:i128) -> i128{
    let mut shift_count: u8 = 0;
    while n != 1 {
        shift_count+=1;
        n = n >> 1
    }

    while shift_count > 0{
        n = (n << 1) | 1;
        shift_count -= 1;
    }

    return n
}

/// Sets all bits of an integer to 0, except for the most significant bit.
///
/// The function finds the most significant bit position and then repeatedly shifts left, clearing  
/// bits until only the leading '1' remains.
///
/// # Arguments
///
/// * `n`: The integer to modify.
///
/// # Returns
///
/// The integer `n` with only its most significant bit set to 1.
///
/// # Example
/// 
/// ```
/// let result = fill_bits_zero(13); // Binary: 1101
/// assert_eq!(result, 8);           // Binary: 1000
/// ```
pub fn fill_bits_zero(mut n: i128) -> i128{
    let mut shift_count: u8 = 0;
    while n != 1 {
        shift_count+=1;
        n = n >> 1
    }

    while shift_count > 0{
        n = n << 1;
        shift_count -= 1;
    }

    return n
}

