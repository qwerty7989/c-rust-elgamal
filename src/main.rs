mod controller;
mod model;
mod view;
use model::*;
use std::env;
mod test;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} [command]", args[0]);
        return;
    }
    let command = &args[1];
    match command.as_str() {
        "prime-gen" => {
            view::prime::prime_gen(
                args[2..].to_vec(),
                controller::prime::prime_gen,
            );
        }
        "elgamal-keygen" => {
            view::elgamal::elgamal_keygen(
                args[2..].to_vec(),
                controller::elgamal::keygen,
            );
        }
        "elgamal-encrypt" => {
            view::elgamal::elgamal_encrypt(
                args[2..].to_vec(),
                controller::elgamal::elgamal_encrypt,
            );
        }
        "elgamal-decrypt" => {
            view::elgamal::elgamal_decrypt(
                args[2..].to_vec(),
                controller::elgamal::elgamal_decrypt,
            );
        }
        _ => {
            println!("Command {} not found", command);
        }
    }
}
