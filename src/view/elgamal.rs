use super::super::model::prime;
use super::util;
use num_bigint::BigInt;
use std::fs;
pub fn elgamal_keygen(
    args: Vec<String>,
    genprime_control: fn(&BigInt, &String, usize) -> (String, String),
) {
    // if contains flag help
    if args.contains(&String::from("--help")) || args.contains(&String::from("-h")) {
        println!("Usage: elgamal-keygen [options]");
        println!("Options:");
        println!("\t--prime <prime_number>\t\t\tPrime number");
        println!("\t--file, -f <file_path>\t\t\tFile path of seed number");
        println!("\t--bits, -b <bits_length>\t\tKey length");
        println!("\t--pk-file <public_key_file_path>\t\tFile path to write public key");
        println!("\t--sk-file <secret_key_file_path>\t\tFile path to write secret key");
        return;
    }

    let mut file_path: String = String::new();
    let mut bits_length: usize = 0;
    let mut prime_in: BigInt = BigInt::from(0);
    let mut pk_file: String = String::new();
    let mut sk_file: String = String::new();

    if args.contains(&String::from("--prime")) {
        let index = args.iter().position(|r| r == "--prime").unwrap();
        let prime_str = args[index + 1].clone();
        prime_in = prime_str.parse::<BigInt>().unwrap();
        if !prime::is_safe_prime(&prime_in) {
            panic!("Error: not safe prime number");
        }
    } else {
        println!("Generate New Prime Number");
        if args.contains(&String::from("--file")) || args.contains(&String::from("-f")) {
            let index = args
                .iter()
                .position(|r| r == "--file" || r == "-f")
                .unwrap();
            file_path = args[index + 1].clone();
        } else {
            // print input console for file path
            util::input_line("Enter file path: ", &mut file_path);
        }

        if args.contains(&String::from("--bits")) || args.contains(&String::from("-b")) {
            let index = args.iter().position(|r| r == "--bits" || r == "-b").unwrap();
            bits_length = args[index + 1].parse::<usize>().unwrap();
        } else {
            let mut bits_length_str = String::new();
            util::input_line("Enter bits length: ", &mut bits_length_str);
            bits_length = bits_length_str.parse::<usize>().unwrap();
        }
    }

    if args.contains(&String::from("--pk-file")) {
        let index = args.iter().position(|r| r == "--pk-file").unwrap();
        pk_file = args[index + 1].clone();
    } else {
        util::input_line("Enter file name to save public key: ", &mut pk_file);
    }
    if args.contains(&String::from("--sk-file")) {
        let index = args.iter().position(|r| r == "--sk-file").unwrap();
        sk_file = args[index + 1].clone();
    } else {
        util::input_line("Enter file name to save secret key: ", &mut sk_file);
    }

    let (public_key, private_key) = genprime_control(&prime_in, &file_path, bits_length);

    fs::write(pk_file, public_key).expect("Unable to write file");
    fs::write(sk_file, private_key).expect("Unable to write file");
}

pub fn elgamal_encrypt(args: Vec<String>, encrypt_control: fn(&String, &String, &String)) {
    // if contains flag help
    if args.contains(&String::from("--help")) || args.contains(&String::from("-h")) {
        println!("Usage: elgamal-encrypt [options]");
        println!("Options:");
        println!("\t--pk-in\t\t\t\t\tEnter public key");
        println!("\t--pk-file <public_key_file_path>\t\tFile path of public key");
        println!("\t--file <paintext_file_path>\t\tFile path of paintext");
        println!("\t--output <encrypted_file_path>\t\tFile path to write encrypted file");
        return;
    }

    let mut pk_str: String = String::new();
    let mut paintext_file: String = String::new();
    let mut encrypted_file: String = String::new();

    if args.contains(&String::from("--pk-in")) {
        util::input_line("Enter public key: ", &mut pk_str);
    } else {
        let mut pk_file: String = String::new();
        if args.contains(&String::from("--pk-file")) {
            let index = args.iter().position(|r| r == "--pk-file").unwrap();
            pk_file = args[index + 1].clone();
        } else {
            util::input_line("Enter file name of public key: ", &mut pk_file);
        }

        pk_str = fs::read_to_string(pk_file).expect("Unable to read file");
    }

    if args.contains(&String::from("--file")) {
        let index = args.iter().position(|r| r == "--file").unwrap();
        paintext_file = args[index + 1].clone();
    } else {
        util::input_line("Enter file name of paintext: ", &mut paintext_file);
    }

    if args.contains(&String::from("--output")) {
        let index = args.iter().position(|r| r == "--output").unwrap();
        encrypted_file = args[index + 1].clone();
    } else {
        util::input_line(
            "Enter file name to save encrypted file: ",
            &mut encrypted_file,
        );
    }

    encrypt_control(&pk_str, &paintext_file, &encrypted_file);
}

pub fn elgamal_decrypt(args: Vec<String>, decrypt_control: fn(&String, &String, &String)) {
    
    // if contains flag help
    if args.contains(&String::from("--help")) || args.contains(&String::from("-h")) {
        println!("Usage: elgamal-decrypt [options]");
        println!("Options:");
        println!("\t--sk-in\t\t\t\t\tEnter secret key");
        println!("\t--sk-file <secret_key_file_path>\t\tFile path of secret key");
        println!("\t--file <encrypted_file_path>\t\tFile path of encrypted file");
        println!("\t--output <decrypted_file_path>\t\tFile path to write decrypted file");
        return;
    }

    let mut sk_str: String = String::new();
    let mut encrypted_file: String = String::new();
    let mut decrypted_file: String = String::new();

    if args.contains(&String::from("--sk-in")) {
        util::input_line("Enter secret key: ", &mut sk_str);
    } else {
        let mut sk_file: String = String::new();
        if args.contains(&String::from("--sk-file")) {
            let index = args.iter().position(|r| r == "--sk-file").unwrap();
            sk_file = args[index + 1].clone();
        } else {
            util::input_line("Enter file name of secret key: ", &mut sk_file);
        }

        sk_str = fs::read_to_string(sk_file).expect("Unable to read file");
    }

    if args.contains(&String::from("--file")) {
        let index = args.iter().position(|r| r == "--file").unwrap();
        encrypted_file = args[index + 1].clone();
    } else {
        util::input_line("Enter file name of encrypted file: ", &mut encrypted_file);
    }

    if args.contains(&String::from("--output")) {
        let index = args.iter().position(|r| r == "--output").unwrap();
        decrypted_file = args[index + 1].clone();
    } else {
        util::input_line(
            "Enter file name to save decrypted file: ",
            &mut decrypted_file,
        );
    }

    decrypt_control(&sk_str, &encrypted_file, &decrypted_file);
}
