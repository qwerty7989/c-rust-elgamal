use num_bigint::BigInt;
use base64::{Engine as _, engine::general_purpose};

pub fn byte_to_bigint(byte: &[u8]) -> BigInt {
    return BigInt::from_bytes_le(num_bigint::Sign::Plus, byte);
}


pub fn bigint_to_byte(n: &BigInt) -> Vec<u8> {
    return n.to_bytes_le().1;
}


pub fn bigint_to_base64(n: &BigInt) -> String {
    // let str = n.to_string();
    // return general_purpose::STANDARD.encode(str.as_bytes());

    let byte = n.to_bytes_le().1;
    return general_purpose::STANDARD.encode(&byte);
}

pub fn base64_to_bigint(base64: String) -> BigInt {
    // let byte = general_purpose::STANDARD.decode(base64).unwrap();
    // let str = std::str::from_utf8(&byte).unwrap();
    // return BigInt::parse_bytes(str.as_bytes(), 10).unwrap();

    let byte = general_purpose::STANDARD.decode(base64).unwrap();
    return BigInt::from_bytes_le(num_bigint::Sign::Plus, &byte);
}