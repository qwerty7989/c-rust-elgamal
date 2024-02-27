use std::fs::File;
use std::io::Read;

pub fn full_one(n:u8)-> i128{
    if n == 127 {
        return 0x7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF
    }
    return (1 << n) - 1;
}

/// print binary of file b bytes
pub fn show_head_file(filename :&str,b: usize) {
    let mut file = File::open(filename).unwrap();
    let mut buffer = vec![0u8; b];
    file.read_exact(&mut buffer).unwrap();
    println!("binary {} bytes of {}", b,filename);
    for i in buffer {
        print!("{:08b}", i);
    }
    println!();
}

pub fn upperbound(n:i128) -> i128{
    return (n | n - 1) + 1
}


// pub fn lowerbound(n: i128) -> i128{

// }

