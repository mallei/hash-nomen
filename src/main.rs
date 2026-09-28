use comfy_table::{CellAlignment, Table};
use regex::Regex;
use serde::Deserialize;
use std::{env, fs, process};

#[derive(Debug, Deserialize)]
struct HashInfo {
    name: String,
    hashcat: Option<u32>,
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

    println!("\n\x1b[4mAnalyzing `{}`\x1b[0m\n", hash);

    let mut table = Table::new();
    table.set_header(vec!["Hash Name", "Hashcat Mode"]);
    table
        .column_mut(1)
        .unwrap()
        .set_cell_alignment(CellAlignment::Right);

    for prototype in prototypes.iter() {
        let Ok(re) = Regex::new(&prototype.regex) else {
            continue;
        };

        if re.is_match(hash) {
            for hash_info in prototype.modes.iter() {
                table.add_row(vec![
                    &hash_info.name,
                    &hash_info
                        .hashcat
                        .map(|m| m.to_string())
                        .unwrap_or_else(|| "None".to_string()),
                ]);
            }

            println!("{}", table);
            process::exit(0);
        }
    }

    println!("Invalid hash");
    process::exit(0);
}
