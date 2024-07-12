// Conformation (smaller to larger, Atom > Residue > Pose)

use crate::io::Record;

#[derive(Debug)]
pub struct XYZ {
    pub x: f64,
    pub y: f64,
    pub z: f64,

    // add the Add trait!
}

/// Holds Cartesian coordinates
///
/// # Example
///
/// ```
/// //use conformation::XYZ; 
///
/// //let xyz = XYZ::new(0.123, 0.234, 0.345); 
///
/// //assert_eq!(0.123, xyz.x); 
/// ```
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

/// Holds a protein 
pub struct Pose {
    pub atoms: Vec<Atom>,
}

impl Pose {
    pub fn from_atoms(atoms: Vec<Atom>) -> Pose {

        Pose { atoms }
    }
}

#[derive(Debug)]
pub struct Residue {
    atoms: Vec<Atom>,
    residue_name: String,
    residue_index: i32,
}

#[derive(Debug)]
pub struct Atom {
    pub charge: f64,
    pub xyz: XYZ,
    pub element: String,
    pub residue_index: i32, 
    pub residue_name: String, 
}

impl Atom {
    pub fn new(record: &Record) -> Atom {
        let xyz = record.xyz.clone();
        let charge = record.charge.clone();
        let element = record.element.clone();
        let residue_index = record.residue_index.clone(); 
        let residue_name = record.residue_name.clone(); 
        // we will probably do some more construction at some point

        Atom { xyz, charge, element, residue_index, residue_name }
    }

    pub fn dist(&self, other: &Atom) -> f64 {
        ((self.xyz.x - other.xyz.x).powi(2) +
         (self.xyz.y - other.xyz.y).powi(2) +
         (self.xyz.z - other.xyz.z).powi(2)).sqrt()
    }

}
