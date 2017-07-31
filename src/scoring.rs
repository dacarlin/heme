/*

Kinds of energies for atoms:

- two body
    - angle
    - dist
    - torsion
    - electrostatic
    - Lennard-Jones

Kinds of energies for residues:

- one body
    - depends only on amino acid
    - depends only on phi and psi angle
- two body

*/

use conformation::Atom;

pub fn score(pose: Vec<Atom>) -> f64 {
    // Right now, we are only scoring atoms
    //let mut total_energy: f64 = 0.0;

    // electrostatics
    let mut coulomb_score: f64 = 0.0;
    let mut lennard_jones: f64 = 0.0;

    for i in &pose {
        for j in &pose {
            let r: f64 = i.dist(j);
            if r == 0.0 {
                // these are the same atom
                // do nothing
            } else if r.max(12.0) == 12.0 {
                // these atoms are greater than 12 Å apart
                // ignore that they interact
            } else {
                let epsilon: f64 = 1.0;
                let C: f64 = 1.0;
                let cou = i.charge * j.charge / r.powi(2);
                coulomb_score += C * cou;

                let rm: f64 = 1.0; // minimum distance is actually going to depend on both atoms' radii
                let lj = (rm/r).powi(12) - 2.0 * (rm/r).powi(6);
                lennard_jones += epsilon * lj;
            }
        }
    }

    coulomb_score + lennard_jones
}
