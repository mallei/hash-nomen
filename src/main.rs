use regex::Regex;
use serde::Deserialize;
use std::{env, fs, process};

#[derive(Debug, Deserialize)]
struct HashInfo {
    name: String,
}

#[derive(Debug, Deserialize)]
struct Prototype {
    regex: String,
    modes: Vec<HashInfo>,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} <HASH>", &args[0]);
        process::exit(1);
    }

    let hash = &args[1];

    let Ok(prototypes_ron) = fs::read_to_string("prototypes.ron") else {
        eprintln!("Failed to read the `prototypes.ron` file.");
        process::exit(1);
    };

    let Ok(prototypes) = ron::from_str::<Vec<Prototype>>(&prototypes_ron) else {
        eprintln!("The `prototypes.ron` file has an invalid format.");
        process::exit(1);
    };

    for prototype in prototypes.iter() {
        let Ok(re) = Regex::new(&prototype.regex) else {
            continue;
        };

        if re.is_match(hash) {
            for hash_info in prototype.modes.iter() {
                println!("{}", hash_info.name);
            }

            process::exit(0);
        }
    }
}
