use num_bigint::BigInt;
use std::fs;
use super::util;
pub fn prime_gen(args: Vec<String>, genprime_control: fn(&String, usize, bool) -> BigInt) {
    // if contains flag help
    if args.contains(&String::from("--help")) || args.contains(&String::from("-h")) {
        println!("Usage: prime-gen [options]");
        println!("Options:");
        println!("\t--file, -f <file_path>\t\t\tFile path of seed number");
        println!("\t--lenght, -l <bits_length>\t\tBits length of prime number");
        println!("\t--safe\t\t\t\t\tGenerate safe prime number");
        println!("\t--output, -o <output_file_path>\t\tFile path to write prime number");
        return;
    }

    let mut file_path: String = String::new();
    let bits_length: usize;
    let safe: bool;
    let mut output_file: String = String::new();

    if args.contains(&String::from("--file")) || args.contains(&String::from("-f")) {
        let index = args.iter().position(|r| r == "--file" || r == "-f").unwrap();
        file_path = args[index + 1].clone();
    } else {
        util::input_line("Enter file path: ", &mut file_path);
    }
    if args.contains(&String::from("--lenght")) || args.contains(&String::from("-l")) {
        let index = args.iter().position(|r| r == "--lenght" || r == "-l").unwrap();
        bits_length = args[index + 1].parse::<usize>().unwrap();
    } else {
        // print input console for bits length
        bits_length = 256;
    }
    if args.contains(&String::from("--safe")) {
        safe = true;
    } else {
        safe = false;
    }
    if args.contains(&String::from("-o")) || args.contains(&String::from("--output")) {
        let index = args
            .iter()
            .position(|r| r == "--output" || r == "-o")
            .unwrap();
        output_file = args[index + 1].clone();
    }
    println! {"{} {} {}", file_path, bits_length, safe};
    let prime = genprime_control(&file_path, bits_length, safe);

    // if output file is not specified, print to console
    if output_file.is_empty() {
        println!("prime number: {}", prime.to_str_radix(10));
    } else {
        fs::write(output_file, prime.to_str_radix(10)).expect("Unable to write file");
    }
}
