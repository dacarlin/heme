// File input and output
// WARNING: contains hand-rolled PDB parser

use conformation::{XYZ, Atom, Pose};
use sampling::get_protocol;

use std::fs::File;
use std::error::Error;
use std::io::prelude::*;

pub struct Config {
    // a Config objects holds the  config options
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

// PDB parsing

// This time, and this one time *only*
// we are going to implement a PDB parser.
// Literally as soon as we have Internet, we
// will look for an existing PDB parser.
// I promise. -- Alex (Jul 30, 2017 at 7:30 PM)

// 0         1         2         3         4         5         6
// 01234567890123456789012345678901234567890123456789012345678901234567890123456789
// ATOM      1  N   ASP A   1      27.405  -7.086  18.389  1.00  0.00           N
// ATOM      2  CA  ASP A   1      26.221  -7.728  18.989  1.00  0.00           C
//                               ||||||||        ||||||||
//                                       ||||||||
//                               30..38  38..46  46..44

pub struct Record {
    // PDB atom record class
    // and this is the only struct
    // we're gonna make

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
    // Parse a PDB file

    let mut atoms = Vec::new();
    // let mut residues = Vec::new();
    for line in contents.lines() {
        if line.starts_with("ATOM") {
            let record = Record::new(&line);
            let atom = Atom::new(&record);
            atoms.push(atom);
        }
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
    let mut pose = Pose::from_atoms(atoms);

    // Apply a protocol to the Pose
    let protocol = get_protocol(&config.protocol);
    protocol.run(&mut pose);

    // println!("Total score: {}", total_score);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_atoms_testy() {
        let record = Record::new("ATOM      8  OD2 ASP A   1      28.042 -10.262  20.858  1.00  0.00           O ");
        let atom = Atom::new(&record);
        assert_eq!(atom.xyz.x, 28.042);
    }

    #[test]
    fn test_pdb_parsing() {
        let contents = "ATOM      1  N   ASP A   1      27.405  -7.086  18.389  1.00  0.00           N
ATOM      2  CA  ASP A   1      26.221  -7.728  18.989  1.00  0.00           C
ATOM      3  C   ASP A   1      25.150  -6.679  19.328  1.00  0.00           C
ATOM      4  O   ASP A   1      24.099  -6.710  18.722  1.00  0.00           O
ATOM      5  CB  ASP A   1      26.607  -8.504  20.250  1.00  0.00           C
ATOM      6  CG  ASP A   1      27.438  -9.745  19.951  1.00  0.00           C
ATOM      7  OD1 ASP A   1      27.457 -10.165  18.817  1.00  0.00           O
ATOM      8  OD2 ASP A   1      28.042 -10.262  20.858  1.00  0.00           O
ATOM      9 1H   ASP A   1      28.091  -7.781  18.175  1.00  0.00           H
ATOM     10 2H   ASP A   1      27.138  -6.611  17.550  1.00  0.00           H
";
        let atoms = parse_pdb(&contents);
        let pose = Pose::from_atoms(atoms);

        assert_eq!(pose.atoms.len(), 10);

    }

    #[test]
    fn test_entropy_function() {

        // entropy function
        fn H(l: &f64) -> f64 {

            // bounds of N choose lN
            // can be found by 2^(n*H(l))
            // where H is this function

            // 0 < l < 0

            let value = 1.0 - l;
            l.ln() - value * value.ln()
        }

        let test_float: f64 = 18.11;
        let answer: f64 = H(&test_float);
        println!(">>> {}", answer);

        // assert_eq!(4, 4);
    }

    #[test]
    fn test_lennard_jones_atom_loading() {


    }

}
