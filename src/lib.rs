// This time, and this one time *only*
// we are going to implement a PDB parser.
// Literally as soon as we have Internet, we
// will look for an existing PDB parser.
// I promise. -- Alex (Sun, Jul 30 at 7:30 PM)

pub mod conformation;
pub mod scoring;
pub mod sampling;

use std::fs::File;
use std::error::Error;
use std::io::prelude::*;
use conformation::{Atom, Group, Pose};
use sampling::{Move, Protocol, get_protocol};
use scoring::score;

// File and protocol input and output
// are in here, because we are shortly
// going to be using a nom parser for this

pub struct Config {
    // a Config objects holds the name of the protocol we want to run
    // and the filename of the input PDB
    pub protocol: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let protocol = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { protocol, filename })
    }
}

pub fn parse_pdb(protocol: &str, contents: &str) -> Vec<Atom> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains("ATOM") {
            results.push(line);
        }
    }
    let mut atoms = Vec::new();
    for result in results {
        let pkg = Atom::new(&result);
        atoms.push(pkg);
    }
    atoms
}

pub fn run(config: Config) -> Result<(), Box<Error>> {

    // Construct a Pose object from the raw text
    let mut f = File::open(config.filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    let pose: Vec<Atom> = parse_pdb("ATOM", &contents);

    // Apply a protocol to the Pose
    let protocol = get_protocol();
    let result = protocol.apply(pose);

    // Print out the result
    println!("Total score: {}", result);

    Ok(())
}

// Tests

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let protocol = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(protocol, contents)
        );
    }
}
