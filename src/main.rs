use std::env;
use std::process;

use heme::io::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Running protocol: {}", config.protocol);
    println!("Input file name: {}", config.filename);

    if let Err(e) = heme::io::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
