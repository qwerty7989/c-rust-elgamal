
use num_bigint::BigInt;

use self::elgamal::PublicKey;

use super::*;
pub fn public_key_format(key: &elgamal::PublicKey) -> String {
    // format string
    let n = util::get_most_significant_bit(&key.p);
    
    let format_key = &key.p << (n*2)| &key.g << n | &key.y;
    let base64_key = parse::bigint_to_base64(&format_key);
    let context = format!("{} {}", n, base64_key);

    return context
}

pub fn read_public_key_format(context: &str) -> elgamal::PublicKey {
    let mut iter = context.split_whitespace();
    let n: u32 = iter.next().unwrap().parse::<u32>().unwrap();
    let base64_key = iter.next().unwrap();

    // binary version
    let key = parse::base64_to_bigint(base64_key.to_owned());
    let mask = util::fill_bits_one(&(BigInt::from(1) << n-1));
    let p = &key >> (n*2);
    let g = (&key >> n) & util::fill_bits_one(&mask);
    let y = &key & util::fill_bits_one(&mask);

    return elgamal::PublicKey { p, g, y };
}

pub fn private_key_format(key: &elgamal::PrivateKey) -> String {
    // format string
    let n = util::get_most_significant_bit(&key.p);

    // binary version
    let format_key = &key.p << n | &key.u;
    let base64_key = parse::bigint_to_base64(&format_key);

    let context = format!("{} {}", n, base64_key);
    return context
}

pub fn read_private_key_format(context: &str) -> elgamal::PrivateKey {
    let mut iter = context.split_whitespace();
    let n: u32 = iter.next().unwrap().parse::<u32>().unwrap();
    let base64_key = iter.next().unwrap();

    // binary version
    let key = parse::base64_to_bigint(base64_key.to_owned());
    let mask = util::fill_bits_one(&(BigInt::from(1) << n-1));
    let p = &key >> n;
    let u = &key & util::fill_bits_one(&mask);

    return elgamal::PrivateKey { u, p };
}


pub fn encryption_format(key:&PublicKey,encrypted_block: &Vec<(BigInt,BigInt)>) -> Vec<u8> {
    let n: u32 = util::get_most_significant_bit(&key.p);

    let mut concat_bit = BigInt::from(0);

    for block in encrypted_block {
        let (c1,c2) = block;
        concat_bit = concat_bit << n*2 | c1 << n | c2;
    }

    let encrypted_message = parse::bigint_to_byte(&concat_bit);
    return encrypted_message;
}

pub fn read_encryption_format(key: &elgamal::PrivateKey, encrypted_message: &Vec<u8>) -> Vec<(BigInt,BigInt)> {
    let mut bigint_message = parse::byte_to_bigint(encrypted_message);
    let mut encrypted_block: Vec<(BigInt,BigInt)> = Vec::new();
    let mask = util::fill_bits_one(&(BigInt::from(1) << key.p.bits()-1));
    while bigint_message > BigInt::from(0) {
        let c2 = &bigint_message & &mask;
        bigint_message = bigint_message >> key.p.bits();
        let c1 = &bigint_message & &mask;
        bigint_message = bigint_message >> key.p.bits();
        let block = (c1,c2);
        encrypted_block.push(block);
    }
    encrypted_block.reverse();
    return encrypted_block;
}
