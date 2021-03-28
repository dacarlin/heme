// Conformation (smaller to larger, Atom > Residue > Pose)

use std::fmt; 
use io::Record;

pub struct XYZ {
    pub x: f64,
    pub y: f64,
    pub z: f64,
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

impl fmt::Display for XYZ {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f, "{} {} {}", self.x, self.y, self.z)
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

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f, "xyz={} elem={}", self.xyz, self.element)
    }
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
}
