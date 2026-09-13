use serde::Deserialize;
use std::fs;

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
    let prototypes_ron =
        fs::read_to_string("prototypes.ron").expect("Failed to read the `prototypes.ron` file.");
    let prototypes = ron::from_str::<Vec<Prototype>>(&prototypes_ron)
        .expect("The `prototypes.ron` file has an invalid format.");

    println!("{:#?}", prototypes);
}
