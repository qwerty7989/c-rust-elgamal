mod model;
use std::{env, fs};

use model::*;
mod test;

use num_bigint::BigInt;


fn main() {
    let args = env::args().collect::<Vec<String>>();

    let command = &args[1];

    match command.as_str() {
        "prime-gen" => {
            let file_path: &String;
            let bits_length: usize;
            let safe_prime: bool;
            let output_file: &String;
            if args.contains(&String::from("--file")) {
                let index = args.iter().position(|r| r == "--file").unwrap();
                file_path = &args[index + 1];
            } else {
                panic!("Error not found flag \"--file <file_path>\"");
            }
            if args.contains(&String::from("--bits")) {
                let index = args.iter().position(|r| r == "--bits").unwrap();
                bits_length = args[index + 1].parse::<usize>().unwrap();
            } else {
                panic!("Error not found flag \"--bits <bits_length>\"");
            }
            if args.contains(&String::from("--safe")) {
                safe_prime = true;
            } else {
                safe_prime = false;
            }
            if args.contains(&String::from("--output")) {
                let index = args.iter().position(|r| r == "--output").unwrap();
                output_file = &args[index + 1];
            } else {
                panic!("Error not found flag \"--output <output_file_path>\"");
            }
            // if args contains "--safe" then use safe prime
            let prime: BigInt;
            if safe_prime{
                prime = prime::gen_safe_prime_from_file(bits_length, file_path);
            } else {
                prime = prime::gen_prime_from_file(bits_length, file_path);
            }
            fs::write(output_file, prime.to_str_radix(10)).expect("Unable to write file");
        }
        "elgamal-keygen" => {
            let file_path: &String;
            let bits_length: usize;
            let pk_file: &String;
            let sk_file: &String;
            if args.contains(&String::from("--file")) {
                let index = args.iter().position(|r| r == "--file").unwrap();
                file_path = &args[index + 1];
            } else {
                panic!("Error not found flag \"--file <file_path>\"");
            }
            if args.contains(&String::from("--bits")) {
                let index = args.iter().position(|r| r == "--bits").unwrap();
                bits_length = args[index + 1].parse::<usize>().unwrap();
            } else {
                panic!("Error not found flag \"--bits <bits_length>\"");
            }
            if args.contains(&String::from("--pk-file")) {
                let index = args.iter().position(|r| r == "--pk-file").unwrap();
                pk_file = &args[index + 1];
            } else {
                panic!("Error not found flag \"--pk-file <pk_file_path>");   
            }
            if args.contains(&String::from("--sk-file")) {
                let index = args.iter().position(|r| r == "--sk-file").unwrap();
                sk_file = &args[index + 1];
            } else {
                panic!("Error not found flag \"--sk-file <sk_file_path>");   
            }

            let prime = prime::gen_safe_prime_from_file(bits_length, file_path);
            let (public_key, private_key) = elgamal::gen_keys(&prime);
            let format_pk = format::public_key_format(&public_key);
            let format_sk = format::private_key_format(&private_key);
            fs::write(pk_file, format_pk).expect("Unable to write file");
            fs::write(sk_file, format_sk).expect("Unable to write file");
        }
        "elgamal-encrypt" => {
            let file_path: &String;
            let pk_file: &String;
            let encrypted_file: &String;
            if args.contains(&String::from("--file")) {
                let  index = args.iter().position(|r| r == "--file").unwrap();
                file_path = &args[index + 1];
            } else {
                panic!("Error not found flag \"--file <file_path>\"");   
            }
            if args.contains(&String::from("--pk-file")) {
                let index = args.iter().position(|r| r == "--pk-file").unwrap();
                pk_file = &args[index + 1];
            } else {
                panic!("Error not found flag \"--pk-file <pk_file_path>\"");   
            }
            if args.contains(&String::from("--encrypted-file")) {
                let index = args.iter().position(|r| r == "--encrypted-file").unwrap();
                encrypted_file = &args[index + 1];
            } else {
                panic!("Error not found flag \"--encrypted-file <encrypted_file_path>\"");   
            }

            // if contains --pk-file then read from file
            let public_key: elgamal::PublicKey = format::read_public_key_format(&fs::read_to_string(pk_file).unwrap());
            let (_,message): (fs::Metadata, Vec<u8>) = util::read_file_bytes(file_path);
            let bigint_message = parse::byte_to_bigint(&message);
            let encrypted_blocks: Vec<(BigInt, BigInt)> = elgamal::encrypt_block(&public_key, &bigint_message);
            let format_encrypted_blocks = format::encryption_format(&public_key, &encrypted_blocks);
            fs::write(encrypted_file, format_encrypted_blocks).expect("Unable to write file");

        }
        "elgamal-decrypt" => {
            let file_path: &String;
            let sk_file: &String;
            let decrypted_file: &String;
            if args.contains(&String::from("--file")) {
                let  index = args.iter().position(|r| r == "--file").unwrap();
                file_path = &args[index + 1];
            } else {
                panic!("Error not found flag \"--file <file_path>\"");   
            }
            if args.contains(&String::from("--sk-file")) {
                let index = args.iter().position(|r| r == "--sk-file").unwrap();
                sk_file = &args[index + 1];
            } else {
                panic!("Error not found flag \"--sk-file <sk_file_path>\"");   
            }
            if args.contains(&String::from("--decrypted-file")) {
                let index = args.iter().position(|r| r == "--decrypted-file").unwrap();
                decrypted_file = &args[index + 1];
            } else {
                panic!("Error not found flag \"--decrypted-file <decrypted_file_path>\"");   
            }
            let private_key = format::read_private_key_format(&fs::read_to_string(sk_file).unwrap());
            let encrypted_blocks: Vec<(BigInt, BigInt)> = format::read_encryption_format(&private_key, &util::read_file_bytes(file_path).1);
            let decrypted_message: BigInt = elgamal::decrypt_block(&private_key, &encrypted_blocks);
            let format_decrypted_message = parse::bigint_to_byte(&decrypted_message);
            fs::write(decrypted_file, format_decrypted_message).expect("Unable to write file");
        }
        _ => {
            println!("Command not found");
        }
    }
}
