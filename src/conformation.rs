// conformation: Atoms, Groups

pub struct Group {
    group: Vec<Atom>,
}

pub struct Atom {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub charge: f64,
    // elem: String,
    // radius: f64,
    // charge: f64,
}

// 0         1         2         3         4         5         6
// 01234567890123456789012345678901234567890123456789012345678901234567890123456789
// ATOM      1  N   ASP A   1      27.405  -7.086  18.389  1.00  0.00           N
// ATOM      2  CA  ASP A   1      26.221  -7.728  18.989  1.00  0.00           C
//                               ||||||||        ||||||||
//                                       ||||||||
//                               30..38  38..46  46..44

impl Atom {
    pub fn new(args: &str) -> Atom {
        let x: f64 = args[30..38].trim().parse().expect("X not a number");
        let y: f64 = args[38..46].trim().parse().expect("Y not a number");
        let z: f64 = args[46..54].trim().parse().expect("Z not a number");
        let charge: f64 = -0.69;
        //println!("Creating atom with {}, {}, {}, charge {}", x, y, z, charge);
        Atom { x, y, z, charge }
    }

    pub fn dist(&self, other: &Atom) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2)).sqrt()
    }
}
