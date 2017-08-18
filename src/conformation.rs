// Conformation (smaller to larger, Atom > Residue > Pose)

use XYZ;
use Record;

pub struct Pose {
    // container to hold conformation, energy, and methods for
    // operating on a macromolecular structure

    pub atoms: Vec<Atom>,
}

impl Pose {
    pub fn from_atoms(atoms: Vec<Atom>) -> Pose {

        Pose { atoms }
    }
}

pub struct Residue {
    atoms: Vec<Atom>,
    residue_name: String,
    residue_index: i32,
}

pub struct Atom {
    // pub residue_name: String,
    // pub residue_index: i32,
    // pub x: f64,
    // pub y: f64,
    // pub z: f64,
    pub charge: f64,
    pub xyz: XYZ,
    pub element: String,
    // elem: String,
    // radius: f64,
}


impl Atom {
    pub fn new(record: Record) -> Atom {

        Atom { xyz: record.xyz, charge: record.charge, element: record.element }
    }

    pub fn dist(&self, other: &Atom) -> f64 {
        ((self.xyz.x - other.xyz.x).powi(2) +
         (self.xyz.y - other.xyz.y).powi(2) +
         (self.xyz.z - other.xyz.z).powi(2)).sqrt()
    }
}
