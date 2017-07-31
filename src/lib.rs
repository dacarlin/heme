// This time, and this one time *only*
// we are going to implement a PDB parser.
// Literally as soon as we have Internet, we
// will look for an existing PDB parser.
// I promise. -- Alex (Sun, Jul 30 at 7:30 PM)

pub mod conformation;
pub mod scoring;

use std::fs::File;
use std::error::Error;
use std::io::prelude::*;
use conformation::Atom;
use scoring::score;

// File and protocol input and output
// are in here, because we are shortly
// going to be using a library for this

pub struct Config {
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
    let mut f = File::open(config.filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let pose: Vec<Atom> = parse_pdb("ATOM", &contents);
    let protocol = get_protocol();
    let result = protocol.apply(pose);

    println!("Total score: {}", result);

    Ok(())
}

// Conformation (smaller to larger, Atom > Group > Pose)

pub struct Pose {
    pub atoms: Vec<Atom>,
}

// Sampling (smaller to larger, Move > Protocol)

pub struct Move {
    pub name: String,
}

impl Move {
    pub fn new() -> Move {

        Move { name: String::from("This is a move") }
    }
    pub fn apply(&self, pose: Vec<Atom>) {
        println!("Applied move!")
    }
}

pub struct Protocol {
    pub protocol: f64,
    pub steps: Vec<Move>,
}

impl Protocol {
    pub fn apply(&self, pose: Vec<Atom>) -> f64 {
        // Right now, we hard code a protocol of score-only
        // into this class, but eventually we will customize
        // the protocols at run time
        let total_score = score(pose);

        total_score
    }

}

pub fn get_protocol() -> Protocol {
    let protocol: f64 = 1211.321;
    let steps = vec![Move::new(), Move::new()];
    Protocol { protocol, steps }
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
