// Conformation (smaller to larger, Atom > Residue > Pose)

use Record;

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

    pub fn translate(&self, other: &XYZ) {
        //
    }

    pub fn rotate(&self, other: &XYZ) {
        //
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
    pub fn new(r: Record) -> Atom {
        Atom { xyz: r.xyz, charge: r.charge, element: r.element }
    }

    pub fn dist(&self, other: &Atom) -> f64 {
        ((self.xyz.x - other.xyz.x).powi(2) +
         (self.xyz.y - other.xyz.y).powi(2) +
         (self.xyz.z - other.xyz.z).powi(2)).sqrt()
    }
}
