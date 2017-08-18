// This time, and this one time *only*
// we are going to implement a PDB parser.
// Literally as soon as we have Internet, we
// will look for an existing PDB parser.
// I promise. -- Alex (Sun, Jul 30 at 7:30 PM)

extern crate daggy;

pub mod conformation;
pub mod scoring;
pub mod sampling;

use std::fs::File;
use std::error::Error;
use std::io::prelude::*;
use conformation::{Atom, Pose};
use sampling::{Move, Protocol, get_protocol};
use scoring::score;

// File and protocol input and output
// are in here, because we are shortly
// going to be using a nom parser for this

pub struct Config {
    // a Config objects holds the Rustetta config options
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


// 0         1         2         3         4         5         6
// 01234567890123456789012345678901234567890123456789012345678901234567890123456789
// ATOM      1  N   ASP A   1      27.405  -7.086  18.389  1.00  0.00           N
// ATOM      2  CA  ASP A   1      26.221  -7.728  18.989  1.00  0.00           C
//                               ||||||||        ||||||||
//                                       ||||||||
//                               30..38  38..46  46..44

pub struct Record {
    // PDB atom record class

    pub xyz: XYZ,
    pub charge: f64,
    pub element: String,
    pub residue_index: i32,
    pub residue_name: String,
}

impl Record {
    pub fn new(args: &str) -> Record {

        // coords
        let x: f64 = args[30..38].trim().parse().expect("X not a number");
        let y: f64 = args[38..46].trim().parse().expect("Y not a number");
        let z: f64 = args[46..54].trim().parse().expect("Z not a number");
        let xyz = XYZ::new(x, y, z);

        // residue-level information
        let residue_index: i32 = args[23..30].trim().parse().expect("Residue index not a number");
        let residue_name = args[17..20].to_string(); //String::from(args[17..20]);

        // charge and radius
        let charge: f64 = -0.69; // actually look up
        let element = args[11..17].trim().to_string();
        // let charge = look_up_charge(atom_name); or something
        //println!("Creating atom with {}, {}, {}, charge {}", x, y, z, charge);

        Record { xyz, charge, element, residue_index, residue_name }
    }
}


pub fn parse_pdb(contents: &str) -> Vec<Atom> {

    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains("ATOM") {
            results.push(line);
        }
    }

    let mut records = Vec::new();
    for result in results {
        let pkg = Record::new(&result);

        // logic to decide what to do with this atom
        records.push(pkg);
    }

    let mut atoms = Vec::new();
    for record in records {
        let pkg = Atom::new(record);

        atoms.push(pkg);
    }

    atoms
}

pub fn run(config: Config) -> Result<(), Box<Error>> {

    // read input files from Config object
    let mut f = File::open(config.filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    // create pose object by parsing the PDB
    let atoms = parse_pdb(&contents);
    let pose = Pose::from_atoms(atoms);

    // Apply a protocol to the Pose
    let protocol = get_protocol(&config.protocol);
    let total_score = protocol.apply(pose);
    println!("Total score: {}", total_score);

    Ok(())
}

#[derive(Debug)]
pub struct XYZ {
    x: f64,
    y: f64,
    z: f64,
}


impl XYZ {
    pub fn new(x: f64, y: f64, z: f64) -> XYZ {
        XYZ { x, y, z }
    }
}
