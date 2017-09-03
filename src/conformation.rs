// Conformation (smaller to larger, Atom > Residue > Pose)

use io::Record;

#[derive(Debug)]
pub struct XYZ {
    pub x: f64,
    pub y: f64,
    pub z: f64,

    // add the Add trait!
}


impl XYZ {
    pub fn new(x: f64, y: f64, z: f64) -> XYZ {
        XYZ { x, y, z }
    }

    pub fn clone(&self) -> XYZ {
        let x = self.x.clone();
        let y = self.y.clone();
        let z = self.z.clone();

        XYZ { x, y, z }
    }
}

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
    pub charge: f64,
    pub xyz: XYZ,
    pub element: String,
}

impl Atom {
    pub fn new(record: &Record) -> Atom {
        let xyz = record.xyz.clone();
        let charge = record.charge.clone();
        let element = record.element.clone();
        // we will probably do some more construction at some point

        Atom { xyz, charge, element }
    }

    pub fn dist(&self, other: &Atom) -> f64 {
        ((self.xyz.x - other.xyz.x).powi(2) +
         (self.xyz.y - other.xyz.y).powi(2) +
         (self.xyz.z - other.xyz.z).powi(2)).sqrt()
    }

    /*

    pub fn polarize(&self, other: &mut Atom, magnitude: f64) {
        // let total_charge: f64 = self.charge + other.charge;
        self.charge += magnitude;
        other.charge -= magnitude;
    }

    */

}
