use crate::parse;

use super::super::model::elgamal;
use super::super::model::format;
use super::super::model::prime;
use super::super::model::util;
use num_bigint::BigInt;
use std::fs;

pub fn keygen(prime_in: &BigInt, file_path: &String, bits_length: usize) -> (String, String) {
    let mut prime = prime_in.clone();
    // if prime is not defined, generate prime number
    if prime == BigInt::from(0) {
        prime = prime::gen_safe_prime_from_file(bits_length, file_path);
    }

    let (public_key, private_key) = elgamal::gen_keys(&prime);
    let format_pk = format::public_key_format(&public_key);
    let format_sk = format::private_key_format(&private_key);

    return (format_pk, format_sk);
}

pub fn elgamal_encrypt(pk_str: &String, paintext_file: &String, encrypted_file: &String) {
    let public_key: elgamal::PublicKey = format::read_public_key_format(pk_str);
    let (_, message): (fs::Metadata, Vec<u8>) = util::read_file_bytes(paintext_file);
    let bigint_message = parse::byte_to_bigint(&message);
    let encrypted_blocks: Vec<(BigInt, BigInt)> =
        elgamal::encrypt_block(&public_key, &bigint_message);
    let format_encrypted_blocks = format::encryption_format(&public_key, &encrypted_blocks);
    fs::write(encrypted_file, format_encrypted_blocks).expect("Unable to write file");
}

pub fn elgamal_decrypt(sk_str: &String, encrypted_file: &String, decrypted_file: &String) {
    let private_key = format::read_private_key_format(sk_str);
    let encrypted_blocks: Vec<(BigInt, BigInt)> =
        format::read_encryption_format(&private_key, &util::read_file_bytes(encrypted_file).1);
    let decrypted_message: BigInt = elgamal::decrypt_block(&private_key, &encrypted_blocks);
    let format_decrypted_message = parse::bigint_to_byte(&decrypted_message);
    fs::write(decrypted_file, format_decrypted_message).expect("Unable to write file");
}
