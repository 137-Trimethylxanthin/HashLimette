use std::{env::{self, Args}, process::exit, ops::Not};
use hash_limette::{encrypt, decrypt, base64_encode, base64_decode};


fn help(){
    println!("Usage: ./HashLimette [OPTION]... [STRING]...");
    println!("");
    println!("  -e, --encrypt           encrypt data");
    println!("  -d, --decrypt           decrypt data");
    println!("  -k, --key               key to encrypt or decrypt data | if none give uses default key");
    println!("the key is only for encrypt and decrypt and does not work with base64");
    println!("  -eb, --encode_base64            encode string in base64");
    println!("  -db, --decode_base64            decode string in base64");
    println!("  -v, --version           output version information and exit");
    println!("  -h, --help              display this help and exit");
    println!("");
    println!("Report bugs to my github via the issues page: @137trimethylxanthin");
}

fn main(){
    let args:Vec<String> = env::args().collect();
    println!("{:?}", args);
    match args.len() {
        1 => {
            help();
            exit(0);
        },
        _ => {
            let mut key: Option<&str> = None;
            if args.iter().any(|arg| arg == "-h" || arg == "--help") {
                help();
                exit(0);
            }
            if args.iter().any(|arg| arg == "-v" || arg == "--version") {
                println!("HashLimette 0.1.3");
                exit(0);
            }
            if args.iter().any(|arg| arg == "-k" || arg == "--key"){
                if let Some(index) = args.iter().position(|arg| arg == "-k" || arg == "--key") {
                    if index + 1 < args.len() && args[index+1].starts_with("-").not(){
                        key = Some(&args[index+1]);
                    } else {
                        println!("Error: no key specified after -k/--key");
                        exit(1);
                    }
                } else {
                    println!("Error: no key specified after -k/--key");
                    exit(1);
                }
            }
            for (i, arg) in args.iter().enumerate() {
                match arg.as_str(){
                    "-e" | "-encrypt" => {
                        if args.len() > i+1 {
                            if args[i+1].starts_with("-").not(){
                                println!("{}", encrypt(&args[i+1], key));
                                exit(0);
                            } else {
                                println!("Error: no string specified after -e/--encrypt");
                                exit(1);
                            }
                        } else {
                            println!("Error: no string specified after -e/--encrypt");
                            exit(1);
                        }
                    },
                    "-d" | "-decrypt" => {
                        if args.len() > i+1 {
                            if args[i+1].starts_with("-").not(){
                                println!("{}", decrypt(&args[i+1], key));
                                exit(0);
                            } else {
                                println!("Error: no string specified after -d/--decrypt");
                                exit(1);
                            }
                        } else {
                            println!("Error: no string specified after -d/--decrypt");
                            exit(1);
                        }
                    },
                    "-eb" | "-encode_base64" => {
                        if args.len() > i+1 {
                            if args[i+1].starts_with("-").not(){
                                println!("{}", base64_encode(&args[i+1]));
                                exit(0);
                            } else {
                                println!("Error: no string specified after -eb/--encode_base64");
                                exit(1);
                            }
                        } else {
                            println!("Error: no string specified after -eb/--encode_base64");
                            exit(1);
                        }
                    },
                    "-db" | "-decode_base64" => {
                        if args.len() > i+1 {
                            if args[i+1].starts_with("-").not(){
                                println!("{}", base64_decode(&args[i+1]));
                                exit(0);
                            } else {
                                println!("Error: no string specified after -db/--decode_base64");
                                exit(1);
                            }
                        } else {
                            println!("Error: no string specified after -db/--decode_base64");
                            exit(1);
                        }
                    },
                    _ => {
                        if i == args.len()-1 {
                            println!("Error: invalid option(s) '{}'", arg);
                            println!("Try './HashLimette --help' for more information.");
                            exit(1);
                        }
                    }
                }
            }
        }
    }   
}
