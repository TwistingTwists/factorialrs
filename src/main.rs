use clap::{Parser, Subcommand};
use num::bigint::BigUint;

use num::bigint::ToBigUint;
use num::One;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};

#[derive(Serialize, Deserialize, Debug)]
struct FactorialCache {
    data: std::collections::HashMap<usize, BigUint>,
}

fn calculate_factorial(n: usize) -> BigUint {
    let mut result = BigUint::one();
    for i in 1..=n {
        result *= ToBigUint::to_biguint(&i).unwrap();
    }
    result
}

fn load_cache() -> Option<FactorialCache> {
    let file = File::open("factorial_cache.json");
    if let Ok(mut file) = file {
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        serde_json::from_str(&contents).ok()
    } else {
        None
    }
}

fn save_cache(cache: &FactorialCache) {
    let json = serde_json::to_string_pretty(cache).unwrap();
    let mut file = File::create("factorial_cache.json").unwrap();
    file.write_all(json.as_bytes()).unwrap();
}

fn factorial(n: usize) -> BigUint {
    if let Some(cache) = load_cache() {
        if let Some(result) = cache.data.get(&n) {
            return result.clone();
        }
    }

    let result = calculate_factorial(n);

    let mut cache = load_cache().unwrap_or_else(|| FactorialCache {
        data: std::collections::HashMap::new(),
    });
    cache.data.insert(n, result.clone());
    save_cache(&cache);

    result
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    #[clap(about = "Calculate factorial")]
    Calculate {
        #[clap(short, long, value_parser)]
        number: usize,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Calculate { number } => {
            let result = factorial(number);
            println!("Factorial of {} is: {}", number, result);
        }
    }
}